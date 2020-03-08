use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, COOKIE};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

const CACHE_DIR: &str = "input_cache";
const SESSION_FILE_PATH: &[&str] = &["..", "..", "session.txt"];

fn get_session_key() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.push(&SESSION_FILE_PATH.iter().cloned().collect::<PathBuf>());

    let mut content = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut content).unwrap();
    content.trim().to_string()
}

fn get_input_web(day: u8) -> Result<String, Box<dyn Error>> {
    let url_str = format!("https://adventofcode.com/2019/day/{}/input", day);
    let mut headers = HeaderMap::new();
    headers.insert(
        COOKIE,
        format!("session={}", get_session_key()).parse().unwrap(),
    );
    let resp = Client::new().get(&url_str).headers(headers).send()?;
    Ok(resp.text()?)
}

fn try_get_input(day: u8) -> Result<String, Box<dyn Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::current_exe()?.parent().unwrap());
    path.push(CACHE_DIR);
    path.push(format!("day_{}.txt", day));

    let _ = std::fs::create_dir(path.parent().unwrap());

    if path.exists() {
        println!("Cache hit for day {}", day);
        Ok(std::fs::read_to_string(path)?)
    } else {
        println!("Cache miss for day {}", day);
        let input_str = get_input_web(day)?;
        std::fs::write(path, &input_str)?;
        Ok(input_str)
    }
}

pub fn get_input(day: u8) -> String {
    try_get_input(day).expect("Failed getting input")
}
