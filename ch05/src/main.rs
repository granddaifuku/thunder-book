use std::cmp::Ordering;

use common::{get_random, init_random_generator, TimeKeeper};

use iterative_deepening::iterative_deepening_action;
use thunder::thunder_search_action_with_time_threshold;

const H: usize = 5;
const W: usize = 5;
const END_TURN: usize = 10;

struct Ai(String, Box<dyn Fn(&AlternateMazeState) -> usize>);

#[derive(Debug)]
enum WinningStatus {
    Win,
    Lose,
    Draw,
    None,
}

#[derive(Debug, Clone)]
struct Character {
    x: i32,
    y: i32,
    game_score: i32,
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

#[derive(Debug, Clone)]
pub struct AlternateMazeState {
    points: Vec<Vec<usize>>,
    turn: usize,
    characters: Vec<Character>,
}

impl AlternateMazeState {
    #[allow(non_upper_case_globals)]
    const dx: [i32; 4] = [1, -1, 0, 0];
    #[allow(non_upper_case_globals)]
    const dy: [i32; 4] = [0, 0, 1, -1];

    fn new() -> AlternateMazeState {
        let mut points = vec![vec![0; W]; H];
        for i in 0..H {
            for j in 0..W {
                if i == H / 2 && (j == W / 2 - 1 || j == W / 2 + 1) {
                    continue;
                }
                points[i][j] = get_random(10);
            }
        }

        AlternateMazeState {
            points,
            turn: 0,
            characters: vec![
                Character::new(H as i32 / 2, W as i32 / 2 - 1),
                Character::new(H as i32 / 2, W as i32 / 2 + 1),
            ],
        }
    }

