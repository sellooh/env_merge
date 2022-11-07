use clap::Parser;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

/// Tool for merging a template of envs into one
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// out file
    #[arg(short, long, default_value = ".env")]
    out: String,

    /// list of templates separated by ',' example: .env.local.template,.env.test.template
    #[arg(short, long, use_value_delimiter=true, value_delimiter=',')]
    templates: Vec<String>,
}

fn read_template(file_name: &str, variables: &mut HashMap<String, String>) {
    let f = File::open(file_name).unwrap_or_else(|_| panic!("Unable to open '{}'", file_name));
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        if line.starts_with('#') {
            continue;
        }
        let line_option = line.split_once('=');
        if let Some((key, value)) = line_option {
            let variable_values: Vec<&str> = value.split(&[' ', '#'][..]).collect();
            variables.insert(key.to_owned(), variable_values[0].to_owned());
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut template_variables: HashMap<String, String> = HashMap::new();
    let mut env_variables: HashMap<String, String> = HashMap::new();

    for template in &args.templates {
        read_template(&template, &mut template_variables);
    }

    let mut dot_env_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(".env")
        .unwrap();

    let f = File::open(&args.out).unwrap_or_else(|_| panic!("Unable to open '{}'", &args.out));
    let mut reader = BufReader::new(f);

    let mut buf = String::new();
    let mut add_line_break = false;
    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            // [EOF]
            break;
        }
        add_line_break = !buf.ends_with('\n');

        let line_option = buf.split_once('=');
        if let Some((key, value)) = line_option {
            let (value, ..)  = value.split_once('#').unwrap_or_else(|| (value, ""));
            if buf.starts_with( '#') {
                buf.clear(); // otherwise the data will accumulate in the buffer
                continue;
            }
            env_variables.insert(
                key.trim().to_string(),
                value.trim().to_string(),
            );
        }
        buf.clear(); // otherwise the data will accumulate in the buffer
    }

    if add_line_break {
        writeln!(dot_env_file).expect("Could not add new line");
    }

    // DEBUG
    // println!("{:?}", template_variables);
    // println!("{:?}", env_variables);
    template_variables.retain(|key, value| {
        if !env_variables.contains_key(key) {
            writeln!(dot_env_file, "{}={}", key, value).expect("Could not write to .env");
        }
        true
    });

    println!("Done merging local {} from {:?}", args.out, args.templates);
}
