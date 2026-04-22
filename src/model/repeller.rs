// Copyright (C) 2026  Antonio-Miguel Corbi Bellot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::constants;
use crate::model::particle::Particle;
use egui::{emath, pos2, text_selection::text_cursor_state, vec2, Pos2, Vec2};
use rand::prelude::*;

// ╔══════════╗
// ║ Repeller ║
// ╚══════════╝
// -- : -------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Repeller {
    pub position: Pos2,
    pub size: f32,
    pub power: f32,
}

impl Repeller {
    pub fn new(x: f32, y: f32, power: f32, size: f32) -> Self {
        let mut rng = rand::rng();
        let power = rng.random_range(1.0..=power);
        let power = power / constants::REP_POWER_DIV;
        let size = rng.random_range(constants::MIN_RSIZE..=size);

        Self {
            position: [x, y].into(),
            size,
            power,
        }
    }

    pub fn repel(&self, p: &mut Particle) -> Vec2 {
        let mut force = self.position - p.position;
        let mut distance = force.to_pos2().distance_sq(Pos2::default());
        // let distance = force.to_pos2().distance(Pos2::default());
        // distance = constrain(distance, 5, 50);
        distance = distance.clamp(5.0, 50.0);
        let strength = -1.0 * self.power / distance;

        force = force.normalized() * strength;
        force
    }

    pub fn wsize(&self) -> f32 {
        emath::remap_clamp(self.size, 0.0..=constants::REP_SIZE, 0.0..=1.0)
    }
}
