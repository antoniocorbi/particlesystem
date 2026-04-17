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

// -- Uses: ---------------------------------------------------------------
use crate::constants::{self, CANVAS_W, MIN_GRAY, REP_SIZE};
use crate::{
    gui::psystemappui::PSystemAppUi,
    model::{particle::Particle, particlesystem::ParticleSystem, repeller::Repeller},
};
use egui::{RichText, Style};
//use delegate::delegate;
use egui::{
    emath::{self, RectTransform},
    epaint::Hsva,
    pos2, Color32, CornerRadius, Frame, PointerButton, Pos2, Rect, Sense, Stroke, Ui, Vec2,
};

// ╔═══════╗
// ║ AppUi ║
// ╚═══════╝
trait AppUi {
    fn create_drawing_widget(&mut self, ui: &mut Ui);
    fn draw_grid(&mut self, painter: &egui::Painter, rect: egui::Rect);
    fn draw_particle(&self, p: &Particle, painter: &egui::Painter);
    fn draw_repeller(&self, r: &Repeller, painter: &egui::Painter);
    fn draw_particle_system(&self, p: &ParticleSystem, painter: &egui::Painter);

    // fn create_stroke_widget(&mut self, ui: &mut Ui) -> egui::Response;
    // fn draw_point(&self, p: Point2D, color: Color32, zoom: f32, painter: &egui::Painter);
    // fn draw_point_sq(&self, p: Point2D, color: Color32, zoom: f32, painter: &egui::Painter);
    // fn draw_lines(&self, lines: &Vec<Pos2>, color: Color32, painter: &egui::Painter);
}

// ╔══════════╗
// ║ Impl For ║
// ╚══════════╝

