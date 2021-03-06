//
// Part of dodgeball-wasm-4
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::palette;
use crate::wasm4::*;

pub struct Menu
{
	rng_seed: u64,
}

impl Menu
{
	pub const fn new() -> Self
	{
		Self { rng_seed: 0 }
	}

	pub fn update(&mut self) -> Option<Outcome>
	{
		let gamepad = unsafe { *GAMEPAD1 };

		self.rng_seed += 1;

		if gamepad & BUTTON_1 != 0
		{
			Some(Outcome::Start {
				rng_seed: self.rng_seed,
			})
		}
		else
		{
			None
		}
	}

	pub fn draw(&mut self)
	{
		unsafe {
			*PALETTE = palette::SODACAP_PALETTE;
		}

		unsafe { *DRAW_COLORS = 4 }
		text("DODGEBALL", 10, 10);

		text("PRESS X TO START", 10, 140);
	}
}

pub enum Outcome
{
	Start
	{
		rng_seed: u64
	},
}
