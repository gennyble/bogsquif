use std::time::Duration;

use crate::physics::{Tick, Vec2};

use super::AxisBox;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Kinetic {
	pub position: Vec2,
	pub size: Vec2,
	/// Velocity in pixels/sec
	pub ground_contact: bool,
	pub velocity: Vec2,
	pub mass: f32,
	pub restitution: f32,
	pub friction: f32,
}

impl Kinetic {
	pub fn new<P: Into<Vec2>, S: Into<Vec2>>(position: P, size: S) -> Self {
		Self::with_velocity(position, size, (0.0, 0.0))
	}

	pub fn with_velocity<P: Into<Vec2>, S: Into<Vec2>, V: Into<Vec2>>(
		position: P,
		size: S,
		velocity: V,
	) -> Self {
		Self {
			position: position.into(),
			size: size.into(),
			ground_contact: false,
			velocity: velocity.into(),
			mass: 1.0,
			restitution: 1.0,
			friction: 0.0,
		}
	}

	pub fn restitution_mut(&mut self) -> &mut f32 {
		&mut self.restitution
	}

	pub fn mass_mut(&mut self) -> &mut f32 {
		&mut self.mass
	}

	pub fn smallen_velocity(&mut self, v: Vec2) {
		if self.velocity.x > 0.0 {
			self.velocity.x -= v.x;
		} else {
			self.velocity.x += v.x;
		}
		if self.velocity.y > 0.0 {
			self.velocity.y -= v.y;
		} else {
			self.velocity.y += v.y;
		}
	}
}

impl Tick for Kinetic {
	fn tick(&mut self, delta: Duration) {
		self.ground_contact = false;
		let movement = self.velocity * delta.as_secs_f32();
		self.position += movement;
	}
}

impl AxisBox for Kinetic {
	fn position(&self) -> Vec2 {
		self.position
	}

	fn size(&self) -> Vec2 {
		self.size
	}

	fn restitution(&self) -> f32 {
		self.restitution
	}

	fn friction(&self) -> f32 {
		self.friction
	}
}