impl AppUi for PSystemAppUi {
    fn create_drawing_widget(&mut self, ui: &mut Ui) {
        let height_for_widgets = 40.0; // Espacio que necesitas abajo
        let scroll_height = ui.available_height() - height_for_widgets;

        egui::ScrollArea::both() // Habilita scroll horizontal y vertical
            .auto_shrink([false; 2]) // Evita que el área se colapse si hay poco contenido
            .max_height(scroll_height) // Limitamos la altura del scroll
            .show(ui, |ui| {
                // 1. Definimos el tamaño total de nuestro "papel" o lienzo
                let canvas_size = egui::vec2(constants::CANVAS_W, constants::CANVAS_H);
                Frame::canvas(ui.style())
                    // .corner_radius(5.0)
                    .fill(Color32::from_rgb(20, 60, 100)) // Fondo azul
                    // .stroke(Stroke::new(1.5, Color32::LIGHT_RED)) // Borde negro
                    .show(ui, |ui| {
                        // self.ui_canvas(ui); // Llamamos a la lógica de dibujo

                        let (response, painter) = ui.allocate_painter(
                            // ui.available_size() - [2.0, 45.0].into(),
                            canvas_size,
                            Sense::drag() | Sense::click(), // ¡Importante! Detectar arrastre (clic + movimiento)
                                                            //Sense::DRAG | Sense::CLICK,
                        );

                        // If there are no worlds object defined,
                        // define it, else update screen_rect
                        self.init_worlds(response.rect);

                        // Draw the GRID
                        if self.grid {
                            self.draw_grid(&painter, response.rect);
                        }

                        // 1. Comprobamos el click izquierdo
                        if response.secondary_clicked() {
                            //println!("¡Click derecho detectado en el Painter!");
                            if let Some(pos) = response.interact_pointer_pos() {
                                // println!("Añadir Repeller!: Click en la posición: {:?}", pos);
                                let wpos = self.worlds.as_ref().unwrap().pos2_to_world(pos);
                                let wx = wpos.x;
                                let wy = wpos.y;
                                let repeller = Repeller::new(
                                    wx,
                                    wy,
                                    constants::REP_POWER,
                                    constants::REP_SIZE,
                                );
                                self.repellers.push(repeller);
                            }
                        }

                        // 2. Comprobamos el click intermedio
                        if response.middle_clicked() {
                            //println!("¡Click central detectado en el Painter!");
                            if let Some(pos) = response.interact_pointer_pos() {
                                println!("Click en la posición: {:?}", pos);
                            }
                            //self.show_data();
                        }

                        // 3. Comprobamos el click izquierdo
                        if response.clicked() {
                            //println!("¡Click izquierdo detectado en el Painter!");
                            if let Some(pos) = response.interact_pointer_pos() {
                                let wpos = self.worlds.as_ref().unwrap().pos2_to_world(pos);
                                let wx = wpos.x;
                                let wy = wpos.y;

                                self.set_status_text(
                                    &format!(
                                        //"sx: {} , sy: {} , wx: {} , wy: {}",
                                        "particle system @ [{:.2},{:.2}]",
                                        wx, wy
                                    ),
                                    egui::Color32::LIGHT_RED,
                                );

                                let wr = Rect::from_min_max(
                                    constants::WR_MIN.into(),
                                    constants::WR_MAX.into(),
                                );
                                let ps = ParticleSystem::new(
                                    constants::NPARTICLES,
                                    wx,
                                    wy,
                                    wr,
                                    self.particle_size,
                                    self.particle_mass,
                                );
                                self.psystems.push(ps);
                            }
                        }

                        if response.dragged_by(PointerButton::Primary) {
                            // Obtenemos la posición actual del puntero
                            if let Some(pos) = response.interact_pointer_pos() {
                                let wpos = self.worlds.as_ref().unwrap().pos2_to_world(pos);
                                let wx = wpos.x;
                                let wy = wpos.y;
                                let ctx = ui.ctx();
                                ctx.send_viewport_cmd(egui::ViewportCommand::CursorVisible(false));

                                // Dibujamos un círculo donde esté el ratón mientras arrastramos
                                //  painter.circle_filled(pos, 2.0, Color32::LIGHT_RED);

                                // También puedes obtener cuánto se ha movido desde el frame anterior
                                // let delta = response.drag_delta();
                                // println!("Moviendo: {:?}", delta);

                                if self.psystems.len() == 0 {
                                    // First psystem, create it
                                    let wr = Rect::from_min_max(
                                        constants::WR_MIN.into(),
                                        constants::WR_MAX.into(),
                                    );
                                    let ps = ParticleSystem::new(
                                        constants::NPARTICLES,
                                        wx,
                                        wy,
                                        wr,
                                        self.particle_size,
                                        self.particle_mass,
                                    );
                                    self.psystems.push(ps);
                                } else {
                                    // Add new particle to last ParticleSystem
                                    let p = Particle::new(wx, wy, self.particle_size);
                                    //dbg!(p);
                                    let ps = self.psystems.last_mut().unwrap();
                                    ps.add_particle(&p);
                                }
                            }
                        }

                        if response.drag_stopped_by(PointerButton::Primary) {
                            let ctx = ui.ctx();
                            // Útil si el usuario estaba arrastrando y soltó el botón
                            ctx.send_viewport_cmd(egui::ViewportCommand::CursorVisible(true));
                        }

                        // Update particles status
                        self.run();

                        // Draw repeller
                        for r in &self.repellers {
                            self.draw_repeller(r, &painter);
                        }
                        //if self.repeller.is_some() {
                        //    self.draw_repeller(self.repeller.as_ref().unwrap(), &painter);
                        //}

                        // Draw particles
                        // println!("N-psystems: {}", self.psystems.len());
                        for ps in &self.psystems {
                            //println!("Drawing psystem");
                            self.draw_particle_system(ps, &painter);
                        }
                    });
            });

        // let (response, painter) = ui.allocate_painter(
        //     Vec2::new(ui.available_width(), ui.available_height() - 50.0),
        //     Sense::DRAG | Sense::CLICK,
        // );
    }

    fn draw_particle(&self, p: &Particle, painter: &egui::Painter) {
        if !p.is_dead() {
            let center = self.pos2_to_screen(p.position);
            // let grayidx = if p.lifespan > (255 - constants::MIN_GRAY) {
            //     constants::MIN_GRAY
            // } else {
            //     255 - p.lifespan
            // };

            // Younger particles: white -> Older particles: black
            let grayidx = p.lifespan;
            let color = Color32::from_gray(grayidx);

            painter.circle_filled(center, p.size, color);
        }
    }

    fn draw_repeller(&self, r: &Repeller, painter: &egui::Painter) {
        let center = self.pos2_to_screen(r.position);
        let color = Color32::RED;
        let red = (r.power * constants::REP_POWER_DIV * 2.5).clamp(0.0, 255.0) as u8;
        let g = Color32::RED.g();
        let b = Color32::RED.b();
        let color = Color32::from_rgb(dbg!(red), g, b);

        painter.circle_filled(center, r.size, color);
    }

