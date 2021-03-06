//
// Part of dodgeball-wasm-4
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

mod wasm4;

#[cfg(feature = "buddy-alloc")]
mod alloc;

mod global_state;
mod level;
mod menu;
mod palette;
mod sprites;

use global_state::Wrapper;
use level::Level;
use menu::Menu;

static GAME: Wrapper<Game> = Wrapper::new(Game::Menu(Menu::new()));

#[no_mangle]
fn update()
{
	let game = GAME.get_mut();
	let outcome = match game
	{
		Game::Menu(menu) => menu.update(),
		Game::Level(level) =>
		{
			level.update();
			None
		}
	};
	match outcome
	{
		Some(menu::Outcome::Start { rng_seed }) =>
		{
			*game = Game::Level(Level::new(rng_seed));
		}
		None => (),
	}

	match game
	{
		Game::Menu(menu) => menu.draw(),
		Game::Level(level) => level.draw(),
	}
}

enum Game
{
	Menu(Menu),
	Level(Level),
}
