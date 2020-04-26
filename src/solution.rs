use rand::{thread_rng, Rng};
use super::CityPoint;

pub const MIN: f64 = 2.2250738585072014e-308f64;

#[derive(Debug, Clone)]
pub struct Solution {
    pub row_of_dna: Vec<usize>,
    pub fitness_value: f64
}

impl Solution {

    pub fn generate(row_of_dna: Vec<usize>, cities: &[CityPoint]) -> Self {
        let fitness_value = calc_fitness_value(&row_of_dna, &cities);
        Solution { row_of_dna, fitness_value }
    }

    pub fn crossover(&self, second: &Solution, cities: &[CityPoint]) -> (Self, Self) {

        let length = self.row_of_dna.len();
        let mut rng_gen = thread_rng();
        let start_ind = rng_gen.gen_range(0, length - 1);
        let end_ind = rng_gen.gen_range(start_ind + 1, length);

        let daughter_row_of_dna = crossover_dna(&self.row_of_dna, &second.row_of_dna, start_ind, end_ind);
        let son_row_of_dna = crossover_dna(&second.row_of_dna, &self.row_of_dna, start_ind, end_ind);
        
        let daughter_solution = Solution::generate(daughter_row_of_dna, cities);
        let son_solution = Solution::generate(son_row_of_dna, cities);
        
        (daughter_solution, son_solution)
    }

    pub fn mutate(&mut self, cities: &[CityPoint]) {
        let index = thread_rng().gen_range(0, self.row_of_dna.len() - 1);
        self.row_of_dna.swap(index, index + 1);
        self.fitness_value = calc_fitness_value(&self.row_of_dna, &cities);
    }
}

fn calc_fitness_value(row_of_dna: &[usize], cities: &[CityPoint]) -> f64 {
    let dist = row_of_dna.windows(2)
               .fold(MIN, |acc, wei| acc + cities[wei[0]].get_sqr_dist(&cities[wei[1]]));
    1.0 / dist
}

fn crossover_dna(mom_row_of_dna: &[usize], dad_row_of_dna: &[usize], start_ind: usize, end_ind: usize) -> Vec<usize> {
    
    let mom_row_of_dna_slice = &mom_row_of_dna[start_ind..=end_ind];
    let mut child: Vec<usize> = Vec::new();
    
    for i in 0..dad_row_of_dna.len() {
        if !mom_row_of_dna_slice.contains(&dad_row_of_dna[i]) {
            child.push(dad_row_of_dna[i]);
        }
    }
    
    let end_row_of_dna_slice = &child.split_off(start_ind);
    child.extend_from_slice(mom_row_of_dna_slice);
    child.extend_from_slice(end_row_of_dna_slice);
    child
}