
// Local modules
use crate::game::{Game};


pub trait GameScreen {
    fn process(&mut self, game: &mut Game) {
    }

    fn update(&mut self, game: &mut Game) {
    }

    fn draw(&mut self, game: &mut Game) {
    }
}
