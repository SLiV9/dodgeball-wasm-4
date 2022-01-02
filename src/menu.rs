/**/

use crate::palette;
use crate::wasm4::*;

pub struct Menu {}

impl Menu
{
	pub const fn new() -> Self
	{
		Self {}
	}

	pub fn update(&mut self) -> Option<Outcome>
	{
		let gamepad = unsafe { *GAMEPAD1 };

		if gamepad & BUTTON_1 != 0
		{
			Some(Outcome::Start)
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
	Start,
}
