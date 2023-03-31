mod gamesearchalgorithms;

use gamesearchalgorithms::{
    maze::{Action, GameState},
    Agent, GameStateBase,
};
use rand::{Rng, RngCore};

struct RandomAgent<'a> {
    rng: &'a mut dyn RngCore,
}

impl<'a> RandomAgent<'a> {
    fn new(rng: &'a mut dyn RngCore) -> RandomAgent {
        RandomAgent { rng }
    }
}

impl Agent<GameState, Action> for RandomAgent<'_> {
    fn choose_action(&mut self, state: &GameState) -> Action {
        let actions = state.valid_actions();
        actions[self.rng.gen_range(0..actions.len())]
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut state = GameState::new(3, 3, 4, rng.gen());
    RandomAgent::new(&mut rng).play_game(&mut state);

    println!("{:?}", state);
}
