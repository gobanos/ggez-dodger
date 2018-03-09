use actions::{Action, GameAction, MoveDirection, PlayerAction};
use baddies::Baddie;
use constants::*;
use ggez::{graphics, timer, Context, GameResult};
use ggez::event::{Axis, Button, EventHandler, Keycode, Mod, MouseButton, MouseState};
use ggez::graphics::Point2;
use player::Player;
use resources::Resources;
use std::collections::HashMap;

pub type ControllerId = i32;

pub struct MainState {
    players: HashMap<ControllerId, Player>,
    actions: Vec<Action>,
    timer: u32,
    baddies: Vec<Baddie>,
    resources: Resources,
    paused: bool,
    input_stack: HashMap<(MoveDirection, ControllerId), u32>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            players: HashMap::new(),
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
        use self::GameAction::*;

        for &action in &self.actions {
            match action {
                Action::Game(Pause) => self.paused = !self.paused,
                Action::Game(Quit) => ctx.quit()?,
                Action::Game(Spawn(id)) => {
                    let index = self.players.len() as u8;
                    self.players
                        .entry(id)
                        .or_insert_with(|| Player::new(index, Point2::new(WIDTH / 2.0, MAX_Y)));
                }
                Action::Player(_, _) if self.paused => (),
                Action::Player(a, id) => self.players.get_mut(&id).unwrap().process_action(a)?,
            }
        }

        self.actions.clear();
        Ok(())
    }

    fn stack_input(&mut self, dir: MoveDirection, instance_id: ControllerId) {
        {
            let n = self.input_stack.entry((dir, instance_id)).or_insert(0);
            *n = n.saturating_add(1);
        }
        self.stack_to_action();
    }

    fn unstack_input(&mut self, dir: MoveDirection, instance_id: ControllerId) {
        {
            let n = self.input_stack.entry((dir, instance_id)).or_insert(0);
            *n = n.saturating_sub(1);
        }
        self.stack_to_action();
    }

    fn stack_to_action(&mut self) {
        let dir = self.input_stack
            .iter()
            .fold(HashMap::new(), |mut acc, (&(dir, id), &n)| {
                {
                    let cur = acc.entry(id).or_insert(None);
                    *cur = if n > 0 {
                        if cur.is_none() {
                            Some(dir)
                        } else {
                            None
                        }
                    } else {
                        *cur
                    };
                }

                acc
            });

        for (id, d) in dir {
            self.add_action((PlayerAction::Move(d), id));
        }
    }
}

impl EventHandler for MainState {
    /// Called upon each physics update to the game.
    /// This should be where the game's logic takes place.
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.process_actions(ctx)?;

        if self.players.is_empty() || self.paused {
            return Ok(());
        }

        for p in self.players.values_mut() {
            p.update(ctx)?;
        }

        if self.timer % 10 == 0 {
            self.baddies.push(Baddie::new());
        }

        self.baddies.retain(|b| b.body.y < HEIGHT);

        // used in place of drain_filter...
        let mut i = 0;
        while i != self.baddies.len() {
            if let Some((&id, _)) = self.players
                .iter()
                .find(|&(_, p)| p.overlaps(&self.baddies[i].body))
            {
                let baddie = self.baddies.remove(i);
                self.add_action((PlayerAction::Collides(baddie), id));
                break;
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
        for p in self.players.values() {
            p.draw(&self.resources, ctx)?;
        }

        // draw ground
        set_color(ctx, Color::from_rgb(0, 0, 0))?;
        rectangle(
            ctx,
            DrawMode::Fill,
            Rect::new(0.0, HEIGHT - GROUND_HEIGHT, WIDTH, GROUND_HEIGHT),
        )?;

        for p in self.players.values() {
            p.draw_ui(&self.resources, self.players.len(), ctx)?;
        }

        // draw message
        if self.players.is_empty() || self.paused {
            set_color(ctx, Color::from_rgb(255, 255, 255))?;
            let text = if self.players.is_empty() {
                &self.resources.waiting
            } else {
                &self.resources.pause
            };
            let Rect { w: tw, h: th, .. } = text.get_dimensions();

            draw(
                ctx,
                text,
                Point2::new((WIDTH - tw) / 2.0, (HEIGHT - GROUND_HEIGHT - th) / 2.0),
                0.0,
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
            //            Left => self.stack_input(MoveDirection::Left),
            //            Right => self.stack_input(MoveDirection::Right),
            //            Down if !self.player.on_the_ground() => self.add_action(PlayerAction::Dump(true)),
            //            Up if self.player.on_the_ground() => self.add_action(PlayerAction::Jump),
            //            Space => self.add_action(GameAction::Pause),
            //            RCtrl => self.add_action(PlayerAction::Shield(true)),
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

        //        use self::Keycode::*;

        match keycode {
            //            Left => self.unstack_input(MoveDirection::Left),
            //            Right => self.unstack_input(MoveDirection::Right),
            //            Down => self.add_action(PlayerAction::Dump(false)),
            //            RCtrl => self.add_action(PlayerAction::Shield(false)),
            _ => (),
        }
    }

    /// A controller button was pressed; instance_id identifies which controller.
    fn controller_button_down_event(
        &mut self,
        _ctx: &mut Context,
        btn: Button,
        instance_id: ControllerId,
    ) {
        debug!("controller_button_down_event - {:?} ({})", btn, instance_id);

        use self::MoveDirection::*;

        let (with_player, on_the_ground) = {
            let player = self.players.get(&instance_id);
            let with_player = player.is_some();
            let on_the_ground = player.map(|p| p.on_the_ground()).unwrap_or(false);

            (with_player, on_the_ground)
        };

        match (btn, with_player) {
            (Button::DPadLeft, true) => self.stack_input(Left, instance_id),
            (Button::DPadRight, true) => self.stack_input(Right, instance_id),
            (Button::DPadDown, true) if !on_the_ground => {
                self.add_action((PlayerAction::Dump(true), instance_id))
            }
            (Button::B, true) if on_the_ground => {
                self.add_action((PlayerAction::Jump, instance_id))
            }
            (Button::A, true) => self.add_action((PlayerAction::Shield(true), instance_id)),
            (Button::Start, _) => self.add_action(GameAction::Pause),
            (Button::Back, _) => self.add_action(GameAction::Spawn(instance_id)),
            _ => (),
        }
    }
    /// A controller button was released.
    fn controller_button_up_event(
        &mut self,
        _ctx: &mut Context,
        btn: Button,
        instance_id: ControllerId,
    ) {
        debug!("controller_button_up_event - {:?} ({})", btn, instance_id);

        use self::MoveDirection::*;

        let with_player = self.players.contains_key(&instance_id);

        match (btn, with_player) {
            (Button::DPadLeft, true) => self.unstack_input(Left, instance_id),
            (Button::DPadRight, true) => self.unstack_input(Right, instance_id),
            (Button::DPadDown, true) => self.add_action((PlayerAction::Dump(false), instance_id)),
            (Button::A, true) => self.add_action((PlayerAction::Shield(false), instance_id)),
            _ => (),
        }
    }
    /// A controller axis moved.
    fn controller_axis_event(
        &mut self,
        _ctx: &mut Context,
        axis: Axis,
        value: i16,
        instance_id: ControllerId,
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
