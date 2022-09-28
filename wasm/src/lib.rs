mod utils;
use std::{collections::HashMap, sync::Mutex};

use game_loop::game_loop;
use serde::{Deserialize, Serialize};
use utils::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;

#[derive(Serialize, Deserialize, Clone)]
enum AtomShape {
    Dot,
    Square,
    Triangle,
}

#[derive(Serialize, Deserialize, Clone)]
struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    pub fn random(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        Vector {
            x: random_range(min_x, max_x),
            y: random_range(min_y, max_y),
        }
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn distance(&self, other: &Vector) -> (f64, Vector) {
        let d = Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        };
        (d.length(), d)
    }
    pub fn plus(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
    }
    pub fn plus_amp(&mut self, other: &Vector, amp: f64) {
        self.x += other.x * amp;
        self.y += other.y * amp;
    }
    pub fn add_force(&mut self, force: Vector) {
        self.x = (self.x + force.x) * 0.5;
        self.y = (self.y + force.y) * 0.5;
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Atom {
    p: Vector,
    v: Vector,
}

impl Atom {
    pub fn born(width: f64, height: f64, padding: f64) -> Self {
        Atom {
            p: Vector::random(padding, width - padding, padding, height - padding),
            v: Vector { x: 0f64, y: 0f64 },
        }
    }
    pub fn reborn(&mut self, width: f64, height: f64, padding: f64) {
        self.p = Vector::random(padding, width - padding, padding, height - padding);
        self.v = Vector { x: 0f64, y: 0f64 };
    }
    pub fn correct_v(&mut self, max_x: f64, max_y: f64) {
        if self.p.x <= 0f64 || self.p.x >= max_x {
            self.v.x *= -1f64
        }
        if self.p.y <= 0f64 || self.p.y >= max_y {
            self.v.y *= -1f64
        }
    }
    pub fn compute(&mut self, others: &Vec<Atom>, w: f64, radius: f64) {
        let mut force = Vector { x: 0f64, y: 0f64 };
        for atom in others {
            let (d, v_force) = self.p.distance(&atom.p);
            if d > 0f64 && d < radius {
                let f = w / d;
                force.plus_amp(&v_force, f)
            }
        }
        self.v.add_force(force);
        self.p.plus(&self.v);
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct AtomSet {
    /// Radius effect of force around points
    radius: f64,
    color: String,
    point_size: f64,
    atoms: Vec<Atom>,
    blur_size: Option<f64>,
    shape: Option<AtomShape>,
}

impl AtomSet {
    pub fn apply_rule(&mut self, other: &AtomSet, w: f64) {
        let radius = self.radius;
        self.atoms
            .iter_mut()
            .for_each(|atom_src| atom_src.compute(&other.atoms, w, radius));
    }
    pub fn correct_pos(&mut self, max_x: f64, max_y: f64) {
        self.atoms
            .iter_mut()
            .for_each(|atom_src| atom_src.correct_v(max_x, max_y));
    }
    pub fn reborn(&mut self, max_x: f64, max_y: f64, padding: f64) {
        self.atoms
            .iter_mut()
            .for_each(|atom_src| atom_src.reborn(max_x, max_y, padding));
    }
    pub fn draw(self, ctx: &CanvasRenderingContext2d) {
        for point in self.atoms {
            ctx.begin_path();
            let mut lower_color = self.color.clone();
            let outline_size: f64 = self.point_size + 4f64;
            lower_color.push_str("99");
            ctx.set_fill_style(&JsValue::from(lower_color));
            ctx.fill_rect(
                point.p.x - 2f64,
                point.p.y - 2f64,
                outline_size,
                outline_size,
            );
            ctx.set_fill_style(&JsValue::from(self.color.clone()));
            ctx.fill_rect(point.p.x, point.p.y, self.point_size, self.point_size);
            if let Some(blur_radius) = self.blur_size {
                ctx.set_shadow_color(&self.color);
                ctx.set_shadow_blur(blur_radius);
                ctx.set_shadow_offset_y(0f64);
                ctx.set_shadow_offset_x(0f64);
            }
            ctx.close_path()
        }
    }
}

struct AtomRule {
    src: String,
    target: String,
    w: f64,
}

impl AtomRule {
    pub fn vec_into(input: Vec<String>) -> Self {
        AtomRule {
            src: input[0].clone(),
            target: input[1].clone(),
            w: str_to_f64(&input[2]),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct AtomConf {
    total: i32,
    size: f64,
    blur: Option<f64>,
    radius: Option<f64>,
    shape: Option<AtomShape>,
}

struct System {
    engine_loaded: bool,
    compute_tick: u32,
    born_padding: f64,
    frame_count: i64,
    rendering: bool,
    rules: Vec<AtomRule>,
    atoms: Option<HashMap<String, AtomSet>>,
    ctx: Option<CanvasRenderingContext2d>,
}

impl System {
    pub fn load_js_rules(&mut self, raw: &JsValue) {
        let raw: Vec<Vec<String>> = raw.into_serde().unwrap();
        self.rules = raw.into_iter().map(AtomRule::vec_into).collect();
    }
    pub fn load_js_conf(&mut self, raw: &JsValue) {
        let canvas = &self.ctx.as_ref().unwrap().canvas().unwrap();
        let cv_w = canvas.width() as f64;
        let cv_h = canvas.height() as f64;
        let conf: HashMap<String, AtomConf> = raw.into_serde().unwrap();
        let atoms = self.atoms.as_mut().unwrap();
        for (name, conf) in &conf {
            let color_atoms = atoms.get_mut(name);
            if let Some(color_atoms) = color_atoms {
                if color_atoms.atoms.len() as i32 > conf.total {
                    unsafe {
                        color_atoms.atoms.set_len(conf.total as usize);
                    }
                } else {
                    for _ in 0..(conf.total - color_atoms.atoms.len() as i32) {
                        color_atoms
                            .atoms
                            .push(Atom::born(cv_w, cv_h, self.born_padding))
                    }
                }
                color_atoms.blur_size = conf.blur;
                color_atoms.point_size = conf.size;
                color_atoms.shape = conf.shape.clone();
                // Already have this color in conf -> ignore create new
                continue;
            }
            // Insert new color set
            let mut atom_set: Vec<Atom> = vec![];
            for _ in 0..conf.total {
                atom_set.push(Atom::born(cv_w, cv_h, self.born_padding))
            }
            atoms.insert(
                name.to_string(),
                AtomSet {
                    atoms: atom_set,
                    color: name.to_string(),
                    point_size: conf.size,
                    blur_size: conf.blur,
                    shape: conf.shape.clone(),
                    radius: conf.radius.unwrap_or(80f64),
                },
            );
        }
        // Check some color is removed
        let mut removeter: Vec<String> = vec![];
        atoms.iter_mut().for_each(|(name, _)| {
            if conf.get(name).is_none() {
                removeter.push(name.to_string());
            }
        });
        for removed in &removeter {
            atoms.remove(removed);
        }
    }
    pub fn refresh(&mut self) {
        let canvas = &self.ctx.as_ref().unwrap().canvas().unwrap();
        let cv_w = canvas.width() as f64;
        let cv_h = canvas.height() as f64;
        let padding = self.born_padding;
        let atoms = self.atoms.as_mut().unwrap();
        atoms
            .iter_mut()
            .for_each(move |(_, atom_set)| atom_set.reborn(cv_w, cv_h, padding));
    }
    pub fn compute_next_tick(&mut self) {
        let rules = &self.rules;
        for rule in rules {
            if let Some(atoms) = &mut self.atoms {
                let atom_b = atoms.get(&rule.target).unwrap().clone();
                atoms
                    .get_mut(&rule.src)
                    .unwrap()
                    .apply_rule(&atom_b, rule.w)
            }
        }
    }
    pub fn correct_point_position(&mut self) {
        if self.rendering && self.engine_loaded {
            let canvas = self.ctx.as_ref().unwrap().canvas().unwrap();
            let cv_w = canvas.width() as f64;
            let cv_h = canvas.height() as f64;
            self.atoms
                .as_mut()
                .unwrap()
                .iter_mut()
                .for_each(move |(_, color)| color.correct_pos(cv_w, cv_h))
        }
    }
    pub fn render(&mut self) {
        if self.rendering && self.engine_loaded {
            let ctx = &self.ctx.as_ref().unwrap();
            ctx.clear_rect(
                0f64,
                0f64,
                ctx.canvas().unwrap().width().into(),
                ctx.canvas().unwrap().height().into(),
            );
            self.frame_count += 1;
            self.atoms
                .as_ref()
                .unwrap()
                .iter()
                .for_each(move |(_, color)| color.to_owned().draw(ctx))
        }
    }
}

static mut SYSTEM: Mutex<System> = Mutex::new(System {
    engine_loaded: false,
    compute_tick: 64,
    born_padding: 10f64,
    frame_count: 0,
    rendering: false,
    rules: vec![],
    atoms: None,
    ctx: None,
});

#[wasm_bindgen]
pub fn init_engine(canvas_id: String) {
    unsafe {
        let crr_state = SYSTEM.get_mut().unwrap();
        if !crr_state.engine_loaded {
            let document = web_sys::window().unwrap().document().unwrap();
            let canvas = document.get_element_by_id(&canvas_id).unwrap();
            let canvas: web_sys::HtmlCanvasElement = canvas
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| ())
                .unwrap();

            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

            crr_state.ctx = Some(context);
            crr_state.engine_loaded = true;
            crr_state.atoms = Some(HashMap::new());
            game_loop(
                crr_state,
                120,
                0.2,
                move |g| {
                    g.set_updates_per_second(g.game.compute_tick);
                    g.game.compute_next_tick();
                    g.game.correct_point_position();
                },
                |g| {
                    g.game.render();
                },
            );
        }
    }
}

#[wasm_bindgen]
pub fn reload() {
    unsafe {
        let crr_state = SYSTEM.get_mut().unwrap();
        crr_state.refresh();
    }
}

#[wasm_bindgen]
pub fn start_render() {
    unsafe {
        let crr_state = SYSTEM.get_mut().unwrap();
        crr_state.rendering = true;
    }
}

#[wasm_bindgen]
pub fn stop_render() {
    unsafe {
        let crr_state = SYSTEM.get_mut().unwrap();
        crr_state.rendering = false;
    }
}

#[wasm_bindgen]
pub fn set_tick(tick: u32) {
    unsafe {
        let crr_state = SYSTEM.get_mut().unwrap();
        crr_state.compute_tick = tick;
    }
}

#[wasm_bindgen]
pub fn update_rules(rules: &JsValue) {
    unsafe {
        let crr_state = SYSTEM.get_mut().unwrap();
        crr_state.load_js_rules(rules)
    }
}

#[wasm_bindgen]
pub fn update_conf(conf: &JsValue) {
    unsafe {
        let crr_state = SYSTEM.get_mut().unwrap();
        crr_state.load_js_conf(conf)
    }
}

#[wasm_bindgen]
pub fn get_crr_frame_idx() -> i64 {
    unsafe {
        let crr_state = SYSTEM.lock().unwrap();
        crr_state.frame_count
    }
}
