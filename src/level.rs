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

		for ball in &self.balls
		{
			if ball.detect_collision(self.little_guy.x, self.little_guy.y)
			{
				self.little_guy.kill();
			}
		}

		if self.little_guy.is_alive()
		{
			self.ticks += 1;
			self.score += num_gone as i32;

			if self.time_until_next_ball <= 0
			{
				let (min_speed, max_speed) = if self.ticks > 120 * 60
				{
					(3, 3)
				}
				else if self.ticks > 90 * 60
				{
					(2, 3)
				}
				else if self.ticks > 60 * 60
				{
					(2, 2)
				}
				else if self.ticks > 10 * 60
				{
					(1, 2)
				}
				else
				{
					(1, 1)
				};
				let speed = if self.rng.bool()
				{
					self.rng.i32(min_speed..=max_speed)
				}
				else
				{
					min_speed
				};
				let warning_time =
					std::cmp::max(5, self.time_between_balls * 3 / 4);
				self.balls.push(Ball::new(
					min_speed,
					speed - min_speed,
					warning_time,
					&mut self.rng,
				));
				self.time_until_next_ball =
					std::cmp::max(1, self.time_between_balls);
				if self.time_between_balls > 30 || self.rng.bool()
				{
					self.time_between_balls -= 1;
				}
			}
			else if self.time_until_next_ball > 0
			{
				self.time_until_next_ball -= 1;
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
				vspd: speed,
				warning_time,
				time_between_warning_shots,
				is_gone: false,
			},
			1 => Self {
				x: maxx + 3,
				y: rng.i32(miny..=maxy),
				hspd: -speed,
				vspd: 0,
				warning_time,
				time_between_warning_shots,
				is_gone: false,
			},
			2 => Self {
				x: rng.i32(minx..=maxx),
				y: maxy + 3,
				hspd: 0,
				vspd: -speed,
				warning_time,
				time_between_warning_shots,
				is_gone: false,
			},
			3 => Self {
				x: minx - 3,
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
		if self.warning_time > 0
		{
			return;
		}

		sprites::ball::draw(self.x, self.y);
	}

	pub fn detect_collision(&self, x: i32, y: i32) -> bool
	{
		(self.x - x).abs() < 8 && (self.y - y).abs() < 3
	}
}
