#[derive(Debug, Copy, Clone)]
pub enum SubState {
    Waiting,
    Done,
}

#[derive(Debug, Copy, Clone)]
pub enum GameState {
    Stage1(SubState),
    Stage2,
    Stage3(SubState),
    GameStart,
    Game,
}