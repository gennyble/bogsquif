use std::borrow::BorrowMut;

use crate::physics::Vec2;

use super::AxisBox;

pub struct Fixed {
	pub position: Vec2,
	pub size: Vec2,
	pub friction: f32,
}

impl Fixed {
	pub fn new<P: Into<Vec2>, S: Into<Vec2>>(position: P, size: S) -> Self {
		Self {
			position: position.into(),
			size: size.into(),
			friction: 0.1,
		}
	}
}

impl AxisBox for Fixed {
	fn position(&self) -> Vec2 {
		self.position
	}

	fn size(&self) -> Vec2 {
		self.size
	}

	fn friction(&self) -> f32 {
		self.friction
	}
}
