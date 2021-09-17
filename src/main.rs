mod physics;

use std::{borrow::Borrow, fs::File, io::Write, time::Duration, vec};

use gifed::{
	block::{
		extension::{DisposalMethod, Extension, GraphicControl},
		ColorTable, Version,
	},
	writer::{GifBuilder, ImageBuilder},
	Color,
};

use physics::{
	objects::{Fixed, Kinetic},
	Simulation, Vec2,
};

use crate::physics::Tick;

fn main() {
	let mut sim = construct_kktest();
	let width = sim.world_bounds.x as u16;
	let height = sim.world_bounds.y as u16;

	let mut ctable = ColorTable::new();
	let make_fade = |ctable: &mut ColorTable, color: Color, length: u8| {
		for x in 0..length {
			let c = Color::new(
				(color.r as f32 * (x as f32 / length as f32)) as u8,
				(color.g as f32 * (x as f32 / length as f32)) as u8,
				(color.b as f32 * (x as f32 / length as f32)) as u8,
			);
			ctable.push(c);
		}
	};

	make_fade(&mut ctable, Color::new(0x88, 0x44, 0xDD), 64); // Purple
	make_fade(&mut ctable, Color::new(0x88, 0xDD, 0x44), 64); // Yellow
	make_fade(&mut ctable, Color::new(0x44, 0x88, 0xDD), 64); // Turquise
	ctable.push(Color::new(127, 127, 127));

	let mut gif = GifBuilder::new(Version::Gif89a, width, height)
		.global_color_table(ctable)
		.background_color_index(0);

	gif = gif.image(make_frame(&sim, &[63, 127, 191]));

	//let init_pos = sim.kinetic[0].position;
	//println!("Init: {}", init_pos);

	let step_size = 0.02;
	let fps = (1.0 / step_size) as usize;
	let frames = 1024;

	let fade_start = frames - fps / 2;

	for frame in 1..=frames {
		let fade_index = if frame > fade_start {
			let far = (frame - fade_start) as f32 / (fps / 2) as f32;
			((1.0 - far) * 63.0) as u8
		} else {
			63
		};

		let cindicies = &[fade_index, fade_index + 64, fade_index + 64 * 2];

		sim.tick(Duration::from_secs_f32(step_size));
		gif = gif
			.extension(Extension::GraphicControl(GraphicControl::new(
				DisposalMethod::Clear,
				false,
				false,
				(step_size * 100.0) as u16,
				0,
			)))
			.image(make_frame(&sim, cindicies));

		/*if sim.kinetic[0].position == init_pos {
			println!(
				"Looping! Frame {} - Time {}s",
				frame,
				step_size * frame as f32
			);
			gif = gif.extension(Extension::Looping(0));
			break;
		}*/
	}

	File::create("simulation.gif")
		.unwrap()
		.write_all(&gif.build().to_vec())
		.unwrap();
}

fn construct_kktest() -> Simulation {
	let bounds = Vec2::new(512.0, 256.0);
	let size = Vec2::new(32.0, 32.0);
	let center = (bounds * 0.5) + (size * 0.5);
	let offset = Vec2::new(128.0, 0.0);
	let velocity = Vec2::new(50.0, 0.0);

	let mut square = Kinetic::with_velocity(center - offset, size, velocity * 0.5);
	*square.mass_mut() = 10.0;

	let mut square2 = Kinetic::with_velocity(center - offset * 2.0, size, velocity * 3.0);
	*square2.mass_mut() = 10.0;

	let mut sim = Simulation::new(bounds);
	sim.gravity = Vec2::new(0.0, 0.0);

	sim.add_object(square);
	sim.add_object(square2);

	sim
}

fn construct_simulation() -> Simulation {
	let bounds = Vec2::new(256.0, 256.0);
	let size = Vec2::new(12.0, 12.0);
	let center = (bounds * 0.5) + (size * 0.5);

	let mut square = Kinetic::with_velocity((118.0, 152.0), size, (655.0, -47.0));
	*square.restitution_mut() = 0.75;
	*square.mass_mut() = 100.0;

	let mut square2 = Kinetic::with_velocity((32.0, 64.0), size, (75.0, -100.0));
	*square2.restitution_mut() = 0.9;
	*square2.mass_mut() = 50.0;

	let mut square3 = Kinetic::with_velocity((4.0, 224.0), size, (24.0, -250.0));
	*square3.restitution_mut() = 0.25;
	*square3.mass_mut() = 5.0;

	let mut sim = Simulation::new(bounds);

	sim.add_object(square);
	sim.add_object(square2);
	sim.add_object(square3);
	sim.add_object(Fixed::new((128.0, 192.0), (128.0, 24.0)));
	sim.add_object(Fixed::new((32.0, 192.0), (32.0, 64.0)));

	sim
}

fn make_frame(sim: &Simulation, cindicies: &[u8]) -> ImageBuilder {
	let width = sim.world_bounds.x as u16;
	let height = sim.world_bounds.y as u16;

	let mut image = vec![0; width as usize * height as usize];
	let ibuild = ImageBuilder::new(width, height);

	for fixed in &sim.fixed {
		let posx = fixed.position.x.round() as usize;
		let posy = fixed.position.y.round() as usize;

		for x in posx..posx + fixed.size.x as usize {
			for y in posy..posy + fixed.size.y as usize {
				let index = width as usize * y + x;
				image[index] = 192;
			}
		}
	}

	for (kindex, kinetic) in sim.kinetic.iter().enumerate() {
		let kinetic = kinetic.borrow();
		let posx = kinetic.position.x.round() as usize;
		let posy = kinetic.position.y.round() as usize;

		for x in posx..posx + kinetic.size.x as usize {
			for y in posy..posy + kinetic.size.y as usize {
				let index = width as usize * y + x;

				if index >= width as usize * height as usize {
					continue;
				}

				image[index] = cindicies[kindex];
			}
		}
	}

	ibuild.indicies(image)
}
