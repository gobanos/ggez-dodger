use actions::{Action, GameAction, MoveDirection, PlayerAction};
use baddies::Baddie;
use constants::*;
use ggez::{Context, GameResult, graphics, timer};
use ggez::event::{Axis, Button, EventHandler, Keycode, Mod, MouseButton, MouseState};
use ggez::graphics::Point2;
use player::Player;
use resources::Resources;
use std::collections::HashMap;

pub struct MainState {
	player: Player,
	actions: Vec<Action>,
	timer: u32,
	baddies: Vec<Baddie>,
	resources: Resources,
	paused: bool,
	input_stack: HashMap<MoveDirection, u32>,
}

impl MainState {
	pub fn new(ctx: &mut Context) -> GameResult<MainState> {
		let s = MainState {
			player: Player::new(Point2::new(WIDTH / 2.0, MAX_Y)),
			actions: Vec::new(),
			baddies: Vec::new(),
			timer: 0,
			resources: Resources::new(ctx)?,
			paused: false,
			input_stack: HashMap::with_capacity(2),
		};
		Ok(s)
	}

	fn add_action<A: Into<Action>>(&mut self, action: A) {
		self.actions.push(action.into());
	}

	fn process_actions(&mut self, ctx: &mut Context) -> GameResult<()> {
		use self::Action::*;
		use self::GameAction::*;

		for &action in &self.actions {
			match action {
				Game(Pause) => self.paused = !self.paused,
				Game(Quit) => ctx.quit()?,
				Player(_) if self.paused => (),
				Player(a) => self.player.process_action(a)?,
			}
		}

		self.actions.clear();
		Ok(())
	}

	fn stack_input(&mut self, dir: MoveDirection) {
		{
			let n = self.input_stack.entry(dir).or_insert(0);
			*n = n.saturating_add(1);
		}
		self.stack_to_action();
	}

	fn unstack_input(&mut self, dir: MoveDirection) {
		{
			let n = self.input_stack.entry(dir).or_insert(0);
			*n = n.saturating_sub(1);
		}
		self.stack_to_action();
	}

	fn stack_to_action(&mut self) {
		let dir = self.input_stack.iter().fold(None, |acc, (&dir, &n)| {
			if n > 0 {
				if acc.is_none() {
					Some(dir)
				} else {
					None
				}
			} else {
				acc
			}
		});

		self.add_action(PlayerAction::Move(dir));
	}
}

impl EventHandler for MainState {
	/// Called upon each physics update to the game.
	/// This should be where the game's logic takes place.
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		self.process_actions(ctx)?;

		if self.paused {
			return Ok(());
		}

		self.player.update(ctx)?;

		if self.timer % 10 == 0 {
			self.baddies.push(Baddie::new());
		}

		self.baddies.retain(|b| b.body.y < HEIGHT);

		// used in place of drain_filter...
		let mut i = 0;
		while i != self.baddies.len() {
			if self.player.overlaps(&self.baddies[i].body) {
				let baddie = self.baddies.remove(i);
				self.player.collides(&baddie);
			} else {
				i += 1;
			}
		}

		self.timer += 1;

		for baddie in &mut self.baddies {
			baddie.update(ctx)?;
		}

