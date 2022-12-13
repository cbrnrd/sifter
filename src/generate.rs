extern crate yaml_rust;
use std::collections::hash_map::RandomState;
use std::fs::File;
use std::io::{Write, self};
use std::path::Path;
use itertools::izip;
use itertools::Itertools; // 0.8.2


use yaml_rust::yaml::{Yaml};
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

    write_names_basic_combo(&mut out_writer, &first_names, &last_names);
    write_name_dates_combo(&mut out_writer, &first_names, &last_names, &dates);
    write_names_numbers(&mut out_writer, &first_names, &last_names, &numbers);
    write_all_first_name_permutations(&mut out_writer, &first_names);
}

fn write_names_basic_combo(out_writer: &mut Box<dyn Write>, first_names: &Vec<&str>, last_names: &Vec<&str>) {
    // For each first name, write the following combinations:
    // first_name + last_name
    // first_name.capitalize() + last_name + [0-9]
    // first_name.capitalize() + last_name.capitalize()
    // first_name.capitalize() + last_name.capitalize() + [0-9]
    // first_name.capitalize() + last_name.capitalize() + [!@#$]
    // first_name + last_name.capitalize()

    for first_name in first_names {
        for last_name in last_names {
            // first_name + last_name
            let password = format!("{}{}", first_name, last_name);
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
        }

        for last_name in last_names {
            // first_name.capitalize() + last_name + [0-9]
            for i in 0..10 {
                let password = format!("{}{}{}", capitalize(first_name), last_name, i);
                out_writer.write_all(password.as_bytes()).unwrap();
                out_writer.write_all(b"\n").unwrap();
            }
        }

        for last_name in last_names {
            // first_name.capitalize() + last_name.capitalize()
            let password = format!("{}{}", capitalize(first_name), capitalize(last_name));
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
        }

        for last_name in last_names {
            for i in 0..10 {
                // first_name.capitalize() + last_name.capitalize() + [0-9]
                let password = format!("{}{}{}", capitalize(first_name), capitalize(last_name), i);
                out_writer.write_all(password.as_bytes()).unwrap();
                out_writer.write_all(b"\n").unwrap();
            }
        }

        for last_name in last_names {
            // first_name.capitalize() + last_name.capitalize() + [!@#$]
            for c in "!@#$".chars() {
                let password = format!("{}{}{}", capitalize(first_name), capitalize(last_name), c);
                out_writer.write_all(password.as_bytes()).unwrap();
                out_writer.write_all(b"\n").unwrap();
            }
        }

        for last_name in last_names {
            // first_name + last_name.capitalize()
            let password = format!("{}{}", first_name, capitalize(last_name));
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
        }

    }
}

fn write_name_dates_combo(out_writer: &mut Box<dyn Write>, first_names: &Vec<&str>, last_names: &Vec<&str>, dates: &Vec<&str>) {
    // For each first name, write the following combinations:
    // first_name + last_name
    // first_name + date
    // first_name + last_name + date
    // first_name + date.reverse()
    // Again but capitalize the first letter of the each name instance for each of the above
    
    for first_name in first_names {
        for last_name in last_names {
            let password = format!("{}{}", first_name, last_name);
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
        }
        for date in dates {
            let password = format!("{}{}", first_name, date);
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
        }
        for (last_name, date) in izip!(last_names, dates) {
            let password = format!("{}{}{}", first_name, last_name, date);
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
        }
        for date in dates {
            let password = format!("{}{}", first_name, date.chars().rev().collect::<String>());
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
        }
    }
    
}

fn write_names_numbers(out_writer: &mut Box<dyn Write>, first_names: &Vec<&str>, last_names: &Vec<&str>, numbers: &Vec<i64>) {
    // For each first name, write the following combinations:
    // first_name + number
    // first_name.capitalize() + number
    // first_name + last_name + number
    // first_name + last_name.capitalize() + number
    // first_name.capitalize() + last_name + number
    // first_name.capitalize() + last_name.capitalize() + number
    // first_name.capitalize() + last_name.capitalize() + number + [!@#$]
    // number + first_name
    // number + first_name.capitalize()
    // number + first_name + last_name
    // number + first_name + last_name.capitalize()

    for first_name in first_names {
        for number in numbers {
            // first_name + number
            let password = format!("{}{}", first_name, number);
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
        }
        for number in numbers {
            // first_name.capitalize() + number
            let password = format!("{}{}", capitalize(first_name), number);
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
        }
        for (last_name, number) in izip!(last_names, numbers) {
            // first_name + last_name + number
            // first_name + last_name.capitalize() + number
            let mut password = format!("{}{}{}", first_name, last_name, number);
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
            password = format!("{}{}{}", first_name, capitalize(last_name), number);
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
        }

        for (last_name, number) in izip!(last_names, numbers) {
            // first_name.capitalize() + last_name + number
            // first_name.capitalize() + last_name.capitalize() + number
            let mut password = format!("{}{}{}", capitalize(first_name), last_name, number);
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
            password = format!("{}{}{}", capitalize(first_name), capitalize(last_name), number);
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
        }

        for (last_name, number) in izip!(last_names, numbers) {
            // first_name.capitalize() + last_name.capitalize() + number + [!@#$]
            let mut password = format!("{}{}{}{}", capitalize(first_name), capitalize(last_name), number, "!");
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
            password = format!("{}{}{}{}", capitalize(first_name), capitalize(last_name), number, "@");
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
            password = format!("{}{}{}{}", capitalize(first_name), capitalize(last_name), number, "#");
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
            password = format!("{}{}{}{}", capitalize(first_name), capitalize(last_name), number, "$");
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
        }

        for number in numbers {
            // number + first_name
            // number + first_name.capitalize()
            let mut password = format!("{}{}", number, first_name);
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
            password = format!("{}{}", number, capitalize(first_name));
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
        }

        for (last_name, number) in izip!(last_names, numbers) {
            // number + first_name + last_name
            // number + first_name + last_name.capitalize()
            let mut password = format!("{}{}{}", number, first_name, last_name);
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
            password = format!("{}{}{}", number, first_name, capitalize(last_name));
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
        }
    }

}

fn write_all_first_name_permutations(out_writer: &mut Box<dyn Write>, first_names: &Vec<&str>) {
    // Get all permutations of the first_names vector greater than 1
    for i in 2..first_names.len() {
        let permutations = first_names.iter().permutations(i);
        for permutation in permutations {
            let mut password = String::new();
            for name in permutation {
                password.push_str(name);
            }
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();

            // Capitalize the first names
            let password = capitalize(&password);
            out_writer.write_all(password.as_bytes()).unwrap();
            out_writer.write_all(b"\n").unwrap();
        }
    }
    
}

//////////////////////////
///// CONFIG PARSING /////
//////////////////////////

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

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

