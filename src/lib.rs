#![warn(clippy::all, rust_2018_idioms)]

mod constants;
mod gui;
mod model;

pub use gui::psystemappui;
pub use model::{particle::Particle, particlesystem::ParticleSystem};
