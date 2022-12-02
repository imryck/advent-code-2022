use common;

#[allow(dead_code)]
struct Elf {
    snacks: Vec<i32>,
    calories: i64,
}

fn main() {
    let cli_args = common::parse_cli_args();
    let mut elves: Vec<i64> = common::read_lines(&cli_args.input_path).unwrap().into_iter()
        .fold(Vec::new(), |mut acc, snack| {
            if acc.is_empty() || snack == "" {
                acc.push(Vec::new());
            } else {
                acc.last_mut().unwrap().push(snack.as_str().parse::<i64>().unwrap());
            }
            acc
        })
        .into_iter()
        .map(|snaks| snaks.into_iter().fold(0, |acc, s| acc + s))
        .collect();
    elves.sort();
    elves.reverse();

    println!("The elf with the most calories carrying {:#?} calories", elves.last().unwrap());
    println!("The top tree elves are {:#?}", &elves[0..3].into_iter().fold(0, |acc, cal| acc + cal));
}
