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
use egui::{pos2, text_selection::text_cursor_state, vec2, Pos2, Vec2};
use rand::prelude::*;

// ╔══════════╗
// ║ Particle ║
// ╚══════════╝
// -- : -------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    pub position: Pos2,
    pub size: f32,
    pub velocity: Vec2,
    pub accel: Vec2,
    pub lifespan: u8,
    pub mass: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        let mut rng = rand::rng();
        let velx = rng.random_range(-0.0008..=0.0008);
        let vely = rng.random_range(-0.0005..=0.0000001);
        let mass = rng.random_range(1.0..=20.0);
        let size = rng.random_range(1.0..=size);

        // println!(
        //     "Particle@{}/{}: vx:{}, vy:{} , size:{}",
        //     x,
        //     y,
        //     velx,
        //     vely,
        //     constants::PSIZE
        // );

        Self {
            position: [x, y].into(),
            size,
            velocity: [velx, vely].into(),
            accel: [0.0, 0.0].into(),
            lifespan: 255,
            mass,
        }
    }

    pub fn apply_force(&mut self, force: impl Into<Vec2>) {
        let f: Vec2 = force.into() / self.mass;
        self.accel += f;
    }

    pub fn with_velocity(&mut self, v: impl Into<Vec2>) -> Self
// where
    //     T: Into<Vec2>,
    {
        self.velocity = v.into();
        *self
    }

    pub fn with_accel<T>(&mut self, a: T) -> Self
    where
        T: Into<Vec2>,
    {
        self.accel = a.into();
        *self
    }

    pub fn with_life(&mut self, life: u8) -> Self {
        self.lifespan = life;
        *self
    }

    pub fn with_mass(&mut self, mass: f32) -> Self {
        self.mass = mass;
        *self
    }

    pub fn with_size(&mut self, size: f32) -> Self {
        self.size = size;
        *self
    }

    fn update_life(&mut self) {
        const FRAME_COUNT_INI: u16 = 30;
        static mut FRAME_COUNT: u16 = FRAME_COUNT_INI;
        unsafe {
            FRAME_COUNT -= 1;
            if FRAME_COUNT == 0 {
                FRAME_COUNT = FRAME_COUNT_INI;
                if self.lifespan >= constants::LIFE_DELTA {
                    self.lifespan -= constants::LIFE_DELTA;
                } else {
                    self.lifespan = 0;
                }
                // println!("Life: {}", self.lifespan);
            }
        };
    }

    pub fn update(&mut self) {
        //let gravity = vec2(0.0, 0.1);
        self.apply_force(constants::GRAVITY);
        self.velocity += self.accel;
        self.position += self.velocity;
        self.accel = Vec2::splat(0.0);

        self.update_life();
        // println!("lifespan: {}", self.lifespan);
    }

    pub fn run(&mut self) {
        //dbg!("particle::run");
        self.update();
    }

    pub fn is_dead(&self) -> bool {
        self.lifespan == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifespan() {
        let p = Particle::new(10.0, 2.5, 5.6).with_life(12);
        assert_eq!(p.lifespan, 12);
        assert_eq!(p.position, [10.0, 2.5].into());
    }

    #[test]
    fn lifespan_update() {
        let mut p = Particle::new(10.0, 2.5, 5.6).with_life(10);
        p.update();
        assert_eq!(p.lifespan, 8);
    }

    #[test]
    fn accel_update() {
        let mut p = Particle::new(10.0, 2.5, 5.6).with_accel([3.6, 2.2]);
        p.update();
        assert_eq!(p.accel, [0.0, 0.0].into());
    }

    #[test]
    fn velocity_update() {
        let mut p = Particle::new(10.0, 2.5, 5.6).with_accel([3.6, 2.2]);
        p.update();
        assert_eq!(p.velocity, [3.6, 2.2].into());
    }

    #[test]
    fn test_apply_force_accumulation() {
        let mut p = Particle::new(0.0, 0.0, 4.5);

        // Aplicamos dos fuerzas distintas
        p.apply_force(vec2(1.0, 0.0));
        p.apply_force(vec2(0.0, 2.0));

        // La aceleración debe ser la suma de ambas
        assert_eq!(p.accel, vec2(1.0, 2.0));
    }

    #[test]
    fn test_update_physics_step() {
        let mut p = Particle::new(10.0, 10.0, 10.0);
        p.apply_force(vec2(1.0, 0.0)); // Accel = (1, 0)

        p.update();

        // 1. Velocity += Accel -> (0,0) + (1,0) = (1,0)
        assert_eq!(p.velocity, vec2(1.0, 0.0));
        // 2. Position += Velocity -> (10,10) + (1,0) = (11,10)
        assert_eq!(p.position, pos2(11.0, 10.0));
        // 3. Accel debe resetearse a 0
        assert_eq!(p.accel, vec2(0.0, 0.0));
    }

    #[test]
    fn test_run_lifecycle() {
        let mut p = Particle::new(0.0, 0.0, 1.2);
        let initial_life = p.lifespan;

        // 'run' ejecuta update() y luego aplica gravedad para el PRÓXIMO frame
        p.run();

        // Verificamos que la vida ha bajado
        assert!(p.lifespan < initial_life);

        // Verificamos que la gravedad (0.0, 0.1) se ha aplicado a la aceleración
        // DESPUÉS de que update() la reseteara.
        assert_eq!(p.accel, vec2(0.0, 0.1));
    }

    #[test]
    fn test_apply_force_with_different_types() {
        let mut p = Particle::new(0.0, 0.0, 5.5);

        // Gracias a impl Into<Vec2>, podemos pasar arrays o Vec2 directamente
        p.apply_force([1.0, 2.0]);
        assert_eq!(p.accel, vec2(1.0, 2.0));

        p.apply_force(vec2(1.0, 1.0));
        assert_eq!(p.accel, vec2(2.0, 3.0));
    }
}
