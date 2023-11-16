use common::{get_random, init_random_generator};

use std::cmp::Ordering;

const H: i32 = 5;
const W: i32 = 5;
const END_TURN: u32 = 4;

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

#[derive(Debug, Clone, Eq, PartialEq)]
struct WallMazeState {
    points: Vec<Vec<u32>>,
    turn: u32,
    walls: Vec<Vec<bool>>,
    character: Coord,
    game_score: u32,
    evaluated_score: u32,
    first_action: u32,
}

impl Ord for WallMazeState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.evaluated_score
            .cmp(&other.evaluated_score)
            .then_with(|| self.turn.cmp(&other.turn))
    }
}

impl PartialOrd for WallMazeState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl WallMazeState {
    #[allow(non_upper_case_globals)]
    const dx: [i32; 4] = [1, -1, 0, 0];
    #[allow(non_upper_case_globals)]
    const dy: [i32; 4] = [0, 0, 1, -1];

    fn new() -> Self {
        let x = get_random(W as usize) as i32;
        let y = get_random(H as usize) as i32;
        let character = Coord::new(x, y);
        let mut points = vec![vec![0; H as usize]; W as usize];
        let mut walls = vec![vec![false; H as usize]; W as usize];

        for x in (1..W).step_by(2) {
            for y in (1..H).step_by(2) {
                let mut tx = x;
                let mut ty = y;
                if tx == character.x && ty == character.y {
                    continue;
                }
                walls[tx as usize][ty as usize] = true;
                let mut direction_size = 3;
                if x == 1 {
                    direction_size = 4;
                }
                let direction = get_random(direction_size);
                tx += Self::dx[direction];
                ty += Self::dy[direction];
                if tx == character.x && ty == character.y {
                    continue;
                }
                walls[tx as usize][ty as usize] = true;
            }
        }

        for i in 0..W {
            for j in 0..H {
                if i == character.x && j == character.y {
                    continue;
                }
                points[i as usize][j as usize] = get_random(10) as u32;
            }
        }

        Self {
            points,
            turn: 0,
            walls,
            character,
            game_score: 0,
            evaluated_score: 0,
            first_action: 0,
        }
    }

    const fn is_done(&self) -> bool {
        self.turn == END_TURN
    }

    fn legal_action(&self) -> Vec<usize> {
        let mut actions = Vec::new();
        for i in 0..4 {
            let nx = self.character.x + Self::dx[i];
            let ny = self.character.y + Self::dy[i];
            if !(0..W).contains(&nx)
                || !(0..H).contains(&ny)
                || self.walls[nx as usize][ny as usize]
            {
                continue;
            }
            actions.push(i);
        }

        actions
    }

    #[allow(dead_code)]
    fn evaluate_score(&mut self) {
        self.evaluated_score = self.game_score;
    }

    fn advance(&mut self, action: usize) {
        self.character.x += Self::dx[action];
        self.character.y += Self::dy[action];
        let point = &mut self.points[self.character.x as usize][self.character.y as usize];
        if *point > 0 {
            self.game_score += *point;
            *point = 0;
        }
        self.turn += 1;
    }

    fn to_string(&self) {
        println!("turn: {}", self.turn);
        println!("score: {}", self.game_score);
        for i in 0..W {
            for j in 0..H {
                if self.walls[i as usize][j as usize] {
                    print!("#");
                } else if i == self.character.x && j == self.character.y {
                    print!("@");
                } else if self.points[i as usize][j as usize] > 0 {
                    print!("{}", self.points[i as usize][j as usize]);
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn random_action(state: &WallMazeState) -> usize {
    let acts = state.legal_action();

    acts[get_random(acts.len())]
}

fn play_game() {
    let mut state = WallMazeState::new();
    state.to_string();
    while !state.is_done() {
        state.advance(random_action(&state));
        state.to_string();
    }
}

fn main() {
    init_random_generator(0);
    play_game();
}
