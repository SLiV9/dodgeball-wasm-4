/**/

use crate::palette;
use crate::sprites;
use crate::wasm4::*;

const PADDING_HEIGHT: u32 = 20;

pub struct Level
{
	little_guy: LittleGuy,
	score: i32,
	ticks: i32,
}

impl Level
{
	pub fn new() -> Self
	{
		Self {
			little_guy: LittleGuy::new(),
			score: 0,
			ticks: 0,
		}
	}

	pub fn update(&mut self)
	{
		self.little_guy.update();

		self.ticks += 1;
	}

	pub fn draw(&mut self)
	{
		unsafe {
			*PALETTE = palette::SODACAP_PALETTE;
		}

		unsafe {
			*DRAW_COLORS = 4;
		}

		let seconds = self.ticks / 60;
		let frac = (self.ticks / 6) % 10;
		let score = self.score;
		text(format!("TM: {:>3}.{}", seconds, frac), 5, 8);
		text(format!("PTS: {:>3}", score), 90, 8);

		unsafe { *DRAW_COLORS = 0x40 }
		rect(
			5,
			(PADDING_HEIGHT as i32) + 5,
			SCREEN_SIZE - 10,
			SCREEN_SIZE - PADDING_HEIGHT - 10,
		);

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
			x: 80,
			y: 120,
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
			if self.x < 5 + 5
				|| self.x > (SCREEN_SIZE as i32) - 5 - 5
				|| self.y < (PADDING_HEIGHT as i32) + 5 + 3
				|| self.y > (SCREEN_SIZE as i32) - 5 - 3
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