    const fn is_first_player(&self) -> bool {
        self.turn % 2 == 0
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
            character.game_score += *point as i32;
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

    fn get_score(&self) -> i32 {
        self.characters[0].game_score - self.characters[1].game_score
    }

    fn get_score_rate(&self) -> f32 {
        if self.characters[0].game_score + self.characters[1].game_score == 0 {
            0.0
        } else {
            self.characters[0].game_score as f32
                / (self.characters[0].game_score + self.characters[1].game_score) as f32
        }
    }

    fn get_first_player_score_for_winning_rate(&self) -> f32 {
        match self.get_winning_status() {
            WinningStatus::Win => {
                if self.is_first_player() {
                    1.0
                } else {
                    0.0
                }
            }
            WinningStatus::Lose => {
                if self.is_first_player() {
                    0.0
                } else {
                    1.0
                }
            }
            _ => 0.5,
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
fn random_action(state: &AlternateMazeState) -> usize {
    let legal_actions = state.legal_actions();

    legal_actions[get_random(legal_actions.len())]
}

#[allow(dead_code)]
fn get_sample_states(game_number: usize) -> Vec<AlternateMazeState> {
    init_random_generator(0);

    let mut states = Vec::new();
    for _ in 0..game_number {
        let mut state = AlternateMazeState::new();
        let turn = get_random(usize::MAX) % END_TURN;
        for _ in 0..turn {
            state.advance(random_action(&state));
        }
        states.push(state);
    }

    states
}

#[allow(dead_code)]
fn calc_execution_speed(ai: &Ai, states: &Vec<AlternateMazeState>) {
    use std::time;

    let start_time = time::Instant::now();
    for state in states {
        ai.1(state);
    }
    let diff = time::Instant::now() - start_time;
    println!(
        "{} take {} ms to process {} nodes",
        ai.0,
        diff.as_millis(),
        states.len()
    );
}

#[allow(dead_code)]
fn test_first_player_win_rate(ais: Vec<Ai>, game_number: usize) {
    let mut first_player_win_rate = 0.0;
    for i in 0..game_number {
        init_random_generator(i as u64);

        let base_state = AlternateMazeState::new();
        for j in 0..2 {
            let mut state = base_state.clone();
            let first_ai = &ais[j];
            let second_ai = &ais[(j + 1) % 2];
            loop {
                state.advance(first_ai.1(&state));
                if state.is_done() {
                    break;
                }
                state.advance(second_ai.1(&state));
                if state.is_done() {
                    break;
                }
            }
            let mut win_rate_point = state.get_first_player_score_for_winning_rate();
            if j == 1 {
                win_rate_point = 1.0 - win_rate_point;
            }
            if win_rate_point >= 0.0 {
                state.to_string();
            }
            first_player_win_rate += win_rate_point;
        }
        println!("i {} w {}", i, first_player_win_rate / ((i + 1) * 2) as f32);
    }
    first_player_win_rate /= (game_number * 2) as f32;

    println!(
        "Winning rate of {} to {} : {}",
        &ais[0].0, &ais[1].0, first_player_win_rate
    );
}

#[allow(dead_code)]
fn play_game() {
    init_random_generator(0);

    let mut state = AlternateMazeState::new();
    state.to_string();
    while !state.is_done() {
        // Player 1
        {
            println!("Player 1 -----------------------");
            let act = minimax::minimax_action(&state, END_TURN) as usize;
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

#[allow(dead_code)]
mod minimax {
    use super::*;
    fn minimax_score(state: &AlternateMazeState, depth: usize) -> i32 {
        if state.is_done() || depth == 0 {
            return state.get_score();
        }
        let legal_actions = state.legal_actions();
        if legal_actions.is_empty() {
            return state.get_score();
        }

        let mut best_score = i32::MIN;
        for act in legal_actions {
            let mut next_state = state.clone();
            next_state.advance(act);
            let score = -minimax_score(&next_state, depth - 1);
            if score > best_score {
                best_score = score;
            }
        }

        best_score
    }

    pub fn minimax_action(state: &AlternateMazeState, depth: usize) -> i32 {
        let mut best_action = -1;
        let mut best_score = i32::MIN;
        for act in state.legal_actions() {
            let mut next_state = state.clone();
            next_state.advance(act);
            let score = -minimax_score(&next_state, depth);
            if score > best_score {
                best_action = act as i32;
                best_score = score;
            }
        }

        best_action
    }
}

#[allow(dead_code)]
mod alphabeta {
    use super::*;
    pub fn alphabeta_score(
        state: &AlternateMazeState,
        mut alpha: i32,
        beta: i32,
        depth: usize,
    ) -> i32 {
        if state.is_done() || depth == 0 {
            return state.get_score();
        }
        let legal_actions = state.legal_actions();
        if legal_actions.is_empty() {
            return state.get_score();
        }
        for act in legal_actions {
            let mut next_state = state.clone();
            next_state.advance(act);
            let score = -alphabeta_score(&next_state, -beta, -alpha, depth - 1);
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                return alpha;
            }
        }
        alpha
    }

    pub fn alphabeta_action(state: &AlternateMazeState, depth: usize) -> i32 {
        let mut best_action = -1;
        let mut alpha = -100000007;
        let beta = 100000007;
        for act in state.legal_actions() {
            let mut next_state = state.clone();
            next_state.advance(act);
            let score = -alphabeta_score(&next_state, -beta, -alpha, depth);
            if score > alpha {
                best_action = act as i32;
                alpha = score;
            }
        }
        best_action
    }
}

#[allow(dead_code)]
mod iterative_deepening {
    use super::{AlternateMazeState, TimeKeeper};
    fn alphabeta_score(
        state: &AlternateMazeState,
        mut alpha: i32,
        beta: i32,
        depth: usize,
        time_keeper: &TimeKeeper,
    ) -> i32 {
        if time_keeper.is_time_over() {
            return 0;
        }
        if state.is_done() || depth == 0 {
            return state.get_score();
        }
        let legal_actions = state.legal_actions();
        if legal_actions.is_empty() {
            return state.get_score();
        }
        for act in legal_actions {
            let mut next_state = state.clone();
            next_state.advance(act);
            let score = -alphabeta_score(&next_state, -beta, -alpha, depth - 1, time_keeper);
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                return alpha;
            }
            if time_keeper.is_time_over() {
                return 0;
            }
        }
        alpha
    }

    fn alpha_beta_action_with_time_threshold(
        state: &AlternateMazeState,
        depth: usize,
        time_keeper: &TimeKeeper,
    ) -> i32 {
        let mut best_action = -1;
        let mut alpha = -100000007;
        let beta = 100000007;
        for act in state.legal_actions() {
            let mut next_state = state.clone();
            next_state.advance(act);
            let score = -alphabeta_score(&next_state, -beta, -alpha, depth, time_keeper);
            if score > alpha {
                best_action = act as i32;
                alpha = score;
            }
            if time_keeper.is_time_over() {
                return 0;
            }
        }
        best_action
    }

    pub fn iterative_deepening_action(state: &AlternateMazeState, threshold: u128) -> usize {
        let time_keeper = TimeKeeper::new(threshold);
        let mut best_action = -1;
        let mut depth = 1;
        loop {
            let act = alpha_beta_action_with_time_threshold(state, depth, &time_keeper);
            if time_keeper.is_time_over() {
                break;
            }
            best_action = act;
            depth += 1;
        }
        best_action as usize
    }
}

#[allow(dead_code)]
mod montecarlo {
    const C: f32 = 1.0;
    const EXPAND_THRESHOLD: usize = 10;

    use super::{random_action, AlternateMazeState, TimeKeeper, WinningStatus};

    #[derive(Debug, Clone)]
    struct Node {
        state: AlternateMazeState,
        w: f32,
        n: usize,
        child_nodes: Vec<Node>,
    }

    impl Node {
        fn new(state: AlternateMazeState) -> Self {
            Self {
                state,
                w: 0.0,
                n: 0,
                child_nodes: Vec::new(),
            }
        }

        fn next_child_node(&mut self) -> usize {
            for (i, child_node) in self.child_nodes.iter().enumerate() {
                if child_node.n == 0 {
                    return i;
                }
            }
            let mut t = 0.0;
            for child_node in &self.child_nodes {
                t += child_node.n as f32;
            }
            let mut best_value = f32::MIN;
            let mut best_action_index = -1;
            for i in 0..self.child_nodes.len() {
                let child_node = &self.child_nodes[i];
                let ucb1_value = 1.0 - child_node.w / child_node.n as f32
                    + C * (2.0 * t.ln() / child_node.n as f32).sqrt();
                if ucb1_value > best_value {
                    best_value = ucb1_value;
                    best_action_index = i as i32;
                }
            }

            best_action_index as usize
        }

        fn expand(&mut self) {
            let legal_actions = self.state.legal_actions();
            self.child_nodes.clear();
            for act in legal_actions {
                self.child_nodes.push(Node::new(self.state.clone()));
                self.child_nodes.last_mut().unwrap().state.advance(act);
            }
        }

        fn evaluate(&mut self) -> f32 {
            if self.state.is_done() {
                let mut value = 0.5;
                match self.state.get_winning_status() {
                    WinningStatus::Win => value = 1.0,
                    WinningStatus::Lose => value = 0.0,
                    _ => {}
                }
                self.w += value;
                self.n += 1;
                return value;
            }

            if self.child_nodes.is_empty() {
                let mut state_copy = self.state.clone();
                let value = playout(&mut state_copy);
                self.w += value;
                self.n += 1;
                if self.n == EXPAND_THRESHOLD {
                    self.expand();
                }
                value
            } else {
                let next_child_node_index = self.next_child_node();
                let value = 1.0
                    - self
                        .child_nodes
                        .get_mut(next_child_node_index)
                        .unwrap()
                        .evaluate();
                self.w += value;
                self.n += 1;

                value
            }
        }

        fn print_tree(&self, depth: usize) {
            for (i, child_node) in self.child_nodes.iter().enumerate() {
                print!("{}", String::from("__").repeat(depth));
                println!(" {} ({})", i, child_node.n);
                if !child_node.child_nodes.is_empty() {
                    child_node.print_tree(depth + 1);
                }
            }
        }
    }

    pub fn mcts_action(state: &AlternateMazeState, playout_number: usize, is_print: bool) -> usize {
        use std::sync::atomic::AtomicBool;
        let mut root_node = Node::new(state.clone());
        root_node.expand();
        for _ in 0..playout_number {
            root_node.evaluate();
        }
        let legal_actions = state.legal_actions();
        let mut best_action_searched_number = -1;
        let mut best_action_index = -1;
        for i in 0..legal_actions.len() {
            let n = root_node.child_nodes[i].n as i32;
            if n > best_action_searched_number {
                best_action_index = i as i32;
                best_action_searched_number = n;
            }
        }
        {
            static mut CALLED_COUNT: AtomicBool = AtomicBool::new(false);
            unsafe {
                let called_count = CALLED_COUNT.get_mut();
                if is_print && !*called_count {
                    root_node.print_tree(0);
                }
                *called_count = true;
            }
        }

        legal_actions[best_action_index as usize]
    }

    pub fn mcts_action_with_time_threshold(state: &AlternateMazeState, threshold: u128) -> usize {
        let mut root_node = Node::new(state.clone());
        root_node.expand();
        let time_keeper = TimeKeeper::new(threshold);
        loop {
            if time_keeper.is_time_over() {
                break;
            }
            root_node.evaluate();
        }
        let legal_actions = state.legal_actions();
        let mut best_action_searched_number = -1;
        let mut best_action_index = -1;
        for i in 0..legal_actions.len() {
            let n = root_node.child_nodes[i].n as i32;
            if n > best_action_searched_number {
                best_action_index = i as i32;
                best_action_searched_number = n;
            }
        }

        legal_actions[best_action_index as usize]
    }

    pub fn primitive_montecarlo_action(state: &AlternateMazeState, playout_number: usize) -> usize {
        let legal_actions = state.legal_actions();
        let mut values = vec![0.0; legal_actions.len()];
        let mut counts = vec![0; legal_actions.len()];
        for count in 0..playout_number {
            let index = count % legal_actions.len();
            let mut next_state = state.clone();
            next_state.advance(legal_actions[index]);
            values[index] += 1.0 - playout(&mut next_state);
            counts[index] += 1;
        }

        let mut best_action_index = -1;
        let mut best_score = f32::MIN;
        for index in 0..legal_actions.len() {
            let value_mean = values[index] / counts[index] as f32;
            if value_mean > best_score {
                best_score = value_mean;
                best_action_index = index as i32;
            }
        }

        legal_actions[best_action_index as usize]
    }

    fn playout(state: &mut AlternateMazeState) -> f32 {
        match state.get_winning_status() {
            WinningStatus::Win => 1.0,
            WinningStatus::Lose => 0.0,
            WinningStatus::Draw => 0.5,
            WinningStatus::None => {
                state.advance(random_action(state));
                1.0 - playout(state)
            }
        }
    }
}

#[allow(dead_code)]
mod thunder {
    use super::{AlternateMazeState, TimeKeeper, WinningStatus};
    #[derive(Debug, Clone)]
    struct Node {
        state: AlternateMazeState,
        w: f32,
        n: usize,
        child_nodes: Vec<Node>,
    }

    impl Node {
        fn new(state: AlternateMazeState) -> Self {
            Self {
                state: state.clone(),
                w: 0.0,
                n: 0,
                child_nodes: Vec::new(),
            }
        }

        fn next_child_node(&mut self) -> usize {
            for (i, child_node) in self.child_nodes.iter().enumerate() {
                if child_node.n == 0 {
                    return i;
                }
            }
            let mut best_value = f32::MIN;
            let mut best_action_index = -1;
            for i in 0..self.child_nodes.len() {
                let child_node = &self.child_nodes[i];
                let thunder_value = 1.0 - child_node.w / child_node.n as f32;
                if thunder_value > best_value {
                    best_value = thunder_value;
                    best_action_index = i as i32;
                }
            }

            best_action_index as usize
        }

        fn expand(&mut self) {
            let legal_actions = self.state.legal_actions();
            self.child_nodes.clear();
            for act in legal_actions {
                self.child_nodes.push(Node::new(self.state.clone()));
                self.child_nodes.last_mut().unwrap().state.advance(act);
            }
        }

        fn evaluate(&mut self) -> f32 {
            if self.state.is_done() {
                let mut value = 0.5;
                match self.state.get_winning_status() {
                    WinningStatus::Win => value = 1.0,
                    WinningStatus::Lose => value = 0.0,
                    _ => {}
                }
                self.w += value;
                self.n += 1;
                return value;
            }
            if self.child_nodes.is_empty() {
                let value = self.state.get_score_rate();
                self.w += value;
                self.n += 1;
                self.expand();

                value
            } else {
                let next_child_node_index = self.next_child_node();
                let value = 1.0
                    - self
                        .child_nodes
                        .get_mut(next_child_node_index)
                        .unwrap()
                        .evaluate();
                self.w += value;
                self.n += 1;

                value
            }
        }
    }

    pub fn thunder_search_action(state: &AlternateMazeState, playout_number: usize) -> usize {
        let mut root_node = Node::new(state.clone());
        root_node.expand();
        for _ in 0..playout_number {
            root_node.evaluate();
        }
        let legal_actions = state.legal_actions();
        let mut best_action_searched_number = -1;
        let mut best_action_index = -1;
        for i in 0..legal_actions.len() {
            let n = root_node.child_nodes[i].n as i32;
            if n > best_action_searched_number {
                best_action_index = i as i32;
                best_action_searched_number = n;
            }
        }

        legal_actions[best_action_index as usize]
    }

    pub fn thunder_search_action_with_time_threshold(
        state: &AlternateMazeState,
        threshold: u128,
    ) -> usize {
        let mut root_node = Node::new(state.clone());
        root_node.expand();
        let time_keeper = TimeKeeper::new(threshold);
        loop {
            if time_keeper.is_time_over() {
                break;
            }
            root_node.evaluate();
        }
        let legal_actions = state.legal_actions();
        let mut best_action_searched_number = -1;
        let mut best_action_index = -1;
        for i in 0..legal_actions.len() {
            let n = root_node.child_nodes[i].n as i32;
            if n > best_action_searched_number {
                best_action_index = i as i32;
                best_action_searched_number = n;
            }
        }

        legal_actions[best_action_index as usize]
    }
}

fn main() {
    let ais = vec![
        Ai(
            String::from("thunderSearchActionWithTimeThreshold 1ms"),
            Box::new(|state| thunder_search_action_with_time_threshold(state, 1)),
        ),
        Ai(
            String::from("iterativeDeepening 1ms"),
            Box::new(|state| iterative_deepening_action(state, 1)),
        ),
    ];
    test_first_player_win_rate(ais, 100);
}
