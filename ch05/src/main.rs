use std::{cmp::Ordering, sync::Mutex};

use once_cell::sync::Lazy;
use rand::Rng;

const H: usize = 3;
const W: usize = 3;
const END_TURN: usize = 4;

static RNG: Lazy<Mutex<rand::rngs::StdRng>> =
    Lazy::new(|| Mutex::new(rand::SeedableRng::seed_from_u64(0)));

fn get_random(limit: usize) -> usize {
    RNG.lock().unwrap().gen_range(0..limit)
}

#[derive(Debug)]
enum WinningStatus {
    Win,
    Lose,
    Draw,
    None,
}

#[derive(Debug)]
struct Character {
    x: i32,
    y: i32,
    game_score: usize,
}

impl Character {
    fn new(x: i32, y: i32) -> Character {
        Character {
            x,
            y,
            game_score: 0,
        }
    }
}

#[derive(Debug)]
struct AlternamteMazeState {
    points: Vec<Vec<usize>>,
    turn: usize,
    characters: Vec<Character>,
}

impl AlternamteMazeState {
    #[allow(non_upper_case_globals)]
    const dx: [i32; 4] = [1, -1, 0, 0];
    #[allow(non_upper_case_globals)]
    const dy: [i32; 4] = [0, 0, 1, -1];

    fn new() -> AlternamteMazeState {
        let mut points = vec![vec![0; W]; H];
        for i in 0..H {
            for j in 0..W {
                if i == H / 2 && (j == W / 2 - 1 || j == W / 2 + 1) {
                    continue;
                }
                points[i][j] = get_random(10);
            }
        }

        AlternamteMazeState {
            points,
            turn: 0,
            characters: vec![
                Character::new(H as i32 / 2, W as i32 / 2 - 1),
                Character::new(H as i32 / 2, W as i32 / 2 + 1),
            ],
        }
    }

    const fn is_done(&self) -> bool {
        self.turn == END_TURN
    }

    fn advance(&mut self, action: usize) {
        let character = &mut self.characters[0];
        character.x += Self::dx[action];
        character.y += Self::dy[action];

        let point = &mut self.points[character.x as usize][character.y as usize];
        if *point > 0 {
            character.game_score += *point;
            *point = 0;
        }
        self.turn += 1;
        self.characters.swap(0, 1);
    }

    fn legal_actions(&self) -> Vec<usize> {
        let mut actions = Vec::new();
        let character = &self.characters[0];
        for i in 0..4 {
            let nx = character.x + Self::dx[i];
            let ny = character.y + Self::dy[i];
            if nx >= 0 && nx < H as i32 && ny >= 0 && ny < W as i32 {
                actions.push(i);
            }
        }

        actions
    }

    fn get_winning_status(&self) -> WinningStatus {
        if self.is_done() {
            let score_0 = self.characters[0].game_score;
            let score_1 = self.characters[1].game_score;
            match score_0.cmp(&score_1) {
                Ordering::Greater => WinningStatus::Win,
                Ordering::Less => WinningStatus::Lose,
                Ordering::Equal => WinningStatus::Draw,
            }
        } else {
            WinningStatus::None
        }
    }

    fn to_string(&self) {
        println!("turn: {}", self.turn);
        // Print the score and the position
        for player_id in 0..self.characters.len() {
            let mut actual_player_id = player_id;
            // If the turn is odd, print the view from the opponent side.
            if self.turn % 2 == 1 {
                actual_player_id = (player_id + 1) % 2;
            }
            let character = &self.characters[actual_player_id];
            println!("score ({}): {}", player_id, character.game_score);
            println!("x: {}, y: {}", character.x, character.y);
        }

        for h in 0..H {
            for w in 0..W {
                let mut is_written = false;
                for (player_id, character) in self.characters.iter().enumerate() {
                    let mut actual_player_id = player_id;
                    // If the turn is odd, print the view from the opponent side.
                    if self.turn % 2 == 1 {
                        actual_player_id = (player_id + 1) % 2;
                    }

                    if character.x as usize == h && character.y as usize == w {
                        if actual_player_id == 0 {
                            print!("A");
                        } else {
                            print!("B");
                        }
                        is_written = true;
                    }
                }
                if !is_written {
                    if self.points[h][w] > 0 {
                        print!("{}", self.points[h][w]);
                    } else {
                        print!(".");
                    }
                }
            }
            println!();
        }
        println!();
    }
}

#[allow(dead_code)]
fn random_action(state: &AlternamteMazeState) -> usize {
    let legal_actions = state.legal_actions();

    legal_actions[get_random(legal_actions.len())]
}

fn play_game() {
    let mut state = AlternamteMazeState::new();
    state.to_string();
    while !state.is_done() {
        // Player 1
        {
            println!("Player 1 -----------------------");
            let act = random_action(&state);
            println!("action: {}", act);
            state.advance(act);
            state.to_string();
            if state.is_done() {
                match state.get_winning_status() {
                    WinningStatus::Win => println!("Winner: Player 2"),
                    WinningStatus::Lose => println!("Winner: Player 1"),
                    _ => println!("Draw"),
                }
                break;
            }
        }
        // Player 2
        {
            println!("Player 2 -----------------------");
            let act = random_action(&state);
            println!("action: {}", act);
            state.advance(act);
            state.to_string();
            if state.is_done() {
                match state.get_winning_status() {
                    WinningStatus::Win => println!("Winner: Player 1"),
                    WinningStatus::Lose => println!("Winner: Player 2"),
                    _ => println!("Draw"),
                }
                break;
            }
        }
    }
}

fn main() {
    play_game();
}
