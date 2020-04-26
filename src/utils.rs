use std::fmt::Debug;

use std::io::prelude::*;
use std::fs::File;
use std::process;

use rand::{thread_rng, Rng};

pub fn print_vector<T: Debug>(v: &[T]) {
    for i in v.iter() { println!("{:?}", i); }   
}

pub fn choose_ind(weights_sum: &[f64]) -> usize {
    let sum_w = weights_sum.last().unwrap(); 
    let rand: f64 = thread_rng().gen_range(0.0, *sum_w);
    weights_sum.iter().rposition(|&wei| wei < rand).unwrap()
}

pub fn open_file_and_read(filepath: &String) -> String {
    let mut f = File::open(filepath).unwrap_or_else(|err| {
        eprintln!("Could not open {:?}\n error: {}\n ", filepath, err);
        process::exit(1)
    });

    let mut data = String::new();

    f.read_to_string(&mut data).unwrap_or_else(|err| { 
        eprintln!("Could not open file.\n error: {}", err); 
        process::exit(1)
    });
    data
}

pub fn read_and_parse_config(str_in: &str) -> Result<(usize, usize, usize, usize, f64, f64), String> { 
    let ve: Vec<String> = str_in.split(',')
                               .map(|value| value.trim().to_string())
                               .collect();

    if ve.len() != 6 {
        return Err("Unexpected number of specs (must be exactly 6)".to_string());
    }

    let debuging_output_level: usize = ve[0].parse().map_err(|err| {
        format!("debuging_output_level = {} is NOT int.\nerror: {}\n", ve[0], err)
    })?;

    let skip_step: usize = ve[1].parse().map_err(|err| {
        format!("skip_step = {} is NOT int.\nerror: {}\n", ve[1], err)
    })?;

    let iterations_cnt: usize = ve[2].parse().map_err(|err| {
        format!("iterations_cnt = {} is NOT int.\nerror: {}\n", ve[2], err)
    })?;

    let size_of_population: usize = ve[3].parse().map_err(|err| {
        format!("size_of_population = {} is NOT int.\nerror: {}\n", ve[3], err)
    })?;

    let probability_of_crossover: f64 = ve[4].parse().map_err(|err| {
        format!("probability_of_crossover = {} is NOT float.\nerror: {}\n", ve[4], err)
    })?;

    let probability_of_mutation: f64 = ve[5].parse().map_err(|err| {
        format!("probability_of_mutation = {} is NOT float.\nerror: {}\n", ve[5], err)
    })?;

    Ok((
        debuging_output_level,
        skip_step,
        iterations_cnt,
        size_of_population,
        probability_of_crossover,
        probability_of_mutation
    ))
}
