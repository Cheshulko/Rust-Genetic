extern crate rand;

use rand::{thread_rng, Rng};

pub mod city_point;
pub mod utils;
mod solution;
mod simulation;

pub use city_point::CityPoint;
pub use solution::Solution;
pub use simulation::Simulation;

pub fn random_dna(n: usize) -> Vec<usize> {
    let mut v:Vec<usize> = (0..n).collect();
    thread_rng().shuffle(&mut v);
    v
}

pub fn select_parents<'a>(w: &[f64], solutions: &'a [Solution]) -> (&'a Solution, &'a Solution) {
    let mom_index = utils::choose_ind(w);
    let dad_index = utils::choose_ind(w);  
    (&solutions[mom_index], &solutions[dad_index])
}

pub fn find_fittest(current_population: &[Solution]) -> Solution {

    let mut best_individual = &current_population[0];
    
    for individual in current_population {
        if best_individual.fitness_value > individual.fitness_value {
            best_individual = individual;
        }
    }
    best_individual.clone()
}

pub fn get_cumulative_weights(solutions: &[Solution]) -> Vec<f64> {

    let mut running_sum = 0.0;
    let mut weights_sum = vec![running_sum];

    for i in solutions {
        running_sum += i.fitness_value;
        weights_sum.push(running_sum);
    }
    weights_sum
}

pub fn random_population(size_of_population: usize, cities: &[CityPoint]) -> Vec<Solution> {

    let cities_number = cities.len();
    let mut solutions:Vec<Solution> = Vec::new();
    
    for _ in 0..size_of_population {
        let row_of_dna = random_dna(cities_number);
        let indiv = Solution::generate(row_of_dna, &cities);
        solutions.push(indiv);
    } 
    solutions
}