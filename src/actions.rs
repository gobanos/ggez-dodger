use baddies::Baddie;
use game::ControllerId;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Action {
    Game(GameAction),
    Player(PlayerAction, ControllerId),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayerAction {
    Move(Option<MoveDirection>),
    Jump,
    Dump(bool),
    Shield(bool),
    Collides(Baddie),
}

impl Into<Action> for (PlayerAction, ControllerId) {
    fn into(self) -> Action {
        Action::Player(self.0, self.1)
    }
}

impl From<MoveDirection> for PlayerAction {
    fn from(dir: MoveDirection) -> PlayerAction {
        PlayerAction::Move(Some(dir))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MoveDirection {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GameAction {
    Pause,
    Quit,
    Spawn(ControllerId),
}

impl Into<Action> for GameAction {
    fn into(self) -> Action {
        Action::Game(self)
    }
}
