use lib::automovemaze;
use lib::automovemaze::Agent;
use rand::{Rng, RngCore};

struct RandomAgent<'a, T: RngCore> {
    rng: &'a mut T,
}

impl<'a, T: RngCore> RandomAgent<'a, T> {
    fn new(rng: &'a mut T) -> Self {
        RandomAgent { rng }
    }
}

impl<T: RngCore> automovemaze::Agent for RandomAgent<'_, T> {
    fn play_game(&mut self, current_state: &automovemaze::State) -> automovemaze::State {
        let mut state = current_state.clone();

        for i in 0..state.num_characters() {
            state.set_character_coord(
                i,
                self.rng.gen_range(0..state.h()),
                self.rng.gen_range(0..state.w()),
            );
        }

        while !state.is_game_over() {
            state.advance();
        }

        state
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let initial_state = automovemaze::State::new(rng.gen(), 5, 5, 3, 5);
    let final_state = RandomAgent::new(&mut rng).play_game(&initial_state);

    println!("{:?}", final_state);
}

#[cfg(test)]
mod tests {
    use lib::automovemaze;
    use lib::automovemaze::Agent;
    use rand::Rng;

    #[test]
    fn score() {
        let mut rng = rand::thread_rng();
        let score = (0..100)
            .map(|_| {
                let initial_state = automovemaze::State::new(rng.gen(), 5, 5, 3, 5);
                let last_state = super::RandomAgent::new(&mut rng).play_game(&initial_state);

                last_state.score()
            })
            .sum::<i32>() as f64
            / 100.0;
        println!("Average score: {}", score);
    }
}
