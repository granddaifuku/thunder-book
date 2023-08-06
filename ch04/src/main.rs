use once_cell::sync::Lazy;
use rand::{self, Rng};
use std::sync::Mutex;

const H: usize = 5;
const W: usize = 5;
const END_TURN: usize = 5;
const CHARACTER_N: usize = 3;

static RNG: Lazy<Mutex<rand::rngs::StdRng>> =
    Lazy::new(|| Mutex::new(rand::SeedableRng::seed_from_u64(100)));

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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

    fn new() -> AutoMoveMazeState {
        let mut points = vec![vec![0; H]; W];
        for i in 0..W {
            for j in 0..H {
                points[i][j] = RNG.lock().unwrap().gen_range(0..10);
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
        let mut tmp_state = self.clone();
        for character in &tmp_state.characters {
            let point = &mut tmp_state.points[character.x as usize][character.y as usize];
            *point = 0;
        }

        while !tmp_state.is_done() {
            tmp_state.advance();
            if is_print {
                tmp_state.to_string();
            }
        }

        tmp_state.game_score
    }

    fn init(&mut self) {
        for character in &mut self.characters {
            character.x = RNG.lock().unwrap().gen_range(0..W) as i32;
            character.y = RNG.lock().unwrap().gen_range(0..H) as i32;
        }
    }

    fn transition(&mut self) {
        let character = &mut self.characters[RNG.lock().unwrap().gen_range(0..CHARACTER_N)];
        character.x = RNG.lock().unwrap().gen_range(0..W) as i32;
        character.y = RNG.lock().unwrap().gen_range(0..H) as i32;
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
fn random_action(state: &mut AutoMoveMazeState) {
    for character_id in 0..CHARACTER_N {
        let x = RNG.lock().unwrap().gen_range(0..W) as i32;
        let y = RNG.lock().unwrap().gen_range(0..H) as i32;
        state.set_character(character_id, x, y);
    }
}

#[allow(dead_code)]
fn hill_climb(state: &mut AutoMoveMazeState, number: usize) {
    state.init();
    let mut best_score = state.get_score(false);
    for _ in 0..number {
        let mut next_state = state.clone();
        next_state.transition();
        let next_score = next_state.get_score(false);
        if next_score > best_score {
            best_score = next_score;
            *state = next_state;
        }
    }
}

fn play_game() {
    let mut state = AutoMoveMazeState::new();
    // random_action(&mut state);
    hill_climb(&mut state, 10000);
    println!("Score of hill climb : {}", state.get_score(false));
}

fn main() {
    play_game();
}
