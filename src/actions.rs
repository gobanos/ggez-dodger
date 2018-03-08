#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Action {
    Game(GameAction),
    Player(PlayerAction),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlayerAction {
    Move(Option<MoveDirection>),
    Jump,
    Dump(bool),
    Shield(bool),
}

impl Into<Action> for PlayerAction {
    fn into(self) -> Action {
        Action::Player(self)
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
}

impl Into<Action> for GameAction {
    fn into(self) -> Action {
        Action::Game(self)
    }
}
