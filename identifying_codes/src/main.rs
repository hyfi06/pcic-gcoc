use bitmaps::Bitmap;
use hypercube;
use rand::prelude::*;

const DIM: usize = 3;
const SIZE: usize = 2_u32.pow(DIM as u32) as usize;
type MyBitmap = Bitmap<SIZE>;
type Graph = hypercube::Hypercube3;

#[derive(Debug, Clone)]
struct IdentifyingCodesProblem {
    graph: Graph,
    curr_choose: MyBitmap,
    next_choose: MyBitmap,
    best_choose: MyBitmap,
}

impl IdentifyingCodesProblem {
    fn update_best_choose(&mut self) {
        self.best_choose = self.next_choose.clone();
    }
    fn update_curr_choose(&mut self) {
        self.curr_choose = self.next_choose.clone();
    }
    fn update_next_choose(&mut self) {
        let mut rng = rand::rng();
        let distance = rng.random_range(1..=3);
        self.next_choose = self.curr_choose.clone();
        for _ in 0..distance {
            let idx = rng.random_range(0..SIZE);
            self.next_choose.set(idx, !self.next_choose.get(idx));
        }
    }
    fn is_valid_next_choose(&self) {
        
    }
    fn best_choose_score(&self) -> u32 {
        self.best_choose.len() as u32
    }
    fn curr_choose_score(&self) -> u32 {
        self.curr_choose.len() as u32
    }
    fn next_choose_score(&self) -> u32 {
        self.next_choose.len() as u32
    }

    fn simulated_annealing(&self, initial_temp: f64, cooling_rate: f64, max_iterations: usize) {
        let mut rng = rand::rng();
        let mut temperature = initial_temp;
        
        for _ in 0..max_iterations {
            self.best_choose_score();
            let new_energy = objective_function(new_x);
            let delta_energy = new_energy - current_energy;
    
            if delta_energy < 0.0 || rng.gen::<f64>() < (-delta_energy / temperature).exp() {
                current_x = new_x;
                current_energy = new_energy;
                if current_energy < best_energy {
                    best_x = current_x;
                    best_energy = current_energy;
                }
            }
    
            temperature *= cooling_rate;
        }
    }

}

fn main() {
    Bitmap::<2>::new().as_value();
}
