use std::{cell::RefCell, cmp::Ordering, collections::BinaryHeap};

use lib::{
    maze::{Action, GameState},
    Agent, GameStateBase,
};
use rand::Rng;

struct ChokudaiSearchAgent {
    beam_width: usize,
    beam_depth: usize,
    beam_count: usize,
}

struct ChokudaiSearchState {
    game_state: GameState,
    first_action: Option<Action>,
}

impl PartialEq for ChokudaiSearchState {
    fn eq(&self, other: &Self) -> bool {
        self.game_state.score == other.game_state.score
    }
}

impl Eq for ChokudaiSearchState {}

impl PartialOrd for ChokudaiSearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ChokudaiSearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.game_state.score.cmp(&other.game_state.score)
    }
}

impl ChokudaiSearchAgent {
    fn new(beam_width: usize, beam_depth: usize, beam_count: usize) -> Self {
        ChokudaiSearchAgent {
            beam_width,
            beam_depth,
            beam_count,
        }
    }

    fn choose_action(&mut self, state: &GameState) -> Option<Action> {
        let beams = (0..(self.beam_depth + 1))
            .map(|_| RefCell::new(BinaryHeap::new()))
            .collect::<Vec<_>>();
        beams[0].borrow_mut().push(ChokudaiSearchState {
            game_state: state.clone(),
            first_action: None,
        });

        for _ in 0..self.beam_count {
            for t in 0..self.beam_depth - 1 {
                let mut current_beam = beams[t].borrow_mut();
                let mut next_beam = beams[t + 1].borrow_mut();

                for _ in 0..self.beam_width {
                    if current_beam.is_empty() {
                        break;
                    }
                    if current_beam.peek().unwrap().game_state.is_game_over() {
                        break;
                    }

                    let ChokudaiSearchState {
                        game_state: current_game_state,
                        first_action,
                    } = current_beam.pop().unwrap();

                    for action in current_game_state.valid_actions() {
                        let mut next_game_state = current_game_state.clone();
                        next_game_state.advance(action);

                        next_beam.push(ChokudaiSearchState {
                            game_state: next_game_state,
                            first_action: first_action.or_else(|| Some(action)),
                        });
                    }
                }
            }
        }

        return beams
            .into_iter()
            .rev()
            .find_map(|beam| beam.borrow_mut().pop())
            .and_then(|state| state.first_action);
    }
}

impl Agent<GameState, Action> for ChokudaiSearchAgent {
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
    let last_state = ChokudaiSearchAgent::new(2, 4, 2).play_game(&initial_state);

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
                let last_state = super::ChokudaiSearchAgent::new(2, 4, 2).play_game(&initial_state);
                last_state.score
            })
            .sum::<i32>() as f64
            / 100.0;

        println!("Average score: {}", score);
    }
}
