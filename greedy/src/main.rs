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
}

impl Agent<GameState, Action> for GreedyAgent {
    fn choose_action(&mut self, state: &GameState) -> Action {
        state
            .valid_actions()
            .into_iter()
            .max_by_key(|action| {
                let mut next_state = state.clone();
                next_state.advance(*action);
                next_state.score
            })
            .unwrap()
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut state = GameState::new(3, 3, 4, rng.gen());
    GreedyAgent::new().play_game(&mut state);

    println!("{:?}", state);
}
