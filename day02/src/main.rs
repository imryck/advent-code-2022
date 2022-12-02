use common;

struct GuideLine {
    pub pattern: String,
    pub target: String,
}

fn main() {
    let cli_args = common::parse_cli_args();
    let guide: Vec<GuideLine> = common::read_lines(&cli_args.input_path)
        .unwrap()
        .into_iter()
        .map(|line| GuideLine {
            pattern: line.clone(),
            target: line.split(" ").collect::<Vec<&str>>()[1].to_string(),
        }).collect();

    let total = guide.into_iter().fold(0, |acc, round| {
        acc + get_round_score(&round.target) + get_sign_score(&get_target_response(&round.pattern))
    });
    println!("Total of all rounds is {}", total);
}

pub fn get_sign_score(sign: &str) -> i32 {
    match sign {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("No matched sign !")
    }
}

pub fn get_target_response(round_pattern: &str) -> String {
    match round_pattern {
        "A Y" | "B X" | "C Z" => String::from("A"),
        "A Z" | "B Y" | "C X" => String::from("B"),
        "A X" | "B Z" | "C Y" => String::from("C"),
        _ => panic!("No matched round pattern ! {}", round_pattern)
    }
}

pub fn get_round_score(round_target: &str) -> i32 {
    match round_target {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("No matched target round ! {}", round_target)
    }
}
