use rand::Rng;
use std::{cmp::Ordering, sync::Mutex};

use once_cell::sync::Lazy;

use montecarlo::primitive_montecarlo_action;

const H: usize = 5;
const W: usize = 5;
const END_TURN: usize = 20;
#[allow(non_upper_case_globals)]
const dstr: &[&str] = &["DOWN", "UP", "RIGHT", "LEFT"];

#[derive(Debug)]
struct RandomGenerator {
    rng: rand::rngs::StdRng,
}

impl RandomGenerator {
    fn new() -> Self {
        Self {
            rng: rand::SeedableRng::seed_from_u64(0),
        }
    }

    fn set_rng(&mut self, seed: u64) {
        self.rng = rand::SeedableRng::seed_from_u64(seed);
    }

    fn gen_range(&mut self, limit: usize) -> usize {
        self.rng.gen_range(0..limit)
    }
}

static GENERATOR: Lazy<Mutex<RandomGenerator>> = Lazy::new(|| Mutex::new(RandomGenerator::new()));

fn init_random_generator(seed: u64) {
    GENERATOR.lock().unwrap().set_rng(seed);
}

fn get_random(limit: usize) -> usize {
    GENERATOR.lock().unwrap().gen_range(limit)
}

struct Ai(String, Box<dyn Fn(&SimultaneousMazeState) -> usize>);

#[derive(Debug)]
enum WinningStatus {
    First,
    Second,
    Draw,
    None,
}

#[derive(Debug, Clone, Copy)]
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

    fn advance(&mut self, dx: i32, dy: i32, points: &[Vec<usize>]) {
        self.x += dx;
        self.y += dy;
        self.game_score += points[self.x as usize][self.y as usize];
    }
}

#[derive(Debug, Clone)]
pub struct SimultaneousMazeState {
    points: Vec<Vec<usize>>,
    turn: usize,
    characters: Vec<Character>,
}

impl SimultaneousMazeState {
    #[allow(non_upper_case_globals)]
    const dx: [i32; 4] = [1, -1, 0, 0];
    #[allow(non_upper_case_globals)]
    const dy: [i32; 4] = [0, 0, 1, -1];

    fn new() -> Self {
        let mut points = vec![vec![0; H]; W];

        for i in 0..H {
            for j in 0..W / 2 + 1 {
                let point = get_random(10);
                if i == H / 2 && (j == W / 2 - 1 || j == W / 2 + 1) {
                    continue;
                }
                let mut tj = j;
                points[i][tj] = point;
                tj = W - 1 - j;
                points[i][tj] = point;
            }
        }
        Self {
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

    fn advance(&mut self, action0: usize, action1: usize) {
        self.characters.get_mut(0).unwrap().advance(
            Self::dx[action0],
            Self::dy[action0],
            &self.points,
        );
        self.characters.get_mut(1).unwrap().advance(
            Self::dx[action1],
            Self::dy[action1],
            &self.points,
        );
        for character in &self.characters {
            self.points[character.x as usize][character.y as usize] = 0;
        }
        self.turn += 1;
    }

    fn legal_actions(&self, player_id: usize) -> Vec<usize> {
        let mut actions = Vec::new();
        let character = &self.characters[player_id];
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
                Ordering::Greater => WinningStatus::First,
                Ordering::Less => WinningStatus::Second,
                Ordering::Equal => WinningStatus::Draw,
            }
        } else {
            WinningStatus::None
        }
    }

    fn get_first_player_score_for_winning_rate(&self) -> f32 {
        match self.get_winning_status() {
            WinningStatus::First => 1.0,
            WinningStatus::Second => 0.0,
            _ => 0.5,
        }
    }

    fn to_string(&self) {
        println!("turn: {}", self.turn);
        for player_id in 0..self.characters.len() {
            println!(
                "score({}): {}",
                player_id, self.characters[player_id].game_score
            );
        }
        for h in 0..H {
            for w in 0..W {
                let mut is_written = false;
                for player_id in 0..self.characters.len() {
                    let character = &self.characters[player_id];
                    if character.x as usize == h && character.y as usize == w {
                        if player_id == 0 {
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
mod montecarlo {
    use crate::get_random;

    use super::{random_action, SimultaneousMazeState, WinningStatus};

    // The view from the player0
    fn playout(state: &mut SimultaneousMazeState) -> f32 {
        match state.get_winning_status() {
            WinningStatus::First => 1.0,
            WinningStatus::Second => 0.0,
            WinningStatus::Draw => 0.5,
            WinningStatus::None => {
                state.advance(random_action(state, 0), random_action(state, 1));
                playout(state)
            }
        }
    }

    pub fn primitive_montecarlo_action(
        state: &SimultaneousMazeState,
        player_id: usize,
        playout_number: usize,
    ) -> usize {
        let my_legal_actions = state.legal_actions(player_id);
        let opp_legal_actions = state.legal_actions((player_id + 1) % 2);
        let mut best_value = f32::MIN;
        let mut best_action_index = -1;

        for (i, act) in my_legal_actions.iter().enumerate() {
            let mut value = 0.0;
            for _ in 0..playout_number {
                let mut next_state = state.clone();
                if player_id == 0 {
                    next_state
                        .advance(*act, opp_legal_actions[get_random(opp_legal_actions.len())]);
                } else {
                    next_state
                        .advance(opp_legal_actions[get_random(opp_legal_actions.len())], *act);
                }
                let player0_win_rate = playout(&mut next_state);
                let win_rate = if player_id == 0 {
                    player0_win_rate
                } else {
                    1.0 - player0_win_rate
                };
                value += win_rate;
            }
            if value > best_value {
                best_value = value;
                best_action_index = i as i32;
            }
        }

        my_legal_actions[best_action_index as usize]
    }
}

#[allow(dead_code)]
fn random_action(state: &SimultaneousMazeState, player_id: usize) -> usize {
    let legal_actions = state.legal_actions(player_id);

    legal_actions[get_random(legal_actions.len())]
}

#[allow(dead_code)]
fn play_game(ais: Vec<Ai>) {
    let mut state = SimultaneousMazeState::new();
    state.to_string();

    while !state.is_done() {
        let actions = (&ais[0].1(&state), &ais[1].1(&state));
        println!("actions {} {}", dstr[*actions.0], dstr[*actions.1]);
        state.advance(*actions.0, *actions.1);
        state.to_string();
    }
}

#[allow(dead_code)]
fn test_first_player_win_rate(ais: Vec<Ai>, game_number: usize) {
    let mut first_player_win_rate = 0.0;
    for i in 0..game_number {
        init_random_generator(i as u64);

        let mut state = SimultaneousMazeState::new();
        let first_ai = &ais[0];
        let second_ai = &ais[1];
        loop {
            state.advance(first_ai.1(&state), second_ai.1(&state));
            if state.is_done() {
                break;
            }
        }
        let win_rate_point = state.get_first_player_score_for_winning_rate();
        if win_rate_point >= 0.0 {
            state.to_string()
        }
        first_player_win_rate += win_rate_point;

        println!("i {} w {}", i, first_player_win_rate / (i + 1) as f32);
    }
    first_player_win_rate /= game_number as f32;
    println!(
        "Winning rate of {} to {} : {}",
        &ais[0].0, &ais[1].0, first_player_win_rate
    );
}

fn main() {
    let ais = vec![
        Ai(
            String::from("primitiveMotecarloAction"),
            Box::new(|state| primitive_montecarlo_action(state, 0, 1000)),
        ),
        Ai(
            String::from("randomAction"),
            Box::new(|state| random_action(state, 1)),
        ),
    ];

    test_first_player_win_rate(ais, 500);
}
