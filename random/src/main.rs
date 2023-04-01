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

    fn choose_action(&mut self, state: &GameState) -> Option<Action> {
        let actions = state.valid_actions();
        Some(actions[self.rng.gen_range(0..actions.len())])
    }
}

impl<T: RngCore> Agent<GameState, Action> for RandomAgent<'_, T> {
    fn play_game(&mut self, state: &GameState) -> GameState {
        let mut next_state = state.clone();

        while !next_state.is_game_over() {
            let action = self.choose_action(&next_state).unwrap();
            next_state.advance(action);
        }

        next_state
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let initial_state = GameState::new(3, 3, 4, rng.gen());
    let last_state = RandomAgent::new(&mut rng).play_game(&initial_state);

    println!("{:?}", last_state);
}
