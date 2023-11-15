use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

struct Config {
    ip: String,
    port: String,
    file: String,
    num_threads: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: String::new(),
            port: String::new(),
            file: String::new(),
            num_threads: String::new(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::default();

    println!("{:?}", args);

    for (index, arg) in args.iter().enumerate() {
        //ip
        if arg == "-ip" && &index + 1 < args.len() {
            config.ip = args[index + 1].clone();
        }

        //port
        if arg == "-p" && &index + 1 < args.len() {
            config.port = args[index + 1].clone();
        }

        // wordlist
        if arg == "-w" && &index + 1 < args.len() {
            config.file = args[index + 1].clone();
            process_wordlist(&config.file);
        }
    }
}

fn process_wordlist(wordlist_path: &str) {
    //open the wordlist file
    if let Ok(file) = File::open(wordlist_path) {
        //read file rows
        for line in io::BufReader::new(file).lines() {
            if let Ok(word) = line {
                println!("word: {}\n", word);
            }
        }
    } else {
        print!("ERROR: Error opening wordlist file.")
    }
}
