use std::fs;
use std::io::{self, BufRead, Write};
use std::thread;
use std::time::Duration;
use std::path::Path;

const DATA_FILE: &str = "dane";
const RESULT_FILE: &str = "wyniki";

// check if the file is not empty (empty = server hasn't answered yet)
fn read_nonempty(path: &str) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(s) if s.trim() != "" => Some(s),
        _ => None,
    }
}

fn main() {
    // make sure the files exist
    if !Path::new(DATA_FILE).exists() {
        fs::File::create(DATA_FILE).unwrap();
    }
    if !Path::new(RESULT_FILE).exists() {
        fs::File::create(RESULT_FILE).unwrap();
    }

    print!("Podaj liczbe calkowita: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).unwrap();
    let number = input.trim().to_string();

    // clear the results just in case, so we don't read a leftover result from a previous run
    fs::write(RESULT_FILE, "").unwrap();
    fs::write(DATA_FILE, format!("{}\n", number)).unwrap();

    // busy waiting for the server's answer
    loop {
        if let Some(text) = read_nonempty(RESULT_FILE) {
            println!("Otrzymany wynik: {}", text.trim());
            // clear the file after reading, like the task says
            fs::write(RESULT_FILE, "").unwrap();
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
}
