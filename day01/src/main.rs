use common;

#[allow(dead_code)]
struct Elf {
    snacks: Vec<i32>,
    calories: i64,
}

fn main() {
    let cli_args = common::parse_cli_args();
    let elves_snacks: Vec<Vec<i32>> = common::read_lines(&cli_args.input_path)
        .unwrap()
        .into_iter()
        .fold(Vec::new(), |mut acc, snack| {
            if acc.is_empty() || snack == "" {
                acc.push(Vec::new());
            }
            if snack != "" {
                acc.last_mut().unwrap().push(
                    snack.to_string().parse::<i32>().unwrap()
                );
            }
            acc
        });
    let mut elves: Vec<Elf> = elves_snacks.into_iter()
        .map(|snacks| {
            let calories = snacks.clone()
                .into_iter()
                .fold(0, |acc, s| acc + i64::from(s));
            Elf {
                snacks: snacks.clone(),
                calories,
            }
        })
        .collect::<Vec<Elf>>();
    elves.sort_by(|a, b| b.calories.cmp(&a.calories));

    println!("The elf with the most calories carrying {:#?} calories", elves.last().unwrap().calories);

    let top_tree = &elves[0..3].into_iter()
        .fold(0, |acc, elf| acc + i64::from(elf.calories));
    println!("The top tree elves are {:#?}", top_tree);
}
