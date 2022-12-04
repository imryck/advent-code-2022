use common;

#[derive(Clone)]
struct RucksackGroup {
    types: Vec<char>,
}

fn main() {
    let cli_args = common::parse_cli_args();
    let mut group: Vec<RucksackGroup> = Vec::new();
    let mut total_priority: i32 = 0;
    let mut total_group_priority: i32 = 0;
    common::read_lines(&cli_args.input_path).unwrap().into_iter().enumerate()
        .for_each(|(i, items)| {
            let (part1, part2) = items.split_at(items.len() / 2);
            let rucksack_items: Vec<char> = items.chars().collect();
            total_priority += i32::from(
                get_priority(
                    get_common_types(part1.chars().collect(), part2.chars().collect())[0]
                )
            );

            if i % 3 == 0 {
                group.push(RucksackGroup { types: rucksack_items.clone() });
            } else {
                let last = group.last_mut().unwrap();
                last.types = get_common_types(last.clone().types, rucksack_items.clone());
                if i % 3 == 2 {
                    total_group_priority += i32::from(get_priority(last.types[0]));
                }
            }
        });

    println!("priority {:#?}", total_priority);
    println!("Group priority {:#?}", total_group_priority);
}

fn get_priority(c: char) -> u8 {
    let char_code = c as u8;
    let priority: u8;
    if char_code >= 97 {
        priority = char_code - 96;
    } else {
        priority = char_code - 38;
    }
    priority
}

fn get_common_types(l1: Vec<char>, l2: Vec<char>) -> Vec<char> {
    let mut common_types: Vec<char> = Vec::new();
    l1.into_iter()
        .for_each(|c| {
            if l2.contains(&c) && !common_types.clone().contains(&c) {
                common_types.push(c);
            }
        });
    common_types
}
