use lib::{
    maze::{Action, GameState},
    Agent, GameStateBase,
};
use rand::Rng;

struct GreedyAgent {}

impl GreedyAgent {
    fn new() -> Self {
        GreedyAgent {}
    }

    fn choose_action(&mut self, state: &GameState) -> Option<Action> {
        state.valid_actions().into_iter().max_by_key(|action| {
            let mut next_state = state.clone();
            next_state.advance(*action);
            next_state.score
        })
    }
}

impl Agent<GameState, Action> for GreedyAgent {
    fn play_game(&mut self, state: &GameState) -> GameState {
        let mut next_state = state.clone();

        while !next_state.is_game_over() {
            let action = self.choose_action(&next_state).unwrap();
            next_state.advance(action)
        }

        next_state
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let initial_state = GameState::new(3, 3, 4, rng.gen());
    let last_state = GreedyAgent::new().play_game(&initial_state);

    println!("{:?}", last_state);
}
