pub struct GameState {
    pub quit: bool,
    pub game_screen: GameScreen,
    pub sleep_time: i64,
    pub frame_duration: i64,
}

pub enum GameScreen {
    Menu,
    Game,
    NextLevel,
    Pause,
    GameOver,
    QuitConfirm,
}
