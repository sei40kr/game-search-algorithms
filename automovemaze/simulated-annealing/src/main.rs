use lib::automovemaze::Agent;
use lib::automovemaze::State;
use rand::{Rng, RngCore};

struct TemperatureRange {
    start: u32,
    end: u32,
}

struct SimulatedAnnealingAgent<'a, T: RngCore> {
    rng: &'a mut T,
    count: u32,
    temp_range: TemperatureRange,
}

impl<'a, T: RngCore> SimulatedAnnealingAgent<'a, T> {
    fn new(rng: &'a mut T, count: u32, temp_start: u32, temp_end: u32) -> Self {
        SimulatedAnnealingAgent {
            rng,
            count,
            temp_range: TemperatureRange {
                start: temp_start,
                end: temp_end,
            },
        }
    }

    fn gen_neighbor(&mut self, state: &State) -> State {
        let mut neighbor = state.clone();

        neighbor.set_character_coord(
            self.rng.gen_range(0..state.num_characters()),
            self.rng.gen_range(0..state.h()),
            self.rng.gen_range(0..state.w()),
        );

        neighbor
    }
}

impl<T: RngCore> Agent for SimulatedAnnealingAgent<'_, T> {
    fn play_game(&mut self, current_state: &State) -> State {
        let mut state = current_state.clone();
        let mut best_state = state.clone();

        for i in 0..self.count {
            let mut neighbor = self.gen_neighbor(&state);

            while !neighbor.is_game_over() {
                neighbor.advance();
            }

            let TemperatureRange {
                start: temp_start,
                end: temp_end,
            } = self.temp_range;

            let delta = neighbor.score() - state.score();
            let temp = temp_start as f64
                + (temp_end as f64 - temp_start as f64) as f64 * (i as f64 / self.count as f64);
            let prob = (delta as f64 / temp).exp();

            if best_state.score() < neighbor.score() {
                best_state = neighbor.clone();
            }

            if delta > 0 || self.rng.gen_bool(prob.max(1.0)) {
                state = neighbor;
            }
        }

        best_state
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let initial_state = State::new(rng.gen(), 5, 5, 3, 5);
    let final_state =
        SimulatedAnnealingAgent::new(&mut rng, 10000, 500, 10).play_game(&initial_state);

    println!("{:?}", final_state);
}

#[cfg(test)]
mod tests {
    use lib::automovemaze::Agent;
    use lib::automovemaze::State;
    use rand::Rng;

    use super::SimulatedAnnealingAgent;

    #[test]
    fn score() {
        let mut rng = rand::thread_rng();
        let score = (0..1000)
            .map(|_| {
                let initial_state = State::new(rng.gen(), 5, 5, 3, 5);
                let last_state = SimulatedAnnealingAgent::new(&mut rng, 10000, 500, 10)
                    .play_game(&initial_state);

                last_state.score()
            })
            .sum::<i32>() as f64
            / 1000.0;

        println!("Average score: {}", score);
    }
}
