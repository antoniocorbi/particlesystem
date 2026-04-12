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
use crate::model::{particle::Particle, repeller::Repeller};
use egui::emath;
use egui::{pos2, vec2, Pos2, Vec2};
//use rand::Rng;
use rand::prelude::*;
use std::ops::Index;

// ╔═══════════════════╗
// ║ ParticleSystemApp ║
// ╚═══════════════════╝
// -- : -------------------------------------------------------------------
#[derive(Clone)]
pub struct ParticleSystem {
    particles: Vec<Particle>,
    world_rect: emath::Rect,
}

impl ParticleSystem {
    pub fn new(
        np: usize,
        x: f32,
        y: f32,
        wr: emath::Rect,
        particle_size: f32,
        particle_mass: f32,
    ) -> Self {
        // Check that point x,y is inside its world_rect.
        assert!(wr.contains(pos2(x, y)));
        let mut rng = rand::rng();
        let mass = rng.random_range(1.0..=particle_mass);

        let mut particles: Vec<Particle> = vec![];
        for i in 0..np {
            let particle = Particle::new(x, y, particle_size).with_mass(mass);
            particles.push(particle);
        }
        let world_rect = wr;
        Self {
            //particles: vec![Particle::new(x, y).with_size(15.0).with_mass(1.5); np],
            particles,
            world_rect,
        }
    }

    pub fn add_particle(&mut self, p: &Particle) {
        self.particles.push(*p);
    }

    pub fn nparticles(&self) -> usize {
        self.particles.len()
    }

    pub fn len(&self) -> usize {
        self.particles.len()
    }

    pub fn world_rect(&self) -> emath::Rect {
        self.world_rect
    }

    pub fn run(&mut self) {
        for i in 0..self.particles.len() {
            let particle = &mut self.particles[i];
            particle.run();
        }
        //println!("psystem size BEFORE cleaning: {}", self.particles.len());
        self.particles.retain(|p| !p.is_dead());
        //println!("psystem size AFTER cleaning: {}", self.particles.len());
    }

    pub fn apply_force(&mut self, force: impl Into<Vec2> + Clone) {
        self.particles
            .iter_mut()
            .for_each(|p| p.apply_force(force.clone()));
    }

    pub fn apply_repeller(&mut self, r: &Repeller) {
        self.particles.iter_mut().for_each(|p| {
            let force = r.repel(p);
            p.apply_force(force.clone())
        });
    }
}

impl Index<usize> for ParticleSystem {
    type Output = Particle;

    fn index(&self, index: usize) -> &Self::Output {
        // Delegamos el indexado al vector de partículas
        &self.particles[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{pos2, vec2, Rect};

    // Helper para crear un entorno de prueba rápido
    fn setup_test_system(n: usize) -> ParticleSystem {
        let world_rect = Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0));
        ParticleSystem::new(n, 50.0, 50.0, world_rect, 10.0, 2.5)
    }

    #[test]
    fn test_system_initialization() {
        let n_particles = 10;
        let ps = setup_test_system(n_particles);

        // Verificar que se han creado el número correcto de partículas
        assert_eq!(ps.particles.len(), n_particles);

        // Verificar que el world_rect se ha asignado correctamente
        assert_eq!(ps.world_rect().width(), 100.0);
    }

    #[test]
    #[should_panic(expected = "contains")]
    fn test_system_panic_outside_bounds() {
        // Este test debe fallar (panic) porque el punto de origen está fuera del rect
        let world_rect = Rect::from_min_max(pos2(0.0, 0.0), pos2(10.0, 10.0));
        ParticleSystem::new(5, 50.0, 50.0, world_rect, 10.2, 2.5);
    }

    #[test]
    fn test_particle_movement() {
        let mut p = Particle::new(0.0, 0.0, 5.6);
        p.with_velocity(vec2(1.0, 1.0));
        p.with_accel(vec2(0.5, 0.0));

        // Primer update: vel se vuelve (1.5, 1.0), pos se vuelve (1.5, 1.0)
        p.update();

        assert!(p.position.x > 0.0);
        assert_eq!(p.position.y, 1.0);
    }

    #[test]
    fn test_particle_lifespan_death() {
        let mut p = Particle::new(0.0, 0.0, 5.6);
        let initial_life = p.lifespan;

        p.update();

        // Verificar que la vida disminuye
        assert!(p.lifespan < initial_life);

        // Forzar muerte (suponiendo que LIFE_DELTA es constante)
        while p.lifespan > 10 {
            // Evitamos el overflow si LIFE_DELTA no cuadra exacto
            p.update();
        }

        // Si tu lógica lo permite, podrías setear la vida a 0 directamente
        p.with_life(0);
        assert!(p.is_dead());
    }

    #[test]
    fn test_system_update_consistency() {
        let mut ps = setup_test_system(5);
        let initial_pos = ps.particles[0].position;

        // Simulamos un movimiento en todas las partículas
        for p in &mut ps.particles {
            p.with_velocity(vec2(1.0, 0.0));
        }

        // Aquí deberías tener un método en ParticleSystem que haga update a todas
        // ps.update();

        // Si no lo tienes, lo hacemos manual para el test:
        for p in &mut ps.particles {
            p.update();
        }

        assert_ne!(ps.particles[0].position, initial_pos);
    }
}
