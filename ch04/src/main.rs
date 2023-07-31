use rand::{self, Rng};

const H: usize = 5;
const W: usize = 5;
const END_TURN: usize = 5;
const CHARACTER_N: usize = 3;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct AutoMoveMazeState {
    points: Vec<Vec<i32>>,
    turn: usize,
    characters: Vec<Coord>,
    game_score: i64,
    evaluated_score: i64,
}

impl AutoMoveMazeState {
    #[allow(non_upper_case_globals)]
    const dx: [i32; 4] = [1, -1, 0, 0];
    #[allow(non_upper_case_globals)]
    const dy: [i32; 4] = [0, 0, 1, -1];

    fn new(seed: u64) -> AutoMoveMazeState {
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
        let mut points = vec![vec![0; H]; W];
        for i in 0..W {
            for j in 0..H {
                points[i][j] = rng.gen_range(0..10);
            }
        }

        AutoMoveMazeState {
            points,
            turn: 0,
            characters: vec![Coord::new(0, 0); CHARACTER_N],
            game_score: 0,
            evaluated_score: 0,
        }
    }

    fn set_character(&mut self, character_id: usize, x: i32, y: i32) {
        self.characters[character_id].x = x;
        self.characters[character_id].y = y;
    }

    const fn is_done(&self) -> bool {
        self.turn == END_TURN
    }

    fn move_player(&mut self, character_id: usize) {
        let character = &mut self.characters[character_id];
        let mut best_point = std::i32::MIN;
        let mut best_action_index = 0;
        for act in 0..4 {
            let nx = character.x + Self::dx[act];
            let ny = character.y + Self::dy[act];
            if nx >= 0 && nx < W as i32 && ny >= 0 && ny < H as i32 {
                let point = self.points[nx as usize][ny as usize];
                if point > best_point {
                    best_point = point;
                    best_action_index = act;
                }
            }
        }
        character.x += Self::dx[best_action_index];
        character.y += Self::dy[best_action_index];
    }

    fn advance(&mut self) {
        for character_id in 0..CHARACTER_N {
            self.move_player(character_id);
        }

        for character in &self.characters {
            let point = &mut self.points[character.x as usize][character.y as usize];
            self.game_score += *point as i64;
            *point = 0;
        }
        self.turn += 1;
    }

    fn get_score(&mut self, is_print: bool) -> i64 {
        for character in &self.characters {
            let point = &mut self.points[character.x as usize][character.y as usize];
            *point = 0;
        }

        while !self.is_done() {
            self.advance();
            if is_print {
                self.to_string();
            }
        }

        self.game_score
    }

    #[allow(dead_code)]
    fn to_string(&self) {
        println!("turn: {}", self.turn);
        println!("score: {}", self.game_score);
        for i in 0..W {
            for j in 0..H {
                let mut is_written = false;
                for character in &self.characters {
                    if i == character.x as usize && j == character.y as usize {
                        print!("@");
                        is_written = true;
                        break;
                    }
                }
                if is_written {
                    continue;
                }
                if self.points[i][j] > 0 {
                    print!("{}", self.points[i][j]);
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

#[allow(dead_code)]
fn random_action(state: &mut AutoMoveMazeState, seed: u64) {
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
    for character_id in 0..CHARACTER_N {
        let x = rng.gen_range(0..W) as i32;
        let y = rng.gen_range(0..H) as i32;
        state.set_character(character_id, x, y);
    }
}

fn play_game(seed: u64) {
    let mut state = AutoMoveMazeState::new(seed);
    random_action(&mut state, seed);
    state.to_string();
    println!("Score of random action : {}", state.get_score(true));
}

fn main() {
    play_game(0);
}
