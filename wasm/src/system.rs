use std::collections::HashMap;

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::{
    atom::{Atom, AtomRule},
    atom_set::{AtomConf, AtomSet},
};

/**
 * System of app, include all state and actions
 */
pub struct System {
    /**
     * True if system is initialized
     */
    pub engine_loaded: bool,
    /**
     * Control how much compute in a second
     */
    pub compute_tick: u32,
    /**
     * Padding around canvas so that atom not move outside
     */
    pub canvas_padding: f64,
    /**
     * Total rendered frame count, this will be use for calculate fps
     */
    pub frame_count: i64,
    /**
     * True if system is rendering, false mean pause
     */
    pub rendering: bool,
    /**
     * Entropy of weight, higher mean atom will have more energy
     */
    pub entropy: f64,
    /**
     * Rules between atoms, include source, target and weight
     */
    pub rules: Vec<AtomRule>,
    /**
     * Current atoms states of system, all calculation base on this
     */
    pub atoms: Option<HashMap<String, AtomSet>>,
    /**
     * Canvas where atoms will render
     */
    pub ctx: Option<CanvasRenderingContext2d>,
}

impl System {
    /**
     * Inject raw js object rules from web app call
     */
    pub fn load_js_rules(&mut self, raw: &JsValue) {
        let raw: Vec<Vec<String>> = raw.into_serde().unwrap();
        self.rules = raw.into_iter().map(AtomRule::vec_into).collect();
    }
    /**
     * Inject raw js color config from web app call
     */
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
                            .push(Atom::born(cv_w, cv_h, self.canvas_padding))
                    }
                }
                color_atoms.config = conf.clone();
                // Already have this color in conf -> ignore create new
                continue;
            }
            // Insert new color set
            let mut atom_set: Vec<Atom> = vec![];
            for _ in 0..conf.total {
                atom_set.push(Atom::born(cv_w, cv_h, self.canvas_padding))
            }
            atoms.insert(
                name.to_string(),
                AtomSet {
                    atoms: atom_set,
                    color: name.to_string(),
                    config: conf.clone(),
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
    
    /**
     * Refresh all atoms into random state
     */
    pub fn refresh(&mut self) {
        let canvas = &self.ctx.as_ref().unwrap().canvas().unwrap();
        let cv_w = canvas.width() as f64;
        let cv_h = canvas.height() as f64;
        let padding = self.canvas_padding;
        let atoms = self.atoms.as_mut().unwrap();
        atoms
            .iter_mut()
            .for_each(move |(_, atom_set)| atom_set.reborn(cv_w, cv_h, padding));
    }
    /**
     * Calculation next state of atoms
     */
    pub fn compute_next_tick(&mut self) {
        if self.rendering && self.engine_loaded {
            let rules = &self.rules;
            for rule in rules {
                if let Some(atoms) = &mut self.atoms {
                    let atom_b = atoms.get(&rule.target).unwrap().clone();
                    atoms
                        .get_mut(&rule.src)
                        .unwrap()
                        .apply_rule(&atom_b, rule.w * self.entropy)
                }
            }
        }
    }
    /**
     * Check if atoms is went outside of canvas and correct it
     */
    pub fn correct_point_position(&mut self) {
        if self.rendering && self.engine_loaded {
            let canvas = self.ctx.as_ref().unwrap().canvas().unwrap();
            let padding = self.canvas_padding;
            let cv_w = canvas.width() as f64 - padding;
            let cv_h = canvas.height() as f64 - padding;
            self.atoms
                .as_mut()
                .unwrap()
                .iter_mut()
                .for_each(move |(_, color)| color.correct_pos(padding, cv_w, padding, cv_h))
        }
    }
    /**
     * Render current state of atoms into canvas
     */
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
