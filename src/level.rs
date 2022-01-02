/**/

use crate::palette;
use crate::sprites;
use crate::wasm4::*;
use fastrand;

const PADDING_HEIGHT: u32 = 20;

pub struct Level
{
	rng: fastrand::Rng,
	little_guy: LittleGuy,
	balls: Vec<Ball>,
	score: i32,
	ticks: i32,
}

impl Level
{
	pub fn new(rng_seed: u64) -> Self
	{
		Self {
			rng: fastrand::Rng::with_seed(rng_seed),
			little_guy: LittleGuy::new(),
			balls: Vec::new(),
			score: 0,
			ticks: 0,
		}
	}

	pub fn update(&mut self)
	{
		self.little_guy.update();
		for ball in &mut self.balls
		{
			ball.update();
		}
		self.balls.retain(|ball| !ball.is_gone);

		self.ticks += 1;

		let max_balls: usize = 1 + ((self.ticks as usize) / 600);
		if (self.ticks % 60) == 0 && self.balls.len() < max_balls
		{
			self.balls.push(Ball::new(&mut self.rng));
		}
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
		text(format!("TM: {:>3}.{}", seconds, frac), 5, 5);
		text(format!("PTS: {:>3}", score), 90, 5);

		unsafe { *DRAW_COLORS = 0x40 }
		rect(
			5,
			(PADDING_HEIGHT as i32) + 5,
			SCREEN_SIZE - 10,
			SCREEN_SIZE - PADDING_HEIGHT - 10,
		);

		for ball in &self.balls
		{
			if ball.y < self.little_guy.y
			{
				ball.draw();
			}
		}
		self.little_guy.draw();
		for ball in &self.balls
		{
			if ball.y >= self.little_guy.y
			{
				ball.draw();
			}
		}
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

struct Ball
{
	x: i32,
	y: i32,
	hspd: i32,
	vspd: i32,
	is_gone: bool,
}

impl Ball
{
	pub fn new(rng: &mut fastrand::Rng) -> Self
	{
		let minx = 5 + 3;
		let miny = (PADDING_HEIGHT as i32) + 5 + 3;
		let maxx = (SCREEN_SIZE as i32) - 5 - 3;
		let maxy = (SCREEN_SIZE as i32) - 5 - 3;
		match rng.u8(0..4)
		{
			0 => Self {
				x: rng.i32(minx..=maxx),
				y: miny - 3,
				hspd: 0,
				vspd: 1,
				is_gone: false,
			},
			1 => Self {
				x: maxx + 3,
				y: rng.i32(miny..=maxy),
				hspd: -1,
				vspd: 0,
				is_gone: false,
			},
			2 => Self {
				x: rng.i32(minx..=maxx),
				y: maxy + 3,
				hspd: 0,
				vspd: -1,
				is_gone: false,
			},
			3 => Self {
				x: minx - 3,
				y: rng.i32(miny..=maxy),
				hspd: 1,
				vspd: 0,
				is_gone: false,
			},
			_ => unreachable!(),
		}
	}

	pub fn update(&mut self)
	{
		self.x += self.hspd;
		self.y += self.vspd;

		if (self.hspd < 0 && self.x < 5)
			|| (self.hspd > 0 && self.x > (SCREEN_SIZE as i32) - 5)
			|| (self.vspd < 0 && self.y < (PADDING_HEIGHT as i32) + 5)
			|| (self.vspd > 0 && self.y > (SCREEN_SIZE as i32) - 5)
		{
			self.is_gone = true;
		}
	}

	pub fn draw(&self)
	{
		sprites::ball::draw(self.x, self.y);
	}
}
