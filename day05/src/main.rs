use std::collections::HashMap;

use regex::Regex;

use common;

#[derive(Clone, Debug)]
struct Instruction {
    from: usize,
    to: usize,
    quantity: i32,
}

fn main() {
    let cli_args = common::parse_cli_args();
    let (instructions, mut stacks) = parse_data(common::read_lines(&cli_args.input_path).unwrap());
    instructions.into_iter().for_each(|i| {
        let mut new_from = stacks.get(&i.from).unwrap().clone();
        let from_stack_size = new_from.len();
        let new_to = [
            stacks.get(&i.to).unwrap().clone(),
            new_from.drain((from_stack_size - i.quantity as usize)..from_stack_size).collect()
        ].concat();
        stacks.remove_entry(&i.to).unwrap();
        stacks.insert(i.to, new_to);
        stacks.remove_entry(&i.from).unwrap();
        stacks.insert(i.from, new_from);
    });

    let mut lasts = stacks.into_iter()
        .map(|(k, s)| (k, s.last().unwrap().to_string()))
        .collect::<Vec<(usize, String)>>();
    lasts.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{:#?}", lasts.into_iter().map(|(_k, s)| s.to_string()).collect::<Vec<String>>().join(""));
}

fn parse_data(lines: Vec<String>) -> (Vec<Instruction>, HashMap<usize, Vec<String>>) {
    let mut stacks: HashMap<usize, Vec<String>> = HashMap::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    let reg_instruct = Regex::new(r"move (?P<quantity>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();

    lines.into_iter().for_each(|l| {
        if l.contains('[') {
            let nb_stacks = (l.len() + 1) / 4;
            let mut rem = l;
            for i in 0..nb_stacks {
                let key = &i + 1;
                if !stacks.contains_key(&key) {
                    stacks.insert(key, Vec::new());
                }
                let (mut content, rest) = rem.split_at(if i == 0 { 3 } else { 4 });
                content = content.trim();
                if content.len() == 3 {
                    stacks.get_mut(&key).unwrap().splice(0..0, [content[1..2].to_string()]);
                }
                rem = rest.to_string();
            }
        } else if l.contains("move") {
            let result = reg_instruct.captures(&l).unwrap();
            instructions.push(Instruction {
                to: result.name("to").map(|s| s.as_str().parse::<usize>().unwrap()).unwrap(),
                from: result.name("from").map(|s| s.as_str().parse::<usize>().unwrap()).unwrap(),
                quantity: result.name("quantity").map(|s| s.as_str().parse::<i32>().unwrap()).unwrap(),
            })
        }
    });
    (instructions, stacks)
}
