mod fixed;
mod kinetic;

pub use fixed::Fixed;
pub use kinetic::Kinetic;

use super::Vec2;

pub trait AxisBox {
	fn position(&self) -> Vec2;
	fn size(&self) -> Vec2;

	fn top(&self) -> f32 {
		self.position().y
	}

	fn left(&self) -> f32 {
		self.position().x
	}

	fn bottom(&self) -> f32 {
		self.position().y + self.size().y
	}

	fn right(&self) -> f32 {
		self.position().x + self.size().x
	}

	fn center(&self) -> Vec2 {
		self.position() + self.size() * 0.5
	}

	fn restitution(&self) -> f32 {
		1.0
	}

	fn friction(&self) -> f32 {
		0.0
	}
}

pub enum Object {
	Kinetic(Kinetic),
	Fixed(Fixed),
}

impl From<Kinetic> for Object {
	fn from(k: Kinetic) -> Self {
		Self::Kinetic(k)
	}
}

impl From<Fixed> for Object {
	fn from(k: Fixed) -> Self {
		Self::Fixed(k)
	}
}
