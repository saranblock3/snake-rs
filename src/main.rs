mod game;
mod direction;
mod snake;
mod food;

fn main() {
   let mut game = game::Game::new(20, 20); 
   game.run();
}
