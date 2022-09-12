mod utils;

extern crate serde_json;
extern crate wasm_bindgen;

use std::f64::consts::PI;
use std::{collections::HashMap, sync::Mutex};

use log::{info, Level};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ColorConfig {
    total: i32,
    size: f64,
}

#[derive(Debug)]
struct RuleConfiguration {
    colors: HashMap<String, ColorConfig>,
    rules: Vec<ColorRule>,
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

#[derive(Debug)]
struct GlobalState {
    atoms: Option<HashMap<String, Vec<Atom>>>,
    canvas_size: f64,
    random_padding: f64,
    rendering: bool,
    configuration: Option<RuleConfiguration>,
}

static mut STATES: Mutex<GlobalState> = Mutex::new(GlobalState {
    atoms: None,
    rendering: false,
    canvas_size: 0f64,
    random_padding: 40f64,
    configuration: None,
});

#[wasm_bindgen]
pub fn main(cv_size: f64) {
    // console_log::init_with_level(Level::Debug).unwrap();
    unsafe {
        let mut crr_state = STATES.get_mut().unwrap();
        crr_state.atoms = Some(HashMap::new());
        crr_state.canvas_size = cv_size
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

fn random(size: f64, padding: f64) -> f64 {
    let seed: f64 = rand::thread_rng().gen();
    seed * size + padding * (1f64 - 2f64 * seed)
}

#[wasm_bindgen]
pub fn initial_configuration(totals: &JsValue, rules: &JsValue) {
    unsafe {
        let mut crr_state = STATES.get_mut().unwrap();
        let config: HashMap<String, ColorConfig> = totals.into_serde().unwrap();
        let mut initial_atom: HashMap<String, Vec<Atom>> = HashMap::new();
        let size = crr_state.canvas_size;

        for (name, color_conf) in &config {
            // initial_atom.insert(k, v);
            let mut atoms: Vec<Atom> = vec![];
            for _ in 0..color_conf.total {
                atoms.push(Atom {
                    x: random(size, 40f64),
                    y: random(size, 40f64),
                    size: color_conf.size,
                    vx: 0f64,
                    vy: 0f64,
                    color: name.clone(),
                })
            }
            initial_atom.insert(name.clone(), atoms);
        }

        let rules: Vec<Vec<String>> = rules.into_serde().unwrap();

        let rule_converted: Vec<ColorRule> =
            rules.into_iter().map(|x| ColorRule::conver(x)).collect();

        crr_state.configuration = Some(RuleConfiguration {
            colors: config,
            rules: rule_converted,
        });
        crr_state.atoms = Some(initial_atom);

        info!("Rule setted: {:?}", crr_state);
    }
}

#[wasm_bindgen]
pub fn update_rule(rules: &JsValue) {
    unsafe {
        let crr_state = STATES.get_mut().unwrap();
        let rules: Vec<Vec<String>> = rules.into_serde().unwrap();
        let rule_converted: Vec<ColorRule> =
            rules.into_iter().map(|x| ColorRule::conver(x)).collect();
        crr_state.configuration.as_mut().unwrap().rules = rule_converted;
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
        let size = crr_state.canvas_size;

        for (name, color_conf) in &config {
            let mut atoms: Vec<Atom> = current_atoms.unwrap().get(&name.clone()).unwrap().to_vec();
            if atoms.len() as i32 > color_conf.total {
                let size = color_conf.total as usize;
                atoms = atoms[0..size].to_vec();
            } else {
                for _ in 0..(color_conf.total - atoms.len() as i32) {
                    atoms.push(Atom {
                        x: random(size, 40f64),
                        y: random(size, 40f64),
                        size: color_conf.size,
                        vx: 0f64,
                        vy: 0f64,
                        color: name.clone(),
                    })
                }
            }
            for i in 0..atoms.len() {
                atoms[i].size = color_conf.size.clone()
            }
            initial_atom.insert(name.clone(), atoms);
        }
        crr_state.configuration.as_mut().unwrap().colors = config;
        crr_state.atoms = Some(initial_atom);
    }
}

fn rule_calculator(
    in1: Vec<Atom>,
    in2: Vec<Atom>,
    g: f64,
    cv_size: f64,
    point_size: f64,
) -> Vec<Atom> {
    let mut atoms1: Vec<Atom> = in1;
    let atoms2: Vec<Atom> = in2;

    for a in &mut atoms1 {
        let mut fx = 0f64;
        let mut fy = 0f64;

        for j in &atoms2 {
            let dx = a.x - j.x;
            let dy = a.y - j.y;
            let d = (dx * dx + dy * dy).sqrt();
            if d > 0f64 && d < 80f64 {
                let f = g / d;
                fx += f * dx;
                fy += f * dy;
            }
        }

        a.vx = (a.vx + fx) * 0.5;
        a.vy = (a.vy + fy) * 0.5;
        a.x += a.vx;
        a.y += a.vy;

        if a.x <= 0f64 || a.x >= cv_size {
            if a.x <= 0f64 {
                a.x = 0f64
            } else {
                a.x = cv_size - point_size
            }
            a.vx = a.vx * -1f64;
        }
        if a.y <= 0f64 || a.y >= cv_size {
            if a.y <= 0f64 {
                a.y = 0f64
            } else {
                a.y = cv_size - point_size
            }
            a.vy = a.vy * -1f64;
        }
    }
    atoms1
}

#[wasm_bindgen]
pub fn start_render() {
    unsafe {
        let crr_state = STATES.get_mut().unwrap();
        let atoms = crr_state.atoms.clone().unwrap();
        let configuration = &crr_state.configuration.as_ref().unwrap();
        let rules = &configuration.rules;
        let colors = &configuration.colors;
        let size = crr_state.canvas_size;

        if crr_state.rendering {
            for rule in rules {
                let atom_a = atoms.get(&rule.color_a).unwrap();
                let atom_a_conf = colors.get(&rule.color_a).unwrap();
                let atom_b = atoms.get(&rule.color_b).unwrap();
                let output = rule_calculator(
                    atom_a.clone(),
                    atom_b.clone(),
                    rule.weight,
                    size,
                    atom_a_conf.size,
                );
                crr_state
                    .atoms
                    .as_mut()
                    .unwrap()
                    .insert(rule.color_a.clone(), output);
            }
            render_canvas();
        }
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
        let size = canvas.width();

        crr_state.canvas_size = size as f64;

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.clear_rect(0f64, 0f64, size as f64, size as f64);
        let configuration = crr_state.configuration.as_ref().unwrap();
        let atoms = crr_state.atoms.as_ref().unwrap();

        for (name, conf) in &configuration.colors {
            let color_atoms = atoms.get(name).unwrap();
            for atom in color_atoms {
                context.begin_path();
                context
                    .arc(atom.x, atom.y, conf.size / 2f64, 0f64, 2f64 * PI)
                    .unwrap();
                context.set_fill_style(&JsValue::from(name));
                context.fill();
                context.set_shadow_color(name);
                context.set_shadow_blur(6f64);
                context.set_shadow_offset_x(0f64);
                context.set_shadow_offset_y(0f64);
                context.close_path();
            }
        }
    }
}
