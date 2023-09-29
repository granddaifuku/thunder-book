use rand::Rng;
use std::sync::Mutex;

use once_cell::sync::Lazy;

const H: usize = 3;
const W: usize = 3;
const END_TURN: usize = 4;
#[allow(non_upper_case_globals)]
const dstr: &[&str] = &["DOWN", "UP", "RIGHT", "LEFT"];

static RNG: Lazy<Mutex<rand::rngs::StdRng>> =
    Lazy::new(|| Mutex::new(rand::SeedableRng::seed_from_u64(0)));

fn get_random(limit: usize) -> usize {
    RNG.lock().unwrap().gen_range(0..limit)
}

struct Ai(String, Box<dyn Fn(&SimultaneousMazeState) -> usize>);

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
struct SimultaneousMazeState {
    points: Vec<Vec<usize>>,
    turn: usize,
    characters: Vec<Character>,
}

impl SimultaneousMazeState {
    #[allow(non_upper_case_globals)]
    const dx: [i32; 4] = [1, -1, 0, 0];
    #[allow(non_upper_case_globals)]
    const dy: [i32; 4] = [0, 0, 1, -1];

    fn new(seed: u64) -> Self {
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
        let mut points = vec![vec![0; H]; W];

        for i in 0..H {
            for j in 0..W / 2 + 1 {
                let point = rng.gen_range(0..10);
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
fn random_action(state: &SimultaneousMazeState, player_id: usize) -> usize {
    let legal_actions = state.legal_actions(player_id);

    legal_actions[get_random(legal_actions.len())]
}

fn play_game(ais: Vec<Ai>, seed: u64) {
    let mut state = SimultaneousMazeState::new(seed);
    state.to_string();

    while !state.is_done() {
        let actions = (&ais[0].1(&state), &ais[1].1(&state));
        println!("actions {} {}", dstr[*actions.0], dstr[*actions.1]);
        state.advance(*actions.0, *actions.1);
        state.to_string();
    }
}

fn main() {
    let ais = vec![
        Ai(
            String::from("randomAction"),
            Box::new(|state| random_action(state, 0)),
        ),
        Ai(
            String::from("randomAction"),
            Box::new(|state| random_action(state, 1)),
        ),
    ];

    play_game(ais, 0);
}
