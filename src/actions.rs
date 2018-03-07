#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Action {
    Game(GameAction),
    Player(PlayerAction),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlayerAction {
    Move(MoveDirection),
    Jump,
    Dump,
    StopDump,
}

impl Into<Action> for PlayerAction {
    fn into(self) -> Action {
        Action::Player(self)
    }
}

impl From<MoveDirection> for PlayerAction {
    fn from(dir: MoveDirection) -> PlayerAction {
        PlayerAction::Move(dir)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MoveDirection {
    Stop,
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
