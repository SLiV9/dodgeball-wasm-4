/**/

use crate::wasm4::*;

pub fn draw(x: i32, y: i32)
{
	unsafe {
		*DRAW_COLORS = 0x40;
	}
	blit(
		&WARNING_HORIZONTAL,
		x - (WARNING_HORIZONTAL_WIDTH as i32) / 2,
		y - (WARNING_HORIZONTAL_HEIGHT as i32) / 2,
		WARNING_HORIZONTAL_WIDTH,
		WARNING_HORIZONTAL_HEIGHT,
		WARNING_HORIZONTAL_FLAGS,
	);
}

// warning_horizontal
const WARNING_HORIZONTAL_WIDTH: u32 = 16;
const WARNING_HORIZONTAL_HEIGHT: u32 = 16;
const WARNING_HORIZONTAL_FLAGS: u32 = 0; // BLIT_1BPP
const WARNING_HORIZONTAL: [u8; 32] = [
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
