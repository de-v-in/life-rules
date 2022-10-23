mod atom;
mod atom_set;
mod system;
mod utils;
mod vector;

use std::{collections::HashMap, sync::Mutex};

use game_loop::game_loop;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

use self::system::System;

static mut SYSTEM: Mutex<System> = Mutex::new(System {
    ctx: None,
    atoms: None,
    entropy: 1.0,
    rules: vec![],
    frame_count: 0,
    rendering: false,
    compute_tick: 64,
    canvas_padding: 10f64,
    engine_loaded: false,
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
pub fn change_entropy(entropy: f64) {
    unsafe {
        let crr_state = SYSTEM.get_mut().unwrap();
        crr_state.entropy = entropy
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
