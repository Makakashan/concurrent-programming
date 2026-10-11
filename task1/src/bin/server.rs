use std::fs;
use std::thread;
use std::time::Duration;
use std::path::Path;

// file names used for communication
const DATA_FILE: &str = "dane";
const RESULT_FILE: &str = "wyniki";

// helper function - returns Some(content) if there is something in the file, None if empty
fn read_nonempty(path: &str) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(s) if !s.trim().is_empty() => Some(s),
        _ => None,
    }
}

// clearing the file after reading (so we don't read the same thing twice)
fn clear_file(path: &str) {
    fs::write(path, "").expect("blad czyszczenia pliku");
}

fn main() {
    // create the files on startup if they don't exist yet
    if !Path::new(DATA_FILE).exists() {
        fs::File::create(DATA_FILE).expect("nie mozna utworzyc pliku dane");
    }
    if !Path::new(RESULT_FILE).exists() {
        fs::File::create(RESULT_FILE).expect("nie mozna utworzyc pliku wyniki");
    }

    println!("Serwer dziala, czekam na liczbe w pliku '{}'", DATA_FILE);
    loop {
        // busy waiting - keep checking the file until the client writes something
        if let Some(text) = read_nonempty(DATA_FILE) {
            let x: i64 = match text.trim().parse() {
                Ok(x) => x,
                Err(_) => {
                    // client sent something that is not a number, send an error back
                    fs::write(RESULT_FILE, format!("BLAD: '{}' to nie liczba\n", text.trim())).unwrap();
                    clear_file(DATA_FILE);
                    continue;
                }
            };

            // my polynomial: w(x) = 2x^3 - 3x^2 + 4x - 5
            let result = 2*x*x*x - 3*x*x + 4*x - 5;

            fs::write(RESULT_FILE, format!("{}\n", result)).expect("blad zapisu wynikow");
            clear_file(DATA_FILE);
            println!("Serwer: {} -> {}", x, result);
        }
        // small pause so the cpu isn't at 100% all the time
        thread::sleep(Duration::from_millis(100));
    }
}
