/**/

use crate::wasm4::*;

pub fn draw(x: i32, y: i32)
{
	unsafe {
		*DRAW_COLORS = 0x40;
	}
	blit(
		&WARNING_VERTICAL,
		x - (WARNING_VERTICAL_WIDTH as i32) / 2,
		y - (WARNING_VERTICAL_HEIGHT as i32) / 2,
		WARNING_VERTICAL_WIDTH,
		WARNING_VERTICAL_HEIGHT,
		WARNING_VERTICAL_FLAGS,
	);
}

// warning_vertical
const WARNING_VERTICAL_WIDTH: u32 = 16;
const WARNING_VERTICAL_HEIGHT: u32 = 16;
const WARNING_VERTICAL_FLAGS: u32 = 0; // BLIT_1BPP
const WARNING_VERTICAL: [u8; 32] = [
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x80, 0x01, 0x80,
	0x01, 0x80, 0x01, 0x80, 0x01, 0x80, 0x01, 0x80, 0x01, 0x80, 0x01, 0x80,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
