use lib::{
    maze::{Action, GameState},
    Agent, GameStateBase,
};
use rand::{Rng, RngCore};

struct RandomAgent<'a, T: RngCore> {
    rng: &'a mut T,
}

impl<'a, T: RngCore> RandomAgent<'a, T> {
    fn new(rng: &'a mut T) -> Self {
        RandomAgent { rng }
    }
}

impl<T: RngCore> Agent<GameState, Action> for RandomAgent<'_, T> {
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
