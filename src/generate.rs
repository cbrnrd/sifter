extern crate yaml_rust;
use std::collections::hash_map::RandomState;
use std::fs::File;
use std::io::{Write, BufWriter, self};
use std::path::Path;
use itertools::izip;


use yaml_rust::yaml::{Hash, Yaml};
use yaml_rust::{YamlLoader};

use linked_hash_map::LinkedHashMap;


// Take in a yaml config file and an output file and generate a password list from it

pub fn generate(config: Option<String>, output: Option<String>) {
    // Read the config file
    let config_file = config.unwrap();

    // Read the config file
    let config = std::fs::read_to_string(config_file).unwrap();

    // Parse the config file
    let docs = YamlLoader::load_from_str(&config).unwrap();
    let doc = &docs[0];
    // hash has keys: names, dates, numbers, other
    let hash = doc.as_hash().unwrap();


    let first_names = get_first_names(hash);
    let last_names = get_last_names(hash);
    let dates = get_dates(hash);
    let numbers = get_numbers(hash);
    let other = get_other(hash);

    do_generate(first_names, last_names, dates, numbers, other, output);

}

fn do_generate(first_names: Vec<&str>, last_names: Vec<&str>, dates: Vec<&str>, numbers: Vec<i64>, other: Vec<&str>, out: Option<String>) {
    // Generate a password list from the given vectors
    // if output is None, print to stdout, otherwise write to a file
    let mut out_writer = match out {
        Some(x) => {
            let path = Path::new(&x);
            Box::new(File::create(&path).unwrap()) as Box<dyn Write>
        }
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };

    // Iterate over slices of the vectors
    for (first_name, last_name, date, number, other) in izip!(first_names, last_names, dates, numbers, other) {
        // Generate a password
        let password = format!("{}{}{}{}{}", first_name, last_name, date, number, other);
        // Write the password to the output file
        out_writer.write_all(password.as_bytes()).unwrap();
        out_writer.write_all(b"\n").unwrap();
    }
}


fn get_first_names(yaml_hash: &LinkedHashMap<Yaml, Yaml, RandomState>) -> Vec<&str>{
    let names = yaml_hash.get(&Yaml::String(String::from("names"))).unwrap();
    let first_names = names.as_hash().unwrap().get(&Yaml::String(String::from("first"))).unwrap();
    let first_names = first_names.as_vec().unwrap();
    let first_names = first_names.iter().map(|s| s.as_str().unwrap()).collect::<Vec<&str>>();
    return first_names;
}

fn get_last_names(yaml_hash: &LinkedHashMap<Yaml, Yaml, RandomState>) -> Vec<&str> {
    // Get the last names
    let names = yaml_hash.get(&Yaml::String(String::from("names"))).unwrap();
    let last_names = names.as_hash().unwrap().get(&Yaml::String(String::from("last"))).unwrap();
    let last_names = last_names.as_vec().unwrap();
    let last_names = last_names.iter().map(|s| s.as_str().unwrap()).collect::<Vec<&str>>();
    return last_names;
}

fn get_dates(yaml_hash: &LinkedHashMap<Yaml, Yaml, RandomState>) -> Vec<&str> {
    let dates = yaml_hash.get(&Yaml::String(String::from("dates"))).unwrap();
    let dates = dates.as_vec().unwrap();
    let dates = dates.iter().map(|s| s.as_str().unwrap()).collect::<Vec<&str>>();
    return dates;
}

fn get_numbers(yaml_hash: &LinkedHashMap<Yaml, Yaml, RandomState>) -> Vec<i64> {
    // Get the numbers
    let numbers = yaml_hash.get(&Yaml::String(String::from("numbers"))).unwrap();
    let numbers = numbers.as_vec().unwrap();
    // Each entry is an integer
    let numbers = numbers.iter().map(|s| s.as_i64().unwrap()).collect::<Vec<i64>>();
    return numbers;
}

fn get_other(yaml_hash: &LinkedHashMap<Yaml, Yaml, RandomState>) -> Vec<&str> {
    // Get the other
    let other = yaml_hash.get(&Yaml::String(String::from("other"))).unwrap();
    let other = other.as_vec().unwrap();
    let other = other.iter().map(|s| s.as_str().unwrap()).collect::<Vec<&str>>();
    return other;
}