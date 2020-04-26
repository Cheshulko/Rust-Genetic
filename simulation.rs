use rand::{thread_rng, Rng};
use super::*; 
use utils::print_vector;

pub struct Simulation {

    iterations_cnt: usize,

    probability_of_crossover: f64,
    probability_of_mutation: f64,
    size_of_population: usize, 

    cities_number: usize,
    cities: Vec<CityPoint>,

    mutations_number: usize,
    crossovers_number: usize,

    pub fitness_value: f64,
    pub row_of_dna: Vec<usize>, 
}

impl Simulation {

    pub fn genetare(iterations_cnt: usize,
               probability_of_crossover: f64,
               probability_of_mutation: f64,
               size_of_population: usize,
               cities: Vec<CityPoint>) -> Self {

        assert_eq!(size_of_population % 10, 0, "size_of_population:{} should be divisible by 10", size_of_population);

        let cities_number = cities.len();
        let mutations_number = 0;
        let crossovers_number = 0;
        let fitness_value = 0.0;
        let row_of_dna: Vec<usize> = Vec::new(); 

        Simulation { 
            iterations_cnt, 
            probability_of_crossover, 
            probability_of_mutation, 
            size_of_population, 
            cities_number, 
            cities, 
            mutations_number,
            crossovers_number,
            fitness_value,
            row_of_dna,
        }
    }

    fn generate_children(&mut self, mom_row_of_dna: &Solution, dad_row_of_dna: &Solution) -> (Solution, Solution) {
        if thread_rng().gen_bool(self.probability_of_crossover) {
            self.crossovers_number += 2;
            mom_row_of_dna.crossover(dad_row_of_dna, &self.cities)
        } else {
            (mom_row_of_dna.clone(), dad_row_of_dna.clone())
        }
    }

    fn maybe_do_mutate_child(&mut self, child: &mut Solution) {
        if thread_rng().gen_bool(self.probability_of_mutation) {
            child.mutate(&self.cities);
            self.mutations_number += 1;
        }
    }

    pub fn generate_population(&mut self, solutions: Vec<Solution>) -> Vec<Solution> {
        let weights_sum = get_cumulative_weights(&solutions);
        let mut generated_population = Vec::new();

        for _ in 0..(self.size_of_population / 2 ) {
            let (mom_row_of_dna, dad_row_of_dna) = select_parents(&weights_sum, &solutions);
            let (mut daughter_row_of_dna, mut son_row_of_dna) = self.generate_children(&mom_row_of_dna, &dad_row_of_dna);
            self.maybe_do_mutate_child(&mut daughter_row_of_dna);
            self.maybe_do_mutate_child(&mut son_row_of_dna);

            generated_population.push(daughter_row_of_dna);
            generated_population.push(son_row_of_dna);
        }
        generated_population
    }

    pub fn run(&mut self, debuging_level: usize, steps_skip: usize) {
        assert!(steps_skip > 0, "steps_skip must be 1 or larger");

        let mut current_population = random_population(self.size_of_population, &self.cities);
        let mut current_champion = find_fittest(&current_population);

        for i in 0..self.iterations_cnt {

            let maybe_new_champion = find_fittest(&current_population);
            current_population = self.generate_population(current_population);
            print_opt_debug(debuging_level, steps_skip, i + 1, &current_population, &current_champion, &maybe_new_champion, self.cities_number);

            if current_champion.fitness_value <= maybe_new_champion.fitness_value {
                current_champion = maybe_new_champion;
            }
        }

        self.fitness_value = current_champion.fitness_value;
        self.row_of_dna = current_champion.row_of_dna;

        if debuging_level >= 2 { self.print_info_debug(); }
    }

    // remove that
    pub fn print_info_debug(&self) {

        let x = self.size_of_population * self.iterations_cnt;

        println!("\n --------------- \n STATS \n --------------- \n");
        println!("BEST TRAVEL PATH: {:?}", self.row_of_dna);
        println!("Fitness Score: {} ", self.fitness_value);
        println!("{} mutations out of {} solutions produced", self.mutations_number, x);
        println!("{} cross-overs out of {} solutions produced", self.crossovers_number, x);

        println!("\n --------------- \n SPECS \n --------------- \n");
        println!("iterations_cnt: {:?}", self.iterations_cnt);
        println!("probability_of_crossover: {:?}", self.probability_of_crossover);
        println!("probability_of_mutation: {:?}", self.probability_of_mutation);
        println!("size_of_population: {:?}", self.size_of_population);
        println!("cities_number: {:?}", self.cities_number);
        println!("\n Cities: ");
        print_vector(&self.cities);

        println!("\n --------------- \n END \n --------------- \n");

    }
}

fn print_opt_debug(debuging_level: usize, 
               steps_skip: usize, 
               i: usize, 
               current_population: &[Solution],
               current_champion: &Solution, 
               maybe_new_champion: &Solution, 
               n: usize) {

    if debuging_level == 1 && (i % steps_skip == 0) {
        print!("{}, {}, {}, {},", i, n, current_champion.fitness_value, maybe_new_champion.fitness_value);

        for i in 0..n {
            print!(" {},", current_champion.row_of_dna[i]);
        }

        for i in 0..(n - 1) {
            print!(" {},", maybe_new_champion.row_of_dna[i]);
        }

        println!(" {}", maybe_new_champion.row_of_dna[n - 1]);
    }

    if debuging_level == 3 {
        println!("#{}: \n current_champion: {:?} \n maybe_new_champion: {:?}", 
            i, current_champion, maybe_new_champion);
    }

    if debuging_level == 4 {
        println!("\n --------------- \n {}: Current Population \n --------------- \n", i);
        print_vector(current_population);
    }
}