use serde::{Deserialize, Serialize};

use crate::utils::random_range;

#[derive(Serialize, Deserialize, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
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
    pub fn combine_avg(&mut self, other: Vector) {
        self.x = (self.x + other.x) * 0.5;
        self.y = (self.y + other.y) * 0.5;
    }
}
