extern crate clap;
extern crate itertools;
extern crate serde_json;

use clap::{App, Arg, ArgMatches};
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;
use std::thread;

// Define type to descript `InputMatrix` format
type InputMatrix = HashMap<String, Vec<String>>;

#[derive(Clone, Debug)]
struct EnvVar {
    name: String,
    value: String,
}

fn parse_matrix(m: &str) -> serde_json::Result<InputMatrix> {
    let parsed: InputMatrix = match serde_json::from_str(m) {
        Ok(v) => v,
        Err(err) => return Err(err),
    };
    Ok(parsed)
}

fn get_args() -> ArgMatches<'static> {
    let app = App::new("Biscotti")
        .version("0.0.1")
        .author("Nemo <jgiannelos@mozilla.com>")
        .about("Generate matrix of all possible combinations of env vars")
        .arg(
            Arg::with_name("matrix")
                .help("Environment variable matrix")
                .short("m")
                .long("matrix")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("command")
                .help("Command to run with matrix combinations")
                .short("c")
                .long("command")
                .takes_value(true)
                .required(true),
        );

    return app.get_matches();
}

fn get_combinations(input_matrix: InputMatrix) -> Vec<Vec<EnvVar>> {
    let mut matrix: Vec<Vec<EnvVar>> = vec![];

    for (key, values) in input_matrix.iter() {
        let mut env_vars: Vec<EnvVar> = vec![];
        for value in values.iter() {
            env_vars.push(EnvVar {
                name: key.to_string(),
                value: value.to_string(),
            });
        }
        matrix.push(env_vars);
    }

    return matrix.into_iter().multi_cartesian_product().collect();
}

fn run_commands(combinations: Vec<Vec<EnvVar>>, command: String) {
    let mut children = vec![];

    for combination in combinations {
        let tuples: Vec<(String, String)> =
            combination.into_iter().map(|x| (x.name, x.value)).collect();
        let envs: HashMap<String, String> = tuples.into_iter().collect();

        let current_command = command.clone();
        let handle = thread::spawn(move || {
            let output = Command::new("sh")
                .arg("-c")
                .arg(current_command)
                .envs(envs)
                .output()
                .expect("Failed to execute command");

            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        });
        children.push(handle);
    }

    for child in children {
        child.join();
    }
}

fn main() -> Result<(), String> {
    let args = get_args();
    let matrix_str = args
        .value_of("matrix")
        .expect("No input given for `matrix`.");
    let command = args
        .value_of("command")
        .expect("No input given for `command`.");

    match parse_matrix(matrix_str) {
        Ok(v) => Ok(run_commands(get_combinations(v), command.to_string())),
        Err(_) => Err("Cannot parse JSON input".to_string()),
    };

    Ok(())
}
