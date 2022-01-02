/**/

use crate::palette;
use crate::wasm4::*;

pub struct Level {}

impl Level
{
	pub const fn new() -> Self
	{
		Self {}
	}

	pub fn update(&mut self)
	{
		// TODO
	}

	pub fn draw(&mut self)
	{
		unsafe {
			*PALETTE = palette::SODACAP_PALETTE;
		}

		unsafe { *DRAW_COLORS = 4 }
		text("WOOHOO", 10, 10);
	}
}
