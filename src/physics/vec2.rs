use std::{
	fmt,
	ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32,
}

impl Vec2 {
	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}
}

impl fmt::Display for Vec2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

impl Default for Vec2 {
	fn default() -> Self {
		Self { x: 0.0, y: 0.0 }
	}
}

impl From<(f32, f32)> for Vec2 {
	fn from(tup: (f32, f32)) -> Self {
		Self { x: tup.0, y: tup.1 }
	}
}

impl Add for Vec2 {
	type Output = Vec2;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl Add<f32> for Vec2 {
	type Output = Vec2;

	fn add(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x + rhs,
			y: self.y + rhs,
		}
	}
}

impl AddAssign for Vec2 {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl AddAssign<f32> for Vec2 {
	fn add_assign(&mut self, rhs: f32) {
		self.x += rhs;
		self.y += rhs;
	}
}

impl Sub for Vec2 {
	type Output = Vec2;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl Sub<f32> for Vec2 {
	type Output = Vec2;

	fn sub(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x - rhs,
			y: self.y - rhs,
		}
	}
}

impl SubAssign for Vec2 {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl SubAssign<f32> for Vec2 {
	fn sub_assign(&mut self, rhs: f32) {
		self.x -= rhs;
		self.y -= rhs;
	}
}

impl Mul for Vec2 {
	type Output = Vec2;

	fn mul(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		}
	}
}

impl Mul<f32> for Vec2 {
	type Output = Vec2;

	fn mul(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl MulAssign for Vec2 {
	fn mul_assign(&mut self, rhs: Self) {
		self.x *= rhs.x;
		self.y *= rhs.y;
	}
}

impl MulAssign<f32> for Vec2 {
	fn mul_assign(&mut self, rhs: f32) {
		self.x *= rhs;
		self.y *= rhs;
	}
}
