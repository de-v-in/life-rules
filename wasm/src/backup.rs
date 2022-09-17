mod utils;

extern crate serde_json;
extern crate wasm_bindgen;

use std::f64::consts::PI;
use std::{collections::HashMap, sync::Mutex};

use log::info;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/**
 * Generate random number in rage width padding
 */
fn random(range: f64, padding: f64) -> f64 {
    let seed: f64 = rand::thread_rng().gen();
    seed * range + padding * (1f64 - 2f64 * seed)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum ColorShape {
    Square,
    Dot,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[wasm_bindgen]
struct ColorConfig {
    total: i32
    size: f64,
    blur: Option<f64>,
    shape: Option<ColorShape>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Atom {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    size: f64,
    color: String,
}

impl Atom {
    pub fn create_random(
        mx_width: f64,
        mx_height: f64,
        padding: f64,
        size: f64,
        color: String,
    ) -> Atom {
        Atom {
            x: random(mx_width, padding),
            y: random(mx_height, padding),
            size,
            vx: 0f64,
            vy: 0f64,
            color,
        }
    }

    pub fn create_random_atoms(
        color_name: &str,
        config: &ColorConfig,
        mx_width: f64,
        mx_height: f64,
        padding: f64,
    ) -> Vec<Atom> {
        let mut atoms: Vec<Atom> = vec![];
        for _ in 0..config.total {
            atoms.push(Atom::create_random(
                mx_width,
                mx_height,
                padding,
                config.size,
                color_name.to_string(),
            ))
        }
        atoms
    }
    pub fn draw(&self, ctx: &CanvasRenderingContext2d, color_conf: &ColorConfig) {
        ctx.begin_path();
        let draw_square_default = || {
            ctx.set_fill_style(&JsValue::from(self.color.clone()));
            ctx.fill_rect(self.x, self.y, color_conf.size, color_conf.size);

            // Draw outline
            let mut lower_color = self.color.clone();
            let outline_size: f64 = color_conf.size + 2f64;
            lower_color.push_str("99");
            ctx.set_fill_style(&JsValue::from(lower_color));
            ctx.fill_rect(self.x - 1f64, self.y - 1f64, outline_size, outline_size);
        };
        match color_conf.shape {
            Some(ColorShape::Dot) => {
                ctx.arc(self.x, self.y, color_conf.size / 2f64, 0f64, 2f64 * PI)
                    .unwrap();
                ctx.set_fill_style(&JsValue::from(self.color.clone()));
                ctx.fill();

                // Draw outline
                let mut lower_color = self.color.clone();
                let outline_size: f64 = (color_conf.size / 2f64) + 2f64;
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
        if let Some(blur_radius) = color_conf.blur {
            ctx.set_shadow_color(&self.color);
            ctx.set_shadow_blur(blur_radius);
            ctx.set_shadow_offset_y(0f64);
            ctx.set_shadow_offset_x(0f64);
        }
        ctx.close_path()
    }
}

#[derive(Debug)]
struct ColorRule {
    color_a: String,
    color_b: String,
    weight: f64,
}

impl ColorRule {
    fn conver(input: Vec<String>) -> ColorRule {
        ColorRule {
            color_a: input[0].clone(),
            color_b: input[1].clone(),
            weight: str::parse::<f64>(&input[2]).unwrap(),
        }
    }
}

#[derive(Debug)]
struct RuleConfiguration {
    colors: HashMap<String, ColorConfig>,
    rules: Vec<ColorRule>,
}

impl RuleConfiguration {
    pub fn convert_rules(raw: Vec<Vec<String>>) -> Vec<ColorRule> {
        raw.into_iter().map(ColorRule::conver).collect()
    }
    pub fn raw_into_rules(raw: &JsValue) -> Vec<ColorRule> {
        RuleConfiguration::convert_rules(raw.into_serde().unwrap())
    }
}

#[derive(Debug)]
struct GlobalState {
    atoms: Option<HashMap<String, Vec<Atom>>>,
    canvas_w: f64,
    canvas_h: f64,
    speed: f64,
    random_padding: f64,
    rendering: bool,
    configuration: Option<RuleConfiguration>,
}

impl GlobalState {
    pub fn set_render(mut self, status: bool) {
        self.rendering = status
    }
    pub fn set_speed(mut self, speed: f64) {
        self.speed = speed
    }
    pub fn set_rand_padding(mut self, padding: f64) {
        self.random_padding = padding
    }
    pub fn set_size(mut self, width: f64, height: f64) {
        self.canvas_w = width;
        self.canvas_h = height;
    }
}

static mut STATES: Mutex<GlobalState> = Mutex::new(GlobalState {
    atoms: None,
    rendering: false,
    speed: 1f64,
    canvas_w: 0f64,
    canvas_h: 0f64,
    random_padding: 40f64,
    configuration: None,
});

#[wasm_bindgen]
pub fn main(cv_w: f64, cv_h: f64) {
    // console_log::init_with_level(Level::Debug).unwrap();
    unsafe {
        let mut crr_state = STATES.get_mut().unwrap();
        crr_state.atoms = Some(HashMap::new());
        crr_state.canvas_w = cv_w;
        crr_state.canvas_h = cv_h;
    }
}

#[wasm_bindgen]
pub fn set_render(status: bool) {
    unsafe {
        let mut state = STATES.get_mut().unwrap();
        if state.rendering && !status {
            state.rendering = status
        } else if !state.rendering && status {
            state.rendering = status;
        }
        info!("Render setted: {:?}", state);
    }
}

#[wasm_bindgen]
pub fn set_speed(n_speed: f64) {
    unsafe {
        let mut crr_state = STATES.get_mut().unwrap();
        crr_state.speed = n_speed
    }
}

#[wasm_bindgen]
pub fn initial_configuration(totals: &JsValue, rules: &JsValue) {
    unsafe {
        let mut crr_state = STATES.get_mut().unwrap();
        let config: HashMap<String, ColorConfig> = totals.into_serde().unwrap();
        let mut initial_atom: HashMap<String, Vec<Atom>> = HashMap::new();

        for (name, color_conf) in &config {
            initial_atom.insert(
                name.clone(),
                Atom::create_random_atoms(
                    name,
                    color_conf,
                    crr_state.canvas_w,
                    crr_state.canvas_h,
                    crr_state.random_padding,
                ),
            );
        }

        crr_state.configuration = Some(RuleConfiguration {
            colors: config,
            rules: RuleConfiguration::raw_into_rules(rules),
        });
        crr_state.atoms = Some(initial_atom);

        info!("Rule setted: {:?}", crr_state);
    }
}

#[wasm_bindgen]
pub fn update_rule(raw: &JsValue) {
    unsafe {
        let crr_state = STATES.get_mut().unwrap();
        crr_state.configuration.as_mut().unwrap().rules = RuleConfiguration::raw_into_rules(raw);
    }
}

#[wasm_bindgen]
// TODO: Update atoms when rendering, current it broken
pub fn update_colors(config: &JsValue) {
    unsafe {
        let crr_state = STATES.get_mut().unwrap();
        let config: HashMap<String, ColorConfig> = config.into_serde().unwrap();
        let mut initial_atom: HashMap<String, Vec<Atom>> = HashMap::new();
        let current_atoms = crr_state.atoms.as_ref();

        for (name, color_conf) in &config {
            let mut atoms: Vec<Atom> = current_atoms.unwrap().get(&name.clone()).unwrap().to_vec();
            if atoms.len() as i32 > color_conf.total {
                let size = color_conf.total as usize;
                atoms = atoms[0..size].to_vec();
            } else {
                for _ in 0..(color_conf.total - atoms.len() as i32) {
                    atoms.push(Atom {
                        x: random(crr_state.canvas_w, 40f64),
                        y: random(crr_state.canvas_h, 40f64),
                        size: color_conf.size,
                        vx: 0f64,
                        vy: 0f64,
                        color: name.clone(),
                    })
                }
            }
            (0..atoms.len()).for_each(|i| atoms[i].size = color_conf.size);
            initial_atom.insert(name.clone(), atoms);
        }
        crr_state.configuration.as_mut().unwrap().colors = config;
        crr_state.atoms = Some(initial_atom);
    }
}

fn rule_calculator(
    atoms1: &mut Vec<Atom>,
    atoms2: Vec<Atom>,
    g: f64,
    speed: f64,
    cv_w: f64,
    cv_h: f64,
    point_size: f64,
) {
    for a in atoms1 {
        let mut fx = 0f64;
        let mut fy = 0f64;

        for j in &atoms2 {
            let dx = a.x - j.x;
            let dy = a.y - j.y;
            let d = (dx * dx + dy * dy).sqrt();
            if d > 0f64 && d < 80f64 * speed {
                let f = g / d;
                fx += f * dx;
                fy += f * dy;
            }
        }

        a.vx = (a.vx + fx) * 0.5 * speed;
        a.vy = (a.vy + fy) * 0.5 * speed;
        a.x += a.vx;
        a.y += a.vy;

        if a.x <= 0f64 || a.x >= cv_w {
            if a.x <= 0f64 {
                a.x = 0f64 + point_size
            } else {
                a.x = cv_w - point_size
            }
            a.vx *= -1f64;
        }
        if a.y <= 0f64 || a.y >= cv_h {
            if a.y <= 0f64 {
                a.y = 0f64 + point_size
            } else {
                a.y = cv_h - point_size
            }
            a.vy *= -1f64;
        }
    }
}

#[wasm_bindgen]
pub fn start_render() {
    unsafe {
        let crr_state = STATES.get_mut().unwrap();
        if !crr_state.rendering {
            return;
        }

        let configuration = &crr_state.configuration.as_ref().unwrap();
        let rules = &configuration.rules;
        let colors = &configuration.colors;

        for rule in rules {
            let atoms = crr_state.atoms.as_mut().unwrap();
            // let atom_a = atoms.get(&rule.color_a).as_mut().unwrap();
            let atom_a_conf = colors.get(&rule.color_a).unwrap();
            let atom_b = atoms.get(&rule.color_b).unwrap().clone();
            {
                rule_calculator(
                    atoms.get_mut(&rule.color_a).unwrap(),
                    atom_b.to_vec(),
                    rule.weight,
                    crr_state.speed,
                    crr_state.canvas_w,
                    crr_state.canvas_h,
                    atom_a_conf.size,
                );
            }
        }
        render_canvas();
    }
}

#[wasm_bindgen]
pub fn render_canvas() {
    unsafe {
        let crr_state = STATES.get_mut().unwrap();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("life").unwrap();
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

        context.clear_rect(0f64, 0f64, crr_state.canvas_w, crr_state.canvas_h);
        let configuration = crr_state.configuration.as_ref().unwrap();
        let atoms = crr_state.atoms.as_ref().unwrap();

        for (name, color_conf) in &configuration.colors {
            let color_atoms = atoms.get(name).unwrap();
            for atom in color_atoms {
                atom.draw(&context, color_conf)
            }
        }
    }
}
