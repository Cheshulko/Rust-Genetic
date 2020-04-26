extern crate citydna;

use std::process;
use std::env;

use citydna::city_point;
use citydna::utils;
use citydna::{CityPoint, Simulation};

fn main() {
    let mut args = env::args().skip(1);

    let config_filename = args.next()
        .unwrap_or_else( || {
            eprintln!("No config file");
            process::exit(1)
            }
        );

    let input_cities_filename = args.next()
        .unwrap_or_else( || {
            eprintln!("No cities file");
            process::exit(1)
            }
        );

    let data = utils::open_file_and_read(&config_filename);
    let (debuging_level, 
         steps_skip,
         iterations_cnt, 
         size_of_population,
         probability_of_crossover,
         probability_of_mutation) = utils::read_and_parse_config(&data).unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
         });

    let data = utils::open_file_and_read(&input_cities_filename);
    let cities: Vec<CityPoint> = city_point::parse_cities(&data);

    let mut simulation_main = Simulation::genetare(
        iterations_cnt,
        probability_of_crossover, 
        probability_of_mutation, 
        size_of_population,
        cities
    );

    simulation_main.run(debuging_level, steps_skip);
}
