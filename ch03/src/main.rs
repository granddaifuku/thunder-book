use rand::Rng;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;

const H: i32 = 30;
const W: i32 = 30;
const END_TURN: i32 = 100;

struct TimeKeeper {
    start_time: Instant,
    threshold: u128,
}

impl TimeKeeper {
    pub fn new(threshold: u128) -> TimeKeeper {
        TimeKeeper {
            start_time: Instant::now(),
            threshold,
        }
    }

    pub fn is_time_over(&self) -> bool {
        (Instant::now() - self.start_time).as_millis() >= self.threshold
    }
}

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
struct MazeState {
    points: Vec<Vec<i32>>,
    turn: i32,
    character: Coord,
    game_score: i32,
    evaluated_score: i32,
    first_action: usize,
}

impl Ord for MazeState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.evaluated_score
            .cmp(&other.evaluated_score)
            .then_with(|| self.turn.cmp(&other.turn))
    }
}

impl PartialOrd for MazeState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl MazeState {
    #[allow(non_upper_case_globals)]
    const dx: [i32; 4] = [1, -1, 0, 0];
    #[allow(non_upper_case_globals)]
    const dy: [i32; 4] = [0, 0, 1, -1];
    pub fn new() -> MazeState {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..W);
        let y = rng.gen_range(0..H);
        let character = Coord::new(x, y);
        let mut points = vec![vec![0; H as usize]; W as usize];

        for i in 0..W {
            for j in 0..H {
                if i == x && j == y {
                    continue;
                }
                points[i as usize][j as usize] = rng.gen_range(0..10);
            }
        }

        MazeState {
            points,
            turn: 0,
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
            if !(0..W).contains(&nx) || !(0..H).contains(&ny) {
                continue;
            }
            actions.push(i);
        }

        actions
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

    fn evaluate_score(&mut self) {
        self.evaluated_score = self.game_score;
    }

    #[allow(dead_code)]
    fn to_string(&self) {
        println!("turn: {}", self.turn);
        println!("score: {}", self.game_score);
        for i in 0..W {
            for j in 0..H {
                if i == self.character.x && j == self.character.y {
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

#[allow(dead_code)]
fn random_action(state: &MazeState) -> usize {
    let mut rng = rand::thread_rng();
    let acts = state.legal_action();

    acts[rng.gen_range(0..acts.len())]
}

#[allow(dead_code)]
fn greedy_action(state: &MazeState) -> usize {
    let acts = state.legal_action();
    let mut best_action = -1_i32;
    let mut best_score = -1e9 as i32;
    for act in acts {
        let mut now = state.clone();
        now.advance(act);
        now.evaluate_score();
        if now.evaluated_score > best_score {
            best_score = now.evaluated_score;
            best_action = act as i32;
        }
    }

    best_action as usize
}

#[allow(dead_code)]
fn beam_search_action(state: &MazeState, beam_width: i32, threshold: u128) -> usize {
    let mut now_beam = BinaryHeap::new();
    let mut best_state = state.clone();
    let time_keeper = TimeKeeper::new(threshold);
    let mut t = true;

    now_beam.push(state.clone());
    loop {
        let mut next_beam = BinaryHeap::new();
        for _ in 0..beam_width {
            if time_keeper.is_time_over() {
                return best_state.first_action;
            }

            if now_beam.is_empty() {
                break;
            }

            let now_state = now_beam.pop().unwrap();
            let legal_actions = now_state.legal_action();
            for act in legal_actions {
                let mut next_state = now_state.clone();
                next_state.advance(act);
                next_state.evaluate_score();
                if t {
                    next_state.first_action = act;
                }
                next_beam.push(next_state);
            }
            t = false;
        }

        now_beam = next_beam;
        best_state = now_beam.peek().unwrap().clone();

        if best_state.is_done() {
            break;
        }
    }

    best_state.first_action
}

#[allow(dead_code)]
fn chokudai_search_action(
    state: &MazeState,
    beam_width: usize,
    beam_depth: usize,
    threshold: u128,
) -> usize {
    let time_keeper = TimeKeeper::new(threshold);
    let mut beam = vec![BinaryHeap::new(); beam_depth + 1];
    beam[0].push(state.clone());
    loop {
        for t in 0..beam_depth {
            let mut now_beam = beam.get(t).unwrap().clone();

            for _ in 0..beam_width {
                if now_beam.is_empty() {
                    break;
                }
                let now_state = now_beam.peek().unwrap().clone();
                if now_state.is_done() {
                    break;
                }
                now_beam.pop();
                let legal_actions = now_state.legal_action();
                for act in legal_actions {
                    let mut next_state = now_state.clone();
                    next_state.advance(act);
                    next_state.evaluate_score();
                    if t == 0 {
                        next_state.first_action = act;
                    }
                    beam[t + 1].push(next_state);
                }
            }
            beam[t] = now_beam;
        }
        if time_keeper.is_time_over() {
            break;
        }
    }
    for t in (0..beam_depth + 1).rev() {
        let now_beam = beam.get(t).unwrap();
        if !now_beam.is_empty() {
            return now_beam.peek().unwrap().first_action;
        }
    }

    0
}

fn play_game() -> i32 {
    let mut state = MazeState::new();
    // state.to_string();
    while !state.is_done() {
        state.advance(chokudai_search_action(&state, 1, END_TURN as usize, 1));
        // state.to_string();
    }
    state.game_score
}

fn test_ai_score(game_number: usize) {
    let mut score_mean = 0.0;
    for _ in 0..game_number {
        score_mean += play_game() as f64;
    }
    score_mean /= game_number as f64;
    println!("Score: {:.2}", score_mean);
}

fn main() {
    test_ai_score(100);
}