    fn draw_particle_system(&self, ps: &ParticleSystem, painter: &egui::Painter) {
        //println!("n-particles in psystem: {}", ps.len());
        for i in 0..ps.len() {
            //println!("Drawing particles {}: ", i);
            self.draw_particle(&ps[i], painter);
        }
    }

    fn draw_grid(&mut self, painter: &egui::Painter, rect: egui::Rect) {
        // 1. Gestionar grid-spacing muy pequeño
        let grid_spacing = self.grid_size * 3.0; // Distancia entre líneas
        let grid_stroke = egui::Stroke::new(0.5, egui::Color32::LIGHT_BLUE); // Líneas finas y claras

        // Líneas verticales
        let mut x = rect.left() + (grid_spacing - rect.left() % grid_spacing);
        while x < rect.right() {
            painter.line_segment(
                [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                grid_stroke,
            );
            x += grid_spacing;
        }

        // Líneas horizontales
        let mut y = rect.top() + (grid_spacing - rect.top() % grid_spacing);
        while y < rect.bottom() {
            painter.line_segment(
                [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                grid_stroke,
            );
            y += grid_spacing;
        }
    }
}

impl eframe::App for PSystemAppUi {
    /// Called by the framework to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.set_zoom_factor(constants::UI_ZOOM);

        self.run_tasks(ui);

        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // 1. Definimos la barra de abajo primero
        egui::Panel::bottom("status_bar").show_inside(ui, |ui| {
            let richtxt = RichText::new(&self.status_text)
                .color(self.status_text_color)
                .small_raised();
            ui.label(richtxt);
        });

        egui::Panel::top("top_panel")
            //.max_size(120.0)
            //.default_size(120.0)
            .show_inside(ui, |ui| {
                // Reducimos el espaciado vertical a 2.0 píxeles (por ejemplo)
                // El primer valor es horizontal, el segundo vertical.
                //ui.spacing_mut().item_spacing = egui::vec2(2.0, 2.0);

                // The top panel is often a good place for a menu bar:
                egui::MenuBar::new().ui(ui, |ui| {
                    // NOTE: no File->Quit on web pages!
                    let is_web = cfg!(target_arch = "wasm32");
                    if !is_web {
                        ui.menu_button("File", |ui| {
                            if ui.button("Quit").clicked() {
                                ui.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        });
                        //ui.add_space(16.0);
                    }

                    // ui.separator();
                    egui::widgets::global_theme_preference_buttons(ui);
                });
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(
                    egui::RichText::new(" · Particle System · ")
                        .color(Color32::GOLD)
                        .background_color(Color32::DARK_GRAY),
                );
            });

            // egui::ScrollArea::vertical().show(ui, |ui| {
            //     lorem_ipsum(ui);
            // });

            ui.horizontal(|ui| {
                // 1. Alive particles
                let txt = egui::RichText::new(format!(" Alive ⮩ particles: ")).color(Color32::RED);
                ui.label(txt);
                ui.label(
                    egui::RichText::new(format!("{}", self.nparticles())).color(Color32::YELLOW),
                );

                // 2. Alive particle systems
                let txt =
                    egui::RichText::new(format!(" and ⮩ Particle systems: ")).color(Color32::RED);
                ui.label(txt);
                ui.label(
                    egui::RichText::new(format!("{}", self.psystems.len())).color(Color32::YELLOW),
                );
            });

            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.grid_size, 1.0..=10.0).text("Grid size"));

                ui.checkbox(&mut self.grid, "Show grid")
                    .on_hover_text("Activate / deactivate the grid.");
            });

            ui.horizontal(|ui| {
                ui.add(
                    egui::Slider::new(&mut self.particle_size, 1.0..=constants::MAX_PSIZE)
                        .text("Max Particle size"),
                );
                ui.separator();
                ui.add(
                    egui::Slider::new(&mut self.particle_mass, 1.0..=constants::MAX_PMASS)
                        .text("Max Particle mass"),
                );
            });

            self.create_drawing_widget(ui);
        });

        ui.request_repaint();
    }
}

fn lorem_ipsum(ui: &mut egui::Ui) {
    ui.with_layout(
        egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
        |ui| {
            ui.label(
                egui::RichText::new(constants::LOREM_IPSUM_LONG)
                    .small()
                    .weak(),
            );
            ui.add(egui::Separator::default().grow(8.0));
            ui.label(
                egui::RichText::new(constants::LOREM_IPSUM_LONG)
                    .small()
                    .weak(),
            );
        },
    );
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
