pub mod objects;
mod simulation;
mod vec2;

use std::time::Duration;

pub use simulation::Simulation;
pub use vec2::Vec2;

pub trait Tick {
	fn tick(&mut self, delta: Duration);
}
