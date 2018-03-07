#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Action {
    Game(GameAction),
    Player(PlayerAction),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    StopMove,
    Jump,
    Dump,
    StopDump,
}

impl Into<Action> for PlayerAction {
    fn into(self) -> Action {
        Action::Player(self)
    }
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