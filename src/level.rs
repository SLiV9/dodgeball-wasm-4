/**/

use crate::palette;
use crate::sprites;
use crate::wasm4::*;

pub struct Level
{
	little_guy: LittleGuy,
}

impl Level
{
	pub fn new() -> Self
	{
		Self {
			little_guy: LittleGuy::new(),
		}
	}

	pub fn update(&mut self)
	{
		self.little_guy.update();
	}

	pub fn draw(&mut self)
	{
		unsafe {
			*PALETTE = palette::SODACAP_PALETTE;
		}

		unsafe { *DRAW_COLORS = 4 }
		text("WOOHOO", 10, 10);

		self.little_guy.draw();
	}
}

struct LittleGuy
{
	x: i32,
	y: i32,
	sprite: sprites::little_guy::Animation,
	is_dead: bool,
}

impl LittleGuy
{
	pub fn new() -> Self
	{
		Self {
			x: 10,
			y: 20,
			sprite: sprites::little_guy::Animation::new(),
			is_dead: false,
		}
	}

	pub fn update(&mut self)
	{
		self.sprite.tick();

		let gamepad = unsafe { *GAMEPAD1 };
		let left = gamepad & BUTTON_LEFT != 0;
		let right = gamepad & BUTTON_RIGHT != 0;
		let up = gamepad & BUTTON_UP != 0;
		let down = gamepad & BUTTON_DOWN != 0;
		let speed = 1;

		if self.is_dead
		{
			// Nothing
		}
		else if left && !right
		{
			self.sprite.run_left();
			self.x -= speed;
		}
		else if right && !left
		{
			self.sprite.run_right();
			self.x += speed;
		}
		else if up && !down
		{
			self.sprite.run_up();
			self.y -= speed;
		}
		else if down && !up
		{
			self.sprite.run_down();
			self.y += speed;
		}
		else
		{
			self.sprite.idle();
		}

		if !self.is_dead
		{
			if self.x < 10
				|| self.x > (SCREEN_SIZE as i32) - 10
				|| self.y < 10 || self.y > (SCREEN_SIZE as i32) - 10
			{
				self.is_dead = true;
				self.sprite.die();
			}
		}
	}

	pub fn draw(&self)
	{
		self.sprite.draw(self.x, self.y);
	}
}
