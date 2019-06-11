pub struct GameState {
    pub quit: bool,
    pub game_screen: GameScreen,
}

pub enum GameScreen {
    Menu,
    Game,
    NextLevel,
    Pause,
    GameOver,
    QuitConfirm,
}
