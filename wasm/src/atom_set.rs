use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::atom::{Atom, AtomShape};

#[derive(Serialize, Deserialize, Clone)]
pub struct AtomConf {
    /**
     * Total atom for this config
     */
    pub total: i32,
    /**
     * Size of each atom in canvas (px)
     */
    pub point_size: f64,
    /**
     * Blur radius of this atom, None by default
     */
    pub blur_radius: Option<f64>,
    /**
     * Radius of effect of weight around atom (R, px)
     * @default 80px
     */
    pub compute_radius: Option<f64>,
    /**
     * Shape of atom in canvas, default is Square
     */
    pub shape: Option<AtomShape>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AtomSet {
    pub color: String,
    pub config: AtomConf,
    pub atoms: Vec<Atom>,
}

impl AtomSet {
    pub fn apply_rule(&mut self, other: &AtomSet, w: f64) {
        let radius = self.config.compute_radius.unwrap_or(80.0);
        self.atoms
            .iter_mut()
            .for_each(|atom_src| atom_src.compute(&other.atoms, w, radius));
    }
    pub fn correct_pos(&mut self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) {
        self.atoms
            .iter_mut()
            .for_each(|atom_src| atom_src.correct_pos(min_x, max_x, min_y, max_y));
    }
    pub fn reborn(&mut self, max_x: f64, max_y: f64, padding: f64) {
        self.atoms
            .iter_mut()
            .for_each(|atom_src| atom_src.reborn(max_x, max_y, padding));
    }
    pub fn draw(self, ctx: &CanvasRenderingContext2d) {
        let point_size = self.config.point_size;
        for point in self.atoms {
            ctx.begin_path();
            let mut lower_color = self.color.clone();
            let outline_size: f64 = point_size + 4f64;
            lower_color.push_str("99");
            match self.config.shape {
                _ => {
                    ctx.set_fill_style(&JsValue::from(lower_color));
                    ctx.fill_rect(
                        point.p.x - 2f64,
                        point.p.y - 2f64,
                        outline_size,
                        outline_size,
                    );
                    ctx.set_fill_style(&JsValue::from(self.color.clone()));
                    ctx.fill_rect(point.p.x, point.p.y, point_size, point_size);
                }
            }
            if let Some(blur_radius) = self.config.blur_radius {
                ctx.set_shadow_color(&self.color);
                ctx.set_shadow_blur(blur_radius);
                ctx.set_shadow_offset_y(0f64);
                ctx.set_shadow_offset_x(0f64);
            }
            ctx.close_path()
        }
    }
}
