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
use crate::model::{particlesystem::ParticleSystem, repeller::Repeller};
use egui::{emath, pos2, Color32, Pos2, Rect, Vec2};
use signals2::*;
use std::{ops::Index, sync::Arc};

// ╔══════════════╗
// ║ PSystemAppUi ║
// ╚══════════════╝
// -- : -------------------------------------------------------------------
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct PSystemAppUi {
    // ╔═════════╗
    // ║ Widgets ║
    // ╚═════════╝
    pub psystems: Vec<ParticleSystem>,
    pub worlds: Option<Worlds>,
    pub repellers: Vec<Repeller>,

    // ╔═══════╗
    // ║ State ║
    // ╚═══════╝
    //pub label: String,
    pub grid_size: f32,
    pub particle_size: f32,
    pub particle_mass: f32,
    pub status_text: String,
    pub status_text_color: egui::Color32,
    pub last_time: f64,
    pub delta_time: f64,
    pub grid: bool,
    pub tasks: Signal<(f64,)>,
}

impl Default for PSystemAppUi {
    fn default() -> Self {
        //let world_rect = emath::Rect::from_points(&[pos2(0.0, 0.0), pos2(1.0, 1.0)]);
        let psystems = vec![];
        let repellers = vec![];
        let tasks = Signal::new();

        Self {
            // Example stuff:
            psystems,
            worlds: None,
            repellers,
            grid_size: 2.5,
            particle_size: constants::MAX_PSIZE,
            particle_mass: constants::MAX_PMASS,
            status_text: "Welcome to Particle System Simulator v1.0".to_owned(),
            status_text_color: egui::Color32::DEBUG_COLOR,
            last_time: 0.0,
            delta_time: constants::DELTA_TIME,
            grid: false,
            tasks,
        }
    }
}

impl Index<usize> for PSystemAppUi {
    type Output = ParticleSystem;

    fn index(&self, index: usize) -> &Self::Output {
        // Delegamos el indexado al vector de sistemas partículas
        &self.psystems[index]
    }
}

impl PSystemAppUi {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //let p = Particle::new(10.0, 2.5).with_velocity([1.0, 2.0]);

        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        // } else {
        //     Default::default()
        // }
        Default::default()
    }

    pub fn add_task<F>(&mut self, f: F)
    where
        F: Fn(f64) + Send + Sync + 'static,
    {
        self.tasks.connect(f);
    }

    pub fn nparticles(&self) -> usize {
        self.psystems.iter().fold(0, |acc, ps| acc + ps.len())
    }

    pub fn run(&mut self) {
        for ps in &mut self.psystems {
            // Update particles state
            //println!("Updatint psystem status");
            ps.run();
            self.repellers.iter().for_each(|r| ps.apply_repeller(r));
            //if self.repeller.is_some() {
            //    ps.apply_repeller(self.repeller.as_ref().unwrap());
            //}
        }
        self.psystems.retain(|ps| ps.len() != 0);
    }

    pub fn create_worlds(&mut self, wr: Rect, sr: Rect) {
        self.worlds = Some(Worlds::new(wr, sr));
    }

    pub fn init_worlds(&mut self, sr: Rect) {
        if self.worlds.is_none() {
            let wr = egui::Rect::from_min_max(constants::WR_MIN.into(), constants::WR_MAX.into());
            self.create_worlds(wr, sr);
            // self.set_status_text(&format!("Worlds created"), egui::Color32::RED);
        } else {
            // Update worlds screen_rect
            self.worlds.as_mut().unwrap().update_screen_rect(sr);
        }
    }

    pub fn pos2_to_screen(&self, pos: Pos2) -> Pos2 {
        self.worlds.as_ref().unwrap().pos2_to_screen(pos)
    }

    pub fn pos2_to_world(&self, pos: Pos2) -> Pos2 {
        self.worlds.as_ref().unwrap().pos2_to_world(pos)
    }

    pub fn set_status_text(&mut self, t: &str, color: egui::Color32) {
        self.status_text = t.to_owned();
        self.status_text_color = color;
    }

    pub fn run_tasks(&mut self, ui: &egui::Ui) {
        let current_time = ui.input(|i| i.time);
        if current_time - self.last_time >= self.delta_time {
            // --- EJECUTA TU TAREA AQUÍ ---
            self.set_status_text(&format!("⌚: {}", current_time), Color32::GREEN);
            self.tasks.emit(current_time);

            // println!("Tarea ejecutada!");
            self.last_time = current_time;
        }
    }

    pub fn remove_repellers_at_point(&mut self, wx: f32, wy: f32) {
        let radius_at_size_one = self.pos2_to_world(pos2(1.0, 0.0)).x;
        self.repellers.retain(|r| {
            let point = pos2(wx, wy);
            let center = r.position;
            let rw = 0.00005;
            // let rw = r.wsize();
            // let rw = r.size * radius_at_size_one;
            let pradius = point.distance_sq(center);
            // dbg!(rw);
            // dbg!(pradius);
            let point_inside_repeller = pradius <= rw;

            // dbg!(point, center);
            //dbg!(point_inside_repeller, rw, r.size, radius_at_size_one);
            !point_inside_repeller
        });
    }
}

// ╔════════╗
// ║ Worlds ║
// ╚════════╝
// -- : -------------------------------------------------------------------
#[derive(Clone)]
pub struct Worlds {
    pub world_rect: Rect,
    pub screen_rect: Rect,
    pub w2s: emath::RectTransform,
    pub s2w: emath::RectTransform,
}

impl Worlds {
    fn new(wr: Rect, sr: Rect) -> Self {
        let w2s = emath::RectTransform::from_to(wr, sr);
        let s2w = w2s.inverse();

        Self {
            world_rect: wr,
            screen_rect: sr,
            w2s,
            s2w,
        }
    }

    pub fn update_screen_rect(&mut self, screen_rect: Rect) {
        //println!("Update worlds sr & transforms");

        // Store the canvas rect
        self.screen_rect = screen_rect;

        // Compute world2screen and screen2world transforms
        self.w2s = emath::RectTransform::from_to(self.world_rect, self.screen_rect);
        self.s2w = self.w2s.inverse();
    }

    pub fn pos2_to_screen(&self, pos: Pos2) -> Pos2 {
        // if !self.world_rect.contains(pos) {
        //     println!("pos: {:?} out of wr", pos);
        // }
        // Check that point x,y is inside its world_rect.
        // assert!(self.world_rect.contains(pos));
        self.w2s.transform_pos_clamped(pos)
    }

    pub fn pos2_to_world(&self, pos: Pos2) -> Pos2 {
        // Check that point x,y is inside its screen_rect.
        //assert!(self.screen_rect.contains(pos));
        self.s2w.transform_pos_clamped(pos)
    }

    pub fn rect_to_screen(&self, rect: Rect) -> Rect {
        // Check that 'rect' is inside its world_rect.
        // assert!(self.world_rect.contains(rect.min));
        // assert!(self.world_rect.contains(rect.max));
        self.w2s.transform_rect(rect)
    }

    pub fn rect_to_world(&self, rect: Rect) -> Rect {
        // Check that 'rect' is inside its screen_rect.
        // assert!(self.screen_rect.contains(rect.min));
        // assert!(self.screen_rect.contains(rect.max));
        self.s2w.transform_rect(rect)
    }
}
