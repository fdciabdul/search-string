use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <search_string>", args[0]);
        return Ok(());
    }

    let path = Path::new("data");
    let search_string = &args[1];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                let file = fs::File::open(&path)?;
                let reader = io::BufReader::new(file);
                for (index, line) in reader.lines().enumerate() {
                    let line = line?;
                    if line.contains(search_string) {
                        println!("Found in file '{}' on line {}: {}", path.display(), index + 1, line);
                    }
                }
            }
        }
    }

    Ok(())
}
