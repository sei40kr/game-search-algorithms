pub mod automovemaze {
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::fmt::Debug;

    pub trait Agent {
        // Play a game and return the final state
        fn play_game(&mut self, state: &State) -> State;
    }

    #[derive(Clone)]
    pub struct Character {
        y: usize,
        x: usize,
    }

    #[derive(Clone, Copy)]
    pub enum Action {
        Up,
        Down,
        Left,
        Right,
    }

    #[derive(Clone)]
    pub struct State {
        h: usize,
        w: usize,
        max_turn: i32,
        turn: i32,
        score: i32,
        points: Vec<Vec<i32>>,
        characters: Vec<Character>,
    }

    impl State {
        pub fn new(
            random_seed: u64,
            h: usize,
            w: usize,
            num_characters: usize,
            max_turn: i32,
        ) -> Self {
            let mut rng: StdRng = SeedableRng::seed_from_u64(random_seed);
            let points = (0..h)
                .map(|_| (0..w).map(|_| rng.gen_range(1..10)).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let characters = (0..num_characters)
                .map(|_| Character { y: 0, x: 0 })
                .collect::<Vec<_>>();

            return State {
                h,
                w,
                max_turn,
                turn: 0,
                score: 0,
                points,
                characters,
            };
        }

        pub fn h(&self) -> usize {
            self.h
        }

        pub fn w(&self) -> usize {
            self.w
        }

        pub fn max_turn(&self) -> i32 {
            self.max_turn
        }

        pub fn num_characters(&self) -> usize {
            self.characters.len()
        }

        pub fn turn(&self) -> i32 {
            self.turn
        }

        pub fn score(&self) -> i32 {
            self.score
        }

        pub fn characters(&self) -> &[Character] {
            self.characters.as_ref()
        }

        pub fn is_game_over(&self) -> bool {
            return self.turn >= self.max_turn;
        }

        pub fn valid_actions(&self, character_index: usize) -> Vec<Action> {
            if self.num_characters() <= character_index {
                panic!("invalid character index");
            }

            let character = &self.characters[character_index];
            let mut actions = Vec::with_capacity(4);

            if character.y > 0 {
                actions.push(Action::Up);
            }
            if character.y < self.h - 1 {
                actions.push(Action::Down);
            }
            if character.x > 0 {
                actions.push(Action::Left);
            }
            if character.x < self.w - 1 {
                actions.push(Action::Right);
            }

            return actions;
        }

        pub fn set_character_coord(&mut self, i: usize, x: usize, y: usize) {
            if self.num_characters() <= i {
                panic!("invalid character index");
            }

            let mut character = &mut self.characters[i];
            character.y = y;
            character.x = x;

            self.points[character.y][character.x] = 0;
        }

        pub fn advance(&mut self) {
            for i in 0..self.num_characters() {
                self.advance_character(i);
            }

            self.turn += 1;
        }

        fn advance_character(&mut self, i: usize) {
            if self.num_characters() <= i {
                panic!("invalid character index");
            }

            let (new_x, new_y) = self
                .valid_actions(i)
                .iter()
                .map(|action| {
                    let character = &self.characters[i];
                    let (dy, dx) = match action {
                        Action::Up => (-1, 0),
                        Action::Down => (1, 0),
                        Action::Left => (0, -1),
                        Action::Right => (0, 1),
                    };

                    let y = (character.y as i32 + dy) as usize;
                    let x = (character.x as i32 + dx) as usize;

                    (y, x)
                })
                .max_by_key(|(y, x)| {
                    return self.points[*y][*x];
                })
                .unwrap();

            let mut character = &mut self.characters[i];
            character.y = new_y;
            character.x = new_x;

            let point = &mut self.points[character.x][character.y];
            self.score += *point;

            *point = 0;
        }
    }

    impl Debug for State {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "turn:\t{}", self.turn)?;
            writeln!(f, "score:\t{}", self.score)?;

            for y in 0..self.h {
                let line = (0..self.w)
                    .map(|x| {
                        if self.characters.iter().any(|c| c.x == x && c.y == y) {
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
