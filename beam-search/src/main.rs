use std::{cmp::Ordering, collections::BinaryHeap};

use lib::{
    maze::{Action, GameState},
    Agent, GameStateBase,
};
use rand::Rng;

struct BeamSearchAgent {
    beam_width: usize,
    beam_depth: usize,
}

struct BeamSearchState {
    state: GameState,
    first_action: Option<Action>,
}

impl PartialEq for BeamSearchState {
    fn eq(&self, other: &Self) -> bool {
        self.state.score == other.state.score
    }
}

impl Eq for BeamSearchState {}

impl PartialOrd for BeamSearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BeamSearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state.score.cmp(&other.state.score)
    }
}

impl BeamSearchAgent {
    fn new(beam_width: usize, beam_depth: usize) -> Self {
        BeamSearchAgent {
            beam_width,
            beam_depth,
        }
    }

    fn choose_action(&mut self, state: &GameState) -> Option<Action> {
        let mut beam = BinaryHeap::new();
        beam.push(BeamSearchState {
            state: state.clone(),
            first_action: None,
        });

        for _ in 0..self.beam_depth {
            let mut next_beam = BinaryHeap::new();

            while let Some(BeamSearchState {
                state,
                first_action,
            }) = beam.pop()
            {
                for action in state.valid_actions() {
                    let mut new_state = state.clone();
                    new_state.advance(action);
                    next_beam.push(BeamSearchState {
                        state: new_state,
                        first_action: first_action.or_else(|| Some(action)),
                    });
                }
            }

            if next_beam.is_empty() {
                break;
            }

            beam = next_beam
                .into_iter()
                .take(self.beam_width)
                .collect::<BinaryHeap<_>>();
        }

        beam.pop().unwrap().first_action
    }
}

impl Agent<GameState, Action> for BeamSearchAgent {
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
    let last_state = BeamSearchAgent::new(2, 4).play_game(&initial_state);

    println!("{:?}", last_state);
}

#[cfg(test)]
mod tests {
    use lib::Agent;
    use rand::Rng;

    #[test]
    fn score() {
        let mut rng = rand::thread_rng();
        let score = (0..100)
            .map(|_| {
                let initial_state = super::GameState::new(3, 3, 4, rng.gen());
                let last_state = super::BeamSearchAgent::new(2, 4).play_game(&initial_state);
                last_state.score
            })
            .sum::<i32>() as f64
            / 100.0;

        println!("Average score: {}", score);
    }
}
