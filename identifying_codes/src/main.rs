use std::{f32::consts::PI, ops::BitOr};

use bitmaps::Bitmap;
use hypercube;
use rand::prelude::*;

const DIM: usize = 8;
const SIZE: usize = 2_u32.pow(DIM as u32) as usize;
type MyBitmap = Bitmap<SIZE>;
type Graph = hypercube::Hypercube8;

#[derive(Debug, Clone)]
struct IdentifyingCodesProblem {
    graph: Graph,
    curr_choose: MyBitmap,
    next_choose: MyBitmap,
    best_choose: MyBitmap,
    seen_vertices: usize,
    seen_leaves: usize,
    level: usize,
    pool: Vec<(u32, MyBitmap)>,
}

impl IdentifyingCodesProblem {
    fn new(graph: Graph) -> Self {
        let mut graph = graph;
        for i in 0..graph.adjacency.len() {
            graph.adjacency[i].set(i, true);
            graph.edges.push((i, i));
        }
        Self {
            graph,
            curr_choose: MyBitmap::mask(SIZE),
            next_choose: MyBitmap::mask(SIZE),
            best_choose: MyBitmap::mask(SIZE),
            seen_vertices: 0,
            seen_leaves: 0,
            level: 0,
            pool: Vec::new(),
        }
    }
    fn dominate_next(&self) -> bool {
        self.next_choose
            .into_iter()
            .fold(MyBitmap::new(), |acc, curr| {
                acc.bitor(self.graph.adjacency[curr])
            })
            == MyBitmap::mask(SIZE)
    }
    fn separate_next(&self) -> bool {
        for i in 0..self.graph.adjacency.len() {
            for j in (i + 1)..self.graph.adjacency.len() {
                let code_i = self.next_choose & self.graph.adjacency[i];
                let code_j = self.next_choose & self.graph.adjacency[j];
                if code_i == code_j {
                    return false;
                }
            }
        }
        true
    }
}

impl IdentifyingCodesProblem {
    fn replace_best_with_next(&mut self) {
        self.best_choose = self.next_choose.clone();
    }
    fn replace_curr_with_next(&mut self) {
        self.curr_choose = self.next_choose.clone();
    }
    fn push_next_to_pool(&mut self) {
        self.pool
            .push((self.next_score(), self.next_choose.clone()));
    }
    fn update_next(&mut self) {
        let mut rng = rand::rng();
        let distance = rng.random_range(1..=3);
        self.next_choose = self.curr_choose.clone();
        for _ in 0..distance {
            let idx = rng.random_range(0..SIZE);
            self.next_choose.set(idx, !self.next_choose.get(idx));
        }
    }
    fn is_next_valid(&self) -> bool {
        self.dominate_next() && self.separate_next()
    }
    fn best_score(&self) -> u32 {
        self.best_choose.len() as u32
    }
    fn curr_score(&self) -> u32 {
        self.curr_choose.len() as u32
    }
    fn next_score(&self) -> u32 {
        self.next_choose.len() as u32
    }
    fn simulated_annealing(&mut self, initial_temp: f64, cooling_rate: f64, max_iterations: usize) {
        let mut rng = rand::rng();
        let mut temperature = initial_temp;

        for _ in 0..max_iterations {
            self.update_next();
            let next_choose_score = self.next_score() as f64;
            let curr_choose_score = self.curr_score() as f64;
            let delta_score = next_choose_score - curr_choose_score;
            let p = (-delta_score / temperature as f64).exp();
            if self.is_next_valid() && (next_choose_score < curr_choose_score || rng.random_bool(p))
            {
                self.replace_curr_with_next();
                if next_choose_score <= self.best_score().into() {
                    self.push_next_to_pool()
                }
                if next_choose_score < self.best_score().into() {
                    self.replace_best_with_next();
                    println!(
                        "Temp: {:.4}, P {:.4}, Best: {:?} Score: {}",
                        temperature,
                        p,
                        self.best_choose,
                        self.best_score()
                    );
                }
            }
            temperature *= cooling_rate;
        }
    }
}

impl IdentifyingCodesProblem {
    fn update_count_seen_vertices(&mut self) {
        self.seen_vertices += 1;
    }
    fn is_leaf(&self) -> bool {
        self.level == self.graph.adjacency.len()
    }
    fn set(&mut self, idx: usize) {
        self.next_choose.set(idx, false);
    }
    fn unset(&mut self, idx: usize) {
        self.next_choose.set(idx, true);
    }
    fn available_chooses(&self) -> MyBitmap {
        let level = self.next_choose.last_false_index().unwrap_or(0);
        let mut result = self.next_choose.clone();
        for i in 0..level {
            result.set(i, false);
        }
        result
    }
    fn minimal_backtrack(&mut self, level: usize) {
        self.update_count_seen_vertices();
        if self.next_score() < self.best_score() {
            self.replace_best_with_next();
        }
        for idx in self.available_chooses().into_iter() {
            self.set(idx);
            if self.is_next_valid() {
                self.minimal_backtrack(self.next_choose.last_false_index().unwrap_or(level));
            }
            self.unset(idx)
        }
    }
}

fn main() {
    let mut problem = IdentifyingCodesProblem::new(Graph::new());
    problem.simulated_annealing(10_000.0, 0.9999, 1000_000);
    println!(
        "Best:{:?}, Score: {}",
        problem.best_choose.as_value(),
        problem.best_score()
    );
    println!(
        "{:?}",
        problem
            .pool
            .iter()
            .filter(|(score, _)| *score <= problem.best_score() + 2)
    );
}
