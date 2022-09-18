mod utils;

extern crate serde_json;
extern crate wasm_bindgen;

use game_loop::game_loop;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use std::time::SystemTime;
use std::{collections::HashMap, sync::Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PartialOrd)]
enum AtomShape {
    Dot,
    Square,
    Triangle,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct AtomRule {
    src: String,
    target: String,
    weight: f64,
}

impl AtomRule {
    pub fn convert(input: Vec<String>) -> AtomRule {
        AtomRule {
            src: input[0].clone(),
            target: input[1].clone(),
            weight: str::parse::<f64>(&input[2]).unwrap(),
        }
    }
    pub fn apply(self, src: &mut Vec<Atom>, target: Vec<Atom>, speed: f64, cv_w: f64, cv_h: f64) {
        for atom_src in src {
            let mut fx = 0f64;
            let mut fy = 0f64;

            for atom_target in &target {
                let dx = atom_src.x - atom_target.x;
                let dy = atom_src.y - atom_target.y;
                if dx.abs() > 80f64 || dy.abs() > 80f64 {
                    continue;
                }
                let d = (dx * dx + dy * dy).sqrt();
                if d > 0f64 && d < 80f64 * speed {
                    let f = self.weight / d;
                    fx += f * dx;
                    fy += f * dy;
                }
            }

            atom_src.vx = (atom_src.vx + fx) * 0.5 * speed;
            atom_src.vy = (atom_src.vy + fy) * 0.5 * speed;
            atom_src.x += atom_src.vx;
            atom_src.y += atom_src.vy;

            if atom_src.x <= 0f64 || atom_src.x >= cv_w {
                if atom_src.x <= 0f64 {
                    atom_src.x = 0f64 + atom_src.size
                } else {
                    atom_src.x = cv_w - atom_src.size
                }
                atom_src.vx *= -1f64;
            }
            if atom_src.y <= 0f64 || atom_src.y >= cv_h {
                if atom_src.y <= 0f64 {
                    atom_src.y = 0f64 + atom_src.size
                } else {
                    atom_src.y = cv_h - atom_src.size
                }
                atom_src.vy *= -1f64;
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ColorConfiguration {
    total: i32,
    size: f64,
    shape: Option<AtomShape>,
    blur: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Configuration {
    colors: Option<HashMap<String, ColorConfiguration>>,
    rules: Vec<AtomRule>,
    speed: Option<f64>,
    padding: Option<f64>,
}

impl Configuration {
    pub fn raw_into_rules(raw: &JsValue) -> Vec<AtomRule> {
        let raw: Vec<Vec<String>> = raw.into_serde().unwrap();
        raw.into_iter().map(AtomRule::convert).collect()
    }
    pub fn raw_into_colors(raw: &JsValue) -> HashMap<String, ColorConfiguration> {
        raw.into_serde().unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PartialOrd)]
struct Atom {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    size: f64,
    color: String,
    shape: Option<AtomShape>,
    blur: Option<f64>,
}

impl Atom {
    fn random_point(range: f64, padding: f64) -> f64 {
        let seed: f64 = rand::thread_rng().gen();
        seed * range + padding * (1f64 - 2f64 * seed)
    }

    // FIXME: This hurt performance so much, need to know how ?
    fn next_state_by_rule(
        &mut self,
        target: Vec<Atom>,
        weight: f64,
        speed: f64,
        max_x: f64,
        max_y: f64,
    ) {
        let mut fx = 0f64;
        let mut fy = 0f64;

        for atom_target in &target {
            let dx = self.x - atom_target.x;
            let dy = self.y - atom_target.y;
            if dx.abs() > 80f64 || dy.abs() > 80f64 {
                continue;
            }
            let d = (dx * dx + dy * dy).sqrt();
            if d > 0f64 && d < 80f64 * speed {
                let f = weight / d;
                fx += f * dx;
                fy += f * dy;
            }
        }

        self.vx = (self.vx + fx) * 0.5 * speed;
        self.vy = (self.vy + fy) * 0.5 * speed;
        self.x += self.vx;
        self.y += self.vy;

        if self.x <= 0f64 || self.x >= max_x {
            if self.x <= 0f64 {
                self.x = 0f64 + self.size
            } else {
                self.x = max_x - self.size
            }
            self.vx *= -1f64;
        }
        if self.y <= 0f64 || self.y >= max_y {
            if self.y <= 0f64 {
                self.y = 0f64 + self.size
            } else {
                self.y = max_y - self.size
            }
            self.vy *= -1f64;
        }
    }

    pub fn born(
        max_w: f64,
        max_h: f64,
        padding: f64,
        size: f64,
        color: String,
        shape: Option<AtomShape>,
        blur: Option<f64>,
    ) -> Atom {
        Atom {
            x: Self::random_point(max_w, padding),
            y: Self::random_point(max_h, padding),
            vx: 0f64,
            vy: 0f64,
            size,
            color,
            shape,
            blur,
        }
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        let draw_square_default = || {
            // Draw outline
            let mut lower_color = self.color.clone();
            let outline_size: f64 = self.size + 4f64;
            lower_color.push_str("99");
            ctx.set_fill_style(&JsValue::from(lower_color));
            ctx.fill_rect(self.x - 2f64, self.y - 2f64, outline_size, outline_size);

            ctx.set_fill_style(&JsValue::from(self.color.clone()));
            ctx.fill_rect(self.x, self.y, self.size, self.size);
        };
        match self.shape {
            Some(AtomShape::Dot) => {
                ctx.arc(self.x, self.y, self.size / 2f64, 0f64, 2f64 * PI)
                    .unwrap();
                ctx.set_fill_style(&JsValue::from(self.color.clone()));
                ctx.fill();

                // Draw outline
                let mut lower_color = self.color.clone();
                let outline_size: f64 = (self.size / 2f64) + 2f64;
                lower_color.push_str("99");

                ctx.arc(self.x, self.y, outline_size, 0f64, 2f64 * PI)
                    .unwrap();
                ctx.set_fill_style(&JsValue::from(lower_color));
                ctx.fill();
                ctx.set_stroke_style(&JsValue::from(self.color.clone()));
                ctx.stroke();
            }
            Some(_) => draw_square_default(),
            None => draw_square_default(),
        }
        if let Some(blur_radius) = self.blur {
            ctx.set_shadow_color(&self.color);
            ctx.set_shadow_blur(blur_radius);
            ctx.set_shadow_offset_y(0f64);
            ctx.set_shadow_offset_x(0f64);
        }
        ctx.close_path()
    }
}

struct RenderEngine {
    tick: u32,
    rendering: bool,
    atoms: Option<HashMap<String, Vec<Atom>>>,
    ctx: Option<CanvasRenderingContext2d>,
    frames: i64,
}

impl RenderEngine {
    pub fn render(&self) {
        if let Some(ctx) = &self.ctx {
            // Clear canvas to render new frame
            ctx.clear_rect(
                0f64,
                0f64,
                ctx.canvas().unwrap().width().into(),
                ctx.canvas().unwrap().height().into(),
            );
            if let Some(atoms) = &self.atoms {
                atoms.iter().for_each(|(_, color_atoms)| {
                    for atom in color_atoms {
                        atom.draw(ctx)
                    }
                });
            }
        }
    }
    pub fn set_rendering(&mut self, status: bool) -> &mut Self {
        self.rendering = status;
        self
    }

    pub fn render_piline(&mut self) {
        if !self.rendering {
            return;
        }
        self.frames += 1;
        self.render();
    }
}

struct LifeManager {
    render_engine: RenderEngine,
    configuration: Configuration,
}

impl LifeManager {
    fn update_engine_state(&mut self) {
        if !self.render_engine.rendering {
            return;
        }
        let engine = &mut self.render_engine;
        let canvas = engine.ctx.as_ref().unwrap().canvas().unwrap();
        let cv_w = canvas.width() as f64;
        let cv_h = canvas.height() as f64;
        let configuration = &self.configuration;
        let speed = configuration.speed.unwrap_or(1f64);
        let rules = &configuration.rules;

        for rule in rules {
            if let Some(atoms) = &mut engine.atoms {
                let atom_b = atoms.get(&rule.target).unwrap().clone();
                rule.clone().apply(
                    atoms.get_mut(&rule.src).unwrap(),
                    atom_b.to_vec(),
                    speed,
                    cv_w,
                    cv_h,
                )
            }
        }
    }
    pub fn generate_environment(
        &mut self,
        colors: HashMap<String, ColorConfiguration>,
        rules: Vec<AtomRule>,
    ) {
        let engine = &mut self.render_engine;
        let canvas = &engine.ctx.as_ref().unwrap().canvas().unwrap();
        self.configuration.colors = Some(colors);
        self.configuration.rules = rules;

        let mut atoms: HashMap<String, Vec<Atom>> = HashMap::new();

        if let Some(colors) = &self.configuration.colors {
            for (color, conf) in colors {
                for _ in 0..conf.total {
                    if atoms.contains_key(color) {
                        atoms.get_mut(color).unwrap().push(Atom::born(
                            canvas.width() as f64,
                            canvas.height() as f64,
                            0f64,
                            conf.size,
                            color.to_string(),
                            conf.shape.clone(),
                            conf.blur,
                        ))
                    } else {
                        let color_vec: Vec<Atom> = vec![Atom::born(
                            canvas.width() as f64,
                            canvas.height() as f64,
                            0f64,
                            conf.size,
                            color.to_string(),
                            conf.shape.clone(),
                            conf.blur,
                        )];
                        atoms.insert(color.clone(), color_vec);
                    }
                }
            }
        }

        engine.atoms = Some(atoms);
    }
}

static mut LIFE_INSTANCE: Mutex<LifeManager> = Mutex::new(LifeManager {
    render_engine: RenderEngine {
        tick: 64,
        rendering: false,
        atoms: None,
        ctx: None,
        frames: 0i64,
    },
    configuration: Configuration {
        colors: None,
        rules: vec![],
        speed: Some(1f64),
        padding: Some(40f64),
    },
});

#[wasm_bindgen]
pub fn initialize(canvas_id: String, colors: &JsValue, rules: &JsValue) {
    unsafe {
        let crr_state = LIFE_INSTANCE.get_mut().unwrap();
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

        crr_state.render_engine.ctx = Some(context);
        let colors = Configuration::raw_into_colors(colors);
        let rules = Configuration::raw_into_rules(rules);
        crr_state.generate_environment(colors, rules);
    }
}

#[wasm_bindgen]
pub fn update_rules(rules: &JsValue) {
    unsafe {
        let crr_state = LIFE_INSTANCE.get_mut().unwrap();
        let rules = Configuration::raw_into_rules(rules);
        crr_state.configuration.rules = rules;
    }
}

#[wasm_bindgen]
pub fn update_colors(colors: &JsValue) {
    unsafe {
        let crr_state = LIFE_INSTANCE.get_mut().unwrap();
        let engine = &mut crr_state.render_engine;
        let colors = Configuration::raw_into_colors(colors);
        let canvas = &engine.ctx.as_ref().unwrap().canvas().unwrap();

        // Update or insert new colors
        for (name, conf) in &colors {
            if let Some(atoms) = &mut engine.atoms {
                let color_atoms = atoms.get_mut(name);

                if let Some(color_atoms) = color_atoms {
                    if color_atoms.len() as i32 > conf.total {
                        color_atoms.set_len(conf.total as usize);
                    } else {
                        for _ in 0..(conf.total - color_atoms.len() as i32) {
                            color_atoms.push(Atom::born(
                                canvas.width() as f64,
                                canvas.height() as f64,
                                0f64,
                                conf.size,
                                name.to_string(),
                                conf.shape.clone(),
                                conf.blur,
                            ))
                        }
                    }

                    for mut atom in color_atoms {
                        atom.size = conf.size;
                        atom.shape = conf.shape.clone();
                    }
                } else {
                    let mut color_atoms: Vec<Atom> = vec![];
                    for _ in 0..conf.total {
                        color_atoms.push(Atom::born(
                            canvas.width() as f64,
                            canvas.height() as f64,
                            0f64,
                            conf.size,
                            name.to_string(),
                            conf.shape.clone(),
                            conf.blur,
                        ))
                    }
                    atoms.insert(name.to_string(), color_atoms);
                }
            }
        }
        // Check some color is removed
        if let Some(atoms) = &mut engine.atoms {
            let mut removeter: Vec<String> = vec![];
            atoms.into_iter().for_each(|(name, _)| {
                if colors.get(name).is_none() {
                    removeter.push(name.to_string());
                }
            });
            for removed in &removeter {
                atoms.remove(removed);
            }
        }
        crr_state.configuration.colors = Some(colors);
    }
}

#[wasm_bindgen]
pub fn start_render() {
    unsafe {
        let crr_state = LIFE_INSTANCE.get_mut().unwrap();
        crr_state.render_engine.set_rendering(true).render_piline();
    }
}

#[wasm_bindgen]
pub fn stop_render() {
    unsafe {
        let crr_state = LIFE_INSTANCE.get_mut().unwrap();
        crr_state.render_engine.set_rendering(false);
    }
}

#[wasm_bindgen]
pub fn next_frame() {
    unsafe {
        let crr_state = LIFE_INSTANCE.get_mut().unwrap();
        crr_state.update_engine_state();
        crr_state.render_engine.set_rendering(true).render_piline();
    }
}

#[wasm_bindgen]
pub fn set_speed(speed: f64) {
    unsafe {
        let crr_state = LIFE_INSTANCE.get_mut().unwrap();
        crr_state.configuration.speed = Some(speed);
    }
}

#[wasm_bindgen]
pub fn set_tick(tick: u32) {
    unsafe {
        let crr_state = LIFE_INSTANCE.get_mut().unwrap();
        crr_state.render_engine.tick = tick;
    }
}

#[wasm_bindgen]
pub fn start_loop_engine() {
    unsafe {
        let crr_state = LIFE_INSTANCE.get_mut().unwrap();
        game_loop(
            crr_state,
            120,
            0.2,
            move |g| {
                g.set_updates_per_second(g.game.render_engine.tick);
                g.game.update_engine_state();
            },
            |g| {
                g.game.render_engine.render_piline();
            },
        );
    }
}

#[wasm_bindgen]
pub fn get_crr_frame_idx() -> i64 {
    unsafe {
        let crr_state = LIFE_INSTANCE.lock().unwrap();
        crr_state.render_engine.frames
    }
}
