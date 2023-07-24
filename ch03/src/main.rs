use rand::Rng;

const H: i32 = 4;
const W: i32 = 3;
const END_TURN: i32 = 4;

#[derive(Debug, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

#[derive(Debug, Clone)]
struct MazeState {
    points: Vec<Vec<i32>>,
    turn: i32,
    character: Coord,
    game_score: i32,
    evaluated_score: i32,
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
            if nx < 0 || nx >= W || ny < 0 || ny >= H {
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
    return acts[rng.gen_range(0..acts.len())];
}

#[allow(dead_code)]
fn greedy_action(state: &MazeState) -> usize {
    let acts = state.legal_action();
    let mut best_action = -1 as i32;
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

fn play_game() -> i32 {
    let mut state = MazeState::new();
    state.to_string();
    while !state.is_done() {
        state.advance(greedy_action(&state));
        state.to_string();
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
