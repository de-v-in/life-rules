use serde::{Deserialize, Serialize};

use crate::{utils::str_to_f64, vector::Vector};

#[derive(Serialize, Deserialize, Clone)]
pub enum AtomShape {
    Dot,
    Square,
    Triangle,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Atom {
    pub p: Vector,
    pub v: Vector,
}

impl Atom {
    pub fn born(width: f64, height: f64, padding: f64) -> Self {
        Atom {
            p: Vector::random(padding, width - padding, padding, height - padding),
            v: Vector { x: 0f64, y: 0f64 },
        }
    }
    /**
     * Make all atom reborn in random position
     */
    pub fn reborn(&mut self, width: f64, height: f64, padding: f64) {
        self.p = Vector::random(padding, width - padding, padding, height - padding);
        self.v = Vector { x: 0f64, y: 0f64 };
    }
    /**
     * Change reverse direction of atom when it move out of canvas
     * This will prevent atom move outside of canvas
     */
    pub fn correct_pos(&mut self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) {
        match self {
            n if n.p.x <= min_x && n.v.x <= 0.0 => n.v.x *= -1.0,
            n if n.p.x >= max_x && n.v.x >= 0.0 => n.v.x *= -1.0,
            n if n.p.y <= min_y && n.v.y <= 0.0 => n.v.y *= -1.0,
            n if n.p.y >= max_y && n.v.y >= 0.0 => n.v.y *= -1.0,
            _ => (),
        }
    }
    /**
     * Compute new position of atom base on weight and distance between other atoms
     */
    pub fn compute(&mut self, others: &Vec<Atom>, w: f64, radius: f64) {
        let mut force = Vector { x: 0f64, y: 0f64 };
        for atom in others {
            let (d, v_force) = self.p.distance(&atom.p);
            if d > 0f64 && d < radius {
                let f = w / d;
                force.plus_amp(&v_force, f)
            }
        }
        self.v.combine_avg(force);
        self.p.plus(&self.v);
    }
}

pub struct AtomRule {
    pub src: String,
    pub target: String,
    pub w: f64,
}

impl AtomRule {
    /**
     * Convert string vector from js into rule
     */
    pub fn vec_into(input: Vec<String>) -> Self {
        AtomRule {
            src: input[0].clone(),
            target: input[1].clone(),
            w: str_to_f64(&input[2]),
        }
    }
}
