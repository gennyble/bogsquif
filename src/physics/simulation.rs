use std::{
	borrow::BorrowMut,
	cell::{Cell, RefCell},
	time::Duration,
};

use super::{
	objects::{AxisBox, Fixed, Kinetic, Object},
	Tick, Vec2,
};

pub struct Simulation {
	pub world_bounds: Vec2,
	pub gravity: Vec2,
	pub kinetic: Vec<RefCell<Kinetic>>,
	pub fixed: Vec<Fixed>,
}

impl Simulation {
	pub fn new<B: Into<Vec2>>(world_bounds: B) -> Self {
		Self {
			world_bounds: world_bounds.into(),
			gravity: Vec2::new(0.0, 450.0),
			kinetic: vec![],
			fixed: vec![],
		}
	}

	pub fn add_object<T: Into<Object>>(&mut self, object: T) {
		match object.into() {
			Object::Kinetic(k) => self.kinetic.push(RefCell::new(k)),
			Object::Fixed(f) => self.fixed.push(f),
		}
	}

	pub fn kinetic_world(kinetic: &mut Kinetic, world: Vec2) {
		if kinetic.top() < 0.0 {
			kinetic.velocity.y *= -kinetic.restitution;
			kinetic.position.y = 0.0;
		} else if kinetic.bottom() > world.y {
			kinetic.velocity.y *= -kinetic.restitution;
			kinetic.position.y = world.y - kinetic.size.y;
		}

		if kinetic.left() < 0.0 {
			kinetic.velocity.x *= -kinetic.restitution;
			kinetic.position.x = 0.0;
		} else if kinetic.right() > world.x {
			kinetic.velocity.x *= -kinetic.restitution;
			kinetic.position.x = world.x - kinetic.size.x;
		}
	}

	pub fn kinetic_fixed(kin: &mut Kinetic, fix: &Fixed) {
		// Is there a seperating axis? We're fixed, so no weird projections..
		let vertical_is_seperate = fix.top() > kin.bottom() || fix.bottom() < kin.top();
		let horizontal_is_seperate = fix.left() > kin.right() || fix.right() < kin.left();

		if vertical_is_seperate || horizontal_is_seperate {
			return;
		}

		// Seperations: negative if touching
		let top = fix.top() - kin.bottom();
		let bottom = kin.top() - fix.bottom();
		let left = fix.left() - kin.right();
		let right = kin.left() - fix.right();
		let friction = kin.friction().max(fix.friction());

		// Where's the smallest? Everything inverted.
		if top > bottom && top > left && top > right {
			kin.position.y = fix.top() - kin.size.y;
			kin.velocity.y *= -kin.restitution;
			kin.velocity.x *= 1.0 - friction;
			kin.ground_contact = true;
		} else if bottom > top && bottom > left && bottom > right {
			kin.position.y = fix.bottom();
			kin.velocity.y *= -kin.restitution;
			kin.velocity.x *= 1.0 - friction;
		} else if left > bottom && left > top && left > right {
			kin.position.x = fix.left() - kin.size.x;
			kin.velocity.x *= -kin.restitution;
			kin.velocity.y *= 1.0 - friction;
		} else if right > bottom && right > top && right > left {
			kin.position.x = fix.right();
			kin.velocity.x *= -kin.restitution;
			kin.velocity.y *= 1.0 - friction;
		}
	}

	pub fn kinetic_kinetic(kin_a: &mut Kinetic, kin_b: &mut Kinetic) {
		// Is there a seperating axis? We're fixed, so no weird projections..
		let vertical_is_seperate = kin_a.top() > kin_b.bottom() || kin_a.bottom() < kin_b.top();
		let horizontal_is_seperate = kin_a.left() > kin_b.right() || kin_a.right() < kin_b.left();

		if vertical_is_seperate || horizontal_is_seperate {
			return;
		}

		// Seperations: negative if touching
		let top = kin_a.top() - kin_b.bottom(); // B hitting A from above
		let bottom = kin_b.top() - kin_a.bottom(); // B hitting A from below
		let left = kin_a.left() - kin_b.right(); // B to A from the left
		let right = kin_b.left() - kin_a.right(); // B to A from the right

		if top >= 0.0 && bottom >= 0.0 && left >= 0.0 && right >= 0.0 {
			println!("Touching, but not intersecting!");
			return;
		}

		let restitution = kin_b.restitution().min(kin_a.restitution());
		let friction = kin_b.friction().max(kin_a.friction());

		//TODO: Kinetic-Kinetic
	}
}

impl Tick for Simulation {
	fn tick(&mut self, since_last_frame: Duration) {
		for (kinetic_index, kinetic_cell) in self.kinetic.iter().enumerate() {
			let kinetic = &mut *kinetic_cell.borrow_mut();
			// Gravity
			if !kinetic.ground_contact {
				kinetic.velocity += self.gravity * since_last_frame.as_secs_f32();
			}

			let tenths = since_last_frame / 10;
			for tenth in 0..10 {
				kinetic.tick(tenths);

				for fixed in &self.fixed {
					Self::kinetic_fixed(kinetic, fixed);
				}

				for (kinetic_b_index, kinetic_b_cell) in self.kinetic.iter().enumerate() {
					if kinetic_index == kinetic_b_index {
						continue;
					}

					Self::kinetic_kinetic(kinetic, &mut *kinetic_b_cell.borrow_mut());
				}
			}

			Self::kinetic_world(kinetic, self.world_bounds);

			// No floor wiggly
			let wiggle = 2.5;
			if kinetic.velocity.x.abs() < wiggle {
				kinetic.velocity.x = 0.0;
			}
			if kinetic.velocity.y.abs() < wiggle {
				kinetic.velocity.y = 0.0;
			}
		}
	}
}
