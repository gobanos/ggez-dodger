#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Action {
    MoveLeft,
    MoveRight,
    StopMove,
    Jump,
    Dump,
    StopDump,
    Pause,
    Quit,
}
