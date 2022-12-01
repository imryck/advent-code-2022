use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct DayCliArgs {
    pub day: String,
    pub run_mode: String,
    pub input_path: String,
}

pub fn parse_cli_args() -> DayCliArgs {
    let vars: Vec<String> = env::args().collect();
    let day = vars[0]
        .split('/')
        .last()
        .expect("Invalid bin name");
    let run_mode = if vars.len() > 1 { &vars[1] } else { "test" };
    println!("Running day {} with mode {}", day, run_mode);
    DayCliArgs {
        day: String::from(day),
        run_mode: String::from(run_mode),
        input_path: format!("./{}/input-{}.txt", day, run_mode)
    }
}

pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(Path::new(path))
        .expect(&format!("No input file found for path {}", path));
    io::BufReader::new(file)
        .lines()
        .collect()
}

// TODO write some tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