		Ok(())
	}

	/// Called to do the drawing of your game.
	/// You probably want to start this with
	/// `graphics::clear()` and end it with
	/// `graphics::present()` and `timer::sleep_until_next_frame()`
	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		use self::graphics::*;

		clear(ctx);

		// draw baddies
		for baddie in &self.baddies {
			baddie.draw(&self.resources, ctx)?;
		}

		// draw player
		self.player.draw(&self.resources, ctx)?;

		// draw ground
		set_color(ctx, Color::from_rgb(0, 0, 0))?;
		rectangle(
			ctx,
			DrawMode::Fill,
			Rect::new(0.0, HEIGHT - GROUND_HEIGHT, WIDTH, GROUND_HEIGHT),
		)?;

		self.player.draw_ui(&self.resources, ctx)?;

		// draw paused
		if self.paused {
			set_color(ctx, Color::from_rgb(255, 255, 255))?;
			let text_pause = Text::new(ctx, "PAUSED", &self.resources.font)?;
			let pos_x =
				ctx.conf.window_mode.width / 2 - self.resources.font.get_width("PAUSED") as u32 / 2;
			let pos_y =
				ctx.conf.window_mode.height / 2 - self.resources.font.get_height() as u32 / 2;

			draw(
				ctx,
				&text_pause,
				Point2::new(pos_x as f32, pos_y as f32),
				0.,
			)?;
		}

		present(ctx);

		let frame = timer::get_ticks(ctx);
		if frame % 100 == 0 {
			info!(
				"[FRAME {}]: {:0.0} FPS / {:0.2} ms per frame",
				frame,
				timer::get_fps(ctx),
				timer::duration_to_f64(timer::get_average_delta(ctx)) * 1000.0
			);
		}

		timer::yield_now();
		Ok(())
	}

	/// A mouse button was pressed
	fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
		debug!("mouse_button_down_event - {:?}: ({},{})", button, x, y);
	}

	/// A mouse button was released
	fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
		debug!("mouse_button_up_event - {:?}: ({},{})", button, x, y);
	}

	/// The mouse was moved; it provides both absolute x and y coordinates in the window,
	/// and relative x and y coordinates compared to its last position.
	fn mouse_motion_event(
		&mut self,
		_ctx: &mut Context,
		_state: MouseState,
		x: i32,
		y: i32,
		xrel: i32,
		yrel: i32,
	) {
		debug!(
			"mouse_motion_event - [STATE]: ({},{})/({},{})",
			x, y, xrel, yrel
		);
	}

	/// The mousewheel was clicked.
	fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: i32, y: i32) {
		debug!("mouse_wheel_event - ({},{})", x, y);
	}

	/// A keyboard button was pressed.
	fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
		debug!(
			"key_down_event - {:?} ({:?}): {}",
			keycode,
			keymod,
			if repeat { "repeated" } else { "first" }
		);

		use self::Keycode::*;

		if repeat {
			return;
		}

		match keycode {
			Escape => self.add_action(GameAction::Quit),
			Left => self.stack_input(MoveDirection::Left),
			Right => self.stack_input(MoveDirection::Right),
			Down if !self.player.on_the_ground() => self.add_action(PlayerAction::Dump),
			Up if self.player.on_the_ground() => self.add_action(PlayerAction::Jump),
			Space => self.add_action(GameAction::Pause),
			_ => (),
		}
	}

	/// A keyboard button was released.
	fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
		debug!(
			"key_up_event - {:?} ({:?}): {}",
			keycode,
			keymod,
			if repeat { "repeated" } else { "first" }
		);

		use self::Keycode::*;

		match keycode {
			Left => self.unstack_input(MoveDirection::Left),
			Right => self.unstack_input(MoveDirection::Right),
			Down => self.add_action(PlayerAction::StopDump),
			_ => (),
		}
	}

	/// A controller button was pressed; instance_id identifies which controller.
	fn controller_button_down_event(&mut self, _ctx: &mut Context, btn: Button, instance_id: i32) {
		debug!("controller_button_down_event - {:?} ({})", btn, instance_id);

		use self::MoveDirection::*;

		match btn {
			Button::DPadLeft => self.stack_input(Left),
			Button::DPadRight => self.stack_input(Right),
			Button::DPadDown if !self.player.on_the_ground() => self.add_action(PlayerAction::Dump),
			Button::B if self.player.on_the_ground() => self.add_action(PlayerAction::Jump),
			Button::Start => self.add_action(GameAction::Pause),
			_ => (),
		}
	}
	/// A controller button was released.
	fn controller_button_up_event(&mut self, _ctx: &mut Context, btn: Button, instance_id: i32) {
		debug!("controller_button_up_event - {:?} ({})", btn, instance_id);

		use self::MoveDirection::*;

		match btn {
			Button::DPadLeft => self.unstack_input(Left),
			Button::DPadRight => self.unstack_input(Right),
			Button::DPadDown => self.add_action(PlayerAction::StopDump),
			_ => (),
		}
	}
	/// A controller axis moved.
	fn controller_axis_event(
		&mut self,
		_ctx: &mut Context,
		axis: Axis,
		value: i16,
		instance_id: i32,
	) {
		debug!(
			"controller_axis_event - {:?}[{}] ({})",
			axis, value, instance_id
		);
	}

	/// Called when the window is shown or hidden.
	fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
		debug!("focus_event - {}", if gained { "gained" } else { "loose" });
	}
}
