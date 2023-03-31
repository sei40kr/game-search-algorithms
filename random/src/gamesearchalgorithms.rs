pub trait Agent<T: GameStateBase<S>, S> {
    fn choose_action(&mut self, state: &T) -> S;

    fn play_game(&mut self, state: &mut T) {
        while !state.is_game_over() {
            let action = self.choose_action(&state);
            state.advance(action);
        }
    }
}

pub trait GameStateBase<T> {
    fn is_game_over(&self) -> bool;

    fn valid_actions(&self) -> Vec<T>;

    fn advance(&mut self, action: T);
}

pub mod maze {
    use rand::Rng;
    use rand::{rngs::StdRng, SeedableRng};
    use std::fmt::Debug;

    use super::GameStateBase;

    struct Player {
        y: usize,
        x: usize,
    }

    pub struct GameState {
        h: usize,
        w: usize,
        score: i32,
        points: Vec<Vec<i32>>,
        max_turns: i32,
        turn: i32,
        player: Player,
    }

    #[derive(Clone, Copy)]
    pub enum Action {
        Up,
        Down,
        Left,
        Right,
    }

    impl GameState {
        pub fn new(h: usize, w: usize, max_turns: i32, seed: u64) -> Self {
            let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

            let points: Vec<Vec<i32>> = (0..h)
                .map(|_| (0..w).map(|_| rng.gen_range(1..10)).collect())
                .collect();

            return GameState {
                h,
                w,
                points,
                player: Player {
                    y: rng.gen_range(0..h),
                    x: rng.gen_range(0..w),
                },
                max_turns,
                turn: 0,
                score: 0,
            };
        }
    }

    impl GameStateBase<Action> for GameState {
        fn is_game_over(&self) -> bool {
            return self.turn >= self.max_turns;
        }

        fn valid_actions(&self) -> Vec<Action> {
            let mut actions = Vec::with_capacity(4);

            if self.player.y > 0 {
                actions.push(Action::Up);
            }
            if self.player.y < self.h - 1 {
                actions.push(Action::Down);
            }
            if self.player.x > 0 {
                actions.push(Action::Left);
            }
            if self.player.x < self.w - 1 {
                actions.push(Action::Right);
            }

            return actions;
        }

        fn advance(&mut self, action: Action) {
            let (dy, dx) = match action {
                Action::Up => (-1, 0),
                Action::Down => (1, 0),
                Action::Left => (0, -1),
                Action::Right => (0, 1),
            };

            self.turn += 1;

            self.player.y = (self.player.y as i32 + dy) as usize;
            self.player.x = (self.player.x as i32 + dx) as usize;

            let point = &mut self.points[self.player.x][self.player.y];

            self.score += *point;

            *point = 0;
        }
    }

    impl Debug for GameState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "turn:\t{}", self.turn)?;
            writeln!(f, "score:\t{}", self.score)?;

            for y in 0..self.h {
                let line = (0..self.w)
                    .map(|x| {
                        if x == self.player.x && y == self.player.y {
                            "@".to_string()
                        } else if 0 < self.points[x][y] {
                            self.points[x][y].to_string()
                        } else {
                            ".".to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("");

                writeln!(f, "{}", line)?;
            }

            Ok(())
        }
    }
}
