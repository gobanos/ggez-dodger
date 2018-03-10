use baddies::Baddie;
use game::PlayerId;
use player::PlayerBody;

#[derive(Copy, Clone, Debug)]
pub enum Action {
    Game(GameAction),
    Player(PlayerAction, PlayerId),
}

#[derive(Copy, Clone, Debug)]
pub enum PlayerAction {
    Move(Option<MoveDirection>),
    Jump,
    Dump(bool),
    Shield(bool),
    Collides(Entity),
}

#[derive(Copy, Clone, Debug)]
pub enum Entity {
    Baddie(Baddie),
    Player(PlayerBody),
}

impl From<Baddie> for Entity {
    fn from(baddie: Baddie) -> Self {
        Entity::Baddie(baddie)
    }
}

impl From<PlayerBody> for Entity {
    fn from(body: PlayerBody) -> Self {
        Entity::Player(body)
    }
}

impl Into<Action> for (PlayerAction, PlayerId) {
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
    Spawn(PlayerId),
}

impl Into<Action> for GameAction {
    fn into(self) -> Action {
        Action::Game(self)
    }
}
