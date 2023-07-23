use rand::Rng;

const H: i32 = 4;
const W: i32 = 3;
const END_TURN: i32 = 4;

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

#[derive(Debug)]
struct MazeState {
    points: Vec<Vec<i32>>,
    turn: i32,
    character: Coord,
    game_score: i32,
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

fn random_action(state: &MazeState) -> usize {
    let mut rng = rand::thread_rng();
    let act = state.legal_action();
    return act[rng.gen_range(0..act.len())];
}

fn play_game() {
    let mut state = MazeState::new();
    state.to_string();
    while !state.is_done() {
        state.advance(random_action(&state));
        state.to_string();
    }
}

fn main() {
    play_game();
}
