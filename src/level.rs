//
// Part of dodgeball-wasm-4
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::palette;
use crate::sprites;
use crate::wasm4::*;
use fastrand;

const BANNER_HEIGHT: u32 = 20;
const PADDING_SIZE: u32 = 10;

pub struct Level
{
	rng: fastrand::Rng,
	little_guy: LittleGuy,
	balls: Vec<Ball>,
	score: i32,
	ticks: i32,
	time_until_next_ball: i32,
	time_between_balls: i32,
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
			time_until_next_ball: 0,
			time_between_balls: 90,
		}
	}

	pub fn update(&mut self)
	{
		self.little_guy.update();
		for ball in &mut self.balls
		{
			ball.update();
		}

		let num_gone = self.balls.iter().filter(|ball| ball.is_gone).count();
		self.balls.retain(|ball| !ball.is_gone);

		if self.little_guy.is_alive()
		{
			for ball in &self.balls
			{
				if ball.detect_collision(self.little_guy.x, self.little_guy.y)
				{
					self.little_guy.kill();
					tone(250, 5 | (10 << 8), 100, TONE_NOISE);
					tone(10, 20 | (80 << 8), 100, TONE_PULSE1);
				}
			}

			self.ticks += 1;
			self.score += num_gone as i32;

			if self.time_until_next_ball <= 0
			{
				let (min_speed, min_bonus, max_bonus) = if self.ticks > 150 * 60
				{
					(5, 0, 0)
				}
				else if self.ticks > 120 * 60
				{
					(3, 0, 0)
				}
				else if self.ticks > 90 * 60
				{
					(2, 1, 1)
				}
				else if self.ticks > 75 * 60
				{
					(2, 0, 1)
				}
				else if self.ticks > 60 * 60
				{
					(2, 0, 0)
				}
				else if self.ticks > 30 * 60
				{
					(1, 1, 1)
				}
				else if self.ticks > 10 * 60
				{
					(1, 0, 1)
				}
				else
				{
					(1, 0, 0)
				};
				let bonus_speed = if self.rng.bool()
				{
					self.rng.i32(min_bonus..=max_bonus)
				}
				else
				{
					0
				};
				let warning_time =
					std::cmp::max(5, self.time_between_balls * 3 / 4);
				self.balls.push(Ball::new(
					min_speed,
					bonus_speed,
					warning_time,
					&mut self.rng,
				));
				self.time_until_next_ball =
					std::cmp::max(1, self.time_between_balls);
				if self.time_between_balls > 70
				{
					self.time_between_balls -= 2;
				}
				else if self.time_between_balls > 45
				{
					self.time_between_balls -= 1;
				}
				else if self.time_between_balls > 30 && self.rng.bool()
				{
					self.time_between_balls -= 1;
				}
				else if self.time_between_balls > 20
					&& self.rng.i32(0..4) == 0
				{
					self.time_between_balls -= 1;
				}
				else if self.time_between_balls > 10
					&& self.rng.i32(0..10) == 0
				{
					self.time_between_balls -= 1;
				}
				else if self.rng.i32(0..100) == 0
				{
					self.time_between_balls -= 1;
				}
			}
			else if self.time_until_next_ball > 0
			{
				self.time_until_next_ball -= 1;
			}
		}
		else if self.balls.is_empty()
		{
			let gamepad = unsafe { *GAMEPAD1 };
			if gamepad & BUTTON_1 != 0
			{
				self.restart();
			}
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

		if !self.little_guy.is_alive() && self.balls.is_empty()
		{
			text("PRESS X TO RESTART", 10, 140);
			return;
		}

		unsafe { *DRAW_COLORS = 0x40 }
		rect(
			PADDING_SIZE as i32,
			(BANNER_HEIGHT + PADDING_SIZE) as i32,
			SCREEN_SIZE - 2 * PADDING_SIZE,
			SCREEN_SIZE - BANNER_HEIGHT - 2 * PADDING_SIZE,
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

	fn restart(&mut self)
	{
		self.little_guy = LittleGuy::new();
		self.score = 0;
		self.ticks = 0;
		self.time_until_next_ball = 0;
		self.time_between_balls = 90;
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

		let is_cheating = (gamepad & BUTTON_2) != 0;
		if !self.is_dead && !is_cheating
		{
			if self.x < (PADDING_SIZE as i32) + 5
				|| self.x > (SCREEN_SIZE as i32) - (PADDING_SIZE as i32) - 5
				|| self.y < (BANNER_HEIGHT as i32) + (PADDING_SIZE as i32) + 3
				|| self.y > (SCREEN_SIZE as i32) - (PADDING_SIZE as i32) - 3
			{
				self.is_dead = true;
				self.sprite.die();
			}
		}
	}

	pub fn kill(&mut self)
	{
		self.is_dead = true;
		self.sprite.die();
	}

	pub fn is_alive(&self) -> bool
	{
		!self.is_dead
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
	warning_time: i32,
	time_between_warning_shots: i32,
	is_gone: bool,
}

impl Ball
{
	pub fn new(
		base_speed: i32,
		bonus_speed: i32,
		warning_time: i32,
		rng: &mut fastrand::Rng,
	) -> Self
	{
		let speed = base_speed + bonus_speed;
		let num_warning_shots = 2 + 2 * bonus_speed;
		let time_between_warning_shots =
			std::cmp::max(1, warning_time / num_warning_shots);

		let minx = (PADDING_SIZE as i32) + 5;
		let miny = (BANNER_HEIGHT as i32) + (PADDING_SIZE as i32) + 5;
		let maxx = (SCREEN_SIZE as i32) - (PADDING_SIZE as i32) - 5;
		let maxy = (SCREEN_SIZE as i32) - (PADDING_SIZE as i32) - 5;
		match rng.u8(0..4)
		{
			0 => Self {
				x: rng.i32(minx..=maxx),
				y: miny - 5 - 2,
				hspd: 0,
				vspd: speed,
				warning_time,
				time_between_warning_shots,
				is_gone: false,
			},
			1 => Self {
				x: maxx + 5 + 2,
				y: rng.i32(miny..=maxy),
				hspd: -speed,
				vspd: 0,
				warning_time,
				time_between_warning_shots,
				is_gone: false,
			},
			2 => Self {
				x: rng.i32(minx..=maxx),
				y: maxy + 5 + 2,
				hspd: 0,
				vspd: -speed,
				warning_time,
				time_between_warning_shots,
				is_gone: false,
			},
			3 => Self {
				x: minx - 5 - 2,
				y: rng.i32(miny..=maxy),
				hspd: speed,
				vspd: 0,
				warning_time,
				time_between_warning_shots,
				is_gone: false,
			},
			_ => unreachable!(),
		}
	}

	pub fn update(&mut self)
	{
		if self.warning_time > 0
		{
			let freq = (700 + self.hspd * 25 + self.vspd * 75) as u32;
			if (self.warning_time % self.time_between_warning_shots) == 0
			{
				tone(freq, 4 | (4 << 8), 30, TONE_TRIANGLE);
			}
			self.warning_time -= 1;
			if self.warning_time == 0
			{
				tone(freq, 12 | (4 << 8), 60, TONE_TRIANGLE);
			}
			return;
		}

		self.x += self.hspd;
		self.y += self.vspd;

		if (self.hspd < 0 && self.x < (PADDING_SIZE as i32))
			|| (self.hspd > 0
				&& self.x > (SCREEN_SIZE as i32) - (PADDING_SIZE as i32))
			|| (self.vspd < 0
				&& self.y < (BANNER_HEIGHT as i32) + (PADDING_SIZE as i32))
			|| (self.vspd > 0
				&& self.y > (SCREEN_SIZE as i32) - (PADDING_SIZE as i32))
		{
			self.is_gone = true;
		}
	}

	pub fn draw(&self)
	{
		if self.warning_time == 0
		{
			sprites::ball::draw(self.x, self.y);
		}
		else
		{
			if (self.warning_time % self.time_between_warning_shots) * 2
				>= self.time_between_warning_shots - 2
			{
				if self.hspd == 0
				{
					sprites::warning_horizontal::draw(self.x, self.y);
				}
				else
				{
					sprites::warning_vertical::draw(self.x, self.y);
				}
			}
		}
	}

	pub fn detect_collision(&self, x: i32, y: i32) -> bool
	{
		(self.x - x).abs() < 8 && (self.y - y).abs() < 3
	}
}
