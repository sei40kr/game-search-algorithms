use lib::automovemaze::Agent;
use lib::automovemaze::State;
use rand::{Rng, RngCore};

struct HillClimbAgent<'a, T: RngCore> {
    rng: &'a mut T,
    count: u32,
}

impl<'a, T: RngCore> HillClimbAgent<'a, T> {
    fn new(rng: &'a mut T, count: u32) -> Self {
        HillClimbAgent { rng, count }
    }

    fn generate_neighbor(&mut self, state: &State) -> State {
        let mut neighbor = state.clone();

        neighbor.set_character_coord(
            self.rng.gen_range(0..state.num_characters()),
            self.rng.gen_range(0..state.h()),
            self.rng.gen_range(0..state.w()),
        );

        neighbor
    }
}

impl<T: RngCore> Agent for HillClimbAgent<'_, T> {
    fn play_game(&mut self, current_state: &State) -> State {
        let mut state = current_state.clone();

        for _ in 0..self.count {
            let mut neighbor = self.generate_neighbor(&state);

            while !neighbor.is_game_over() {
                neighbor.advance();
            }

            if neighbor.score() > state.score() {
                state = neighbor;
            }
        }

        state
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let initial_state = State::new(rng.gen(), 5, 5, 3, 5);
    let final_state = HillClimbAgent::new(&mut rng, 10000).play_game(&initial_state);

    println!("{:?}", final_state);
}

#[cfg(test)]
mod tests {
    use lib::automovemaze::Agent;
    use lib::automovemaze::State;
    use rand::Rng;

    use super::HillClimbAgent;

    #[test]
    fn score() {
        let mut rng = rand::thread_rng();
        let score = (0..100)
            .map(|_| {
                let initial_state = State::new(rng.gen(), 5, 5, 3, 5);
                let last_state = HillClimbAgent::new(&mut rng, 10000).play_game(&initial_state);

                last_state.score()
            })
            .sum::<i32>() as f64
            / 100.0;
        println!("Average score: {}", score);
    }
}
