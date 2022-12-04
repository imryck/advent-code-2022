use common;

type Range = (i32, i32);

#[derive(Debug, Clone)]
struct PairRange {
    contained: bool,
    overlap: bool,
}

fn main() {
    let cli_args = common::parse_cli_args();
    let pair_ranges: Vec<PairRange> = common::read_lines(&cli_args.input_path).unwrap().into_iter()
        .map(|line| {
            let ranges: Vec<String> = line.split(",").map(str::to_string).collect();
            let r1 = parse_range(ranges[0].clone());
            let r2 = parse_range(ranges[1].clone());
            PairRange {
                contained: is_ranges_contained(r1, r2),
                overlap: is_ranges_overlap(r1, r2),
            }
        })
        .collect::<Vec<PairRange>>();

    println!("Number contained {:#?} ", pair_ranges.clone().into_iter().filter(|p| p.contained).count());
    println!("Number overlap {:#?} ", pair_ranges.clone().into_iter().filter(|p| p.overlap).count());
}

fn parse_range(range: String) -> Range {
    let parts: Vec<i32> = range.split("-").map(|i| i.parse::<i32>().unwrap()).collect();
    (parts[0], parts[1])
}

fn is_ranges_contained(r1: Range, r2: Range) -> bool {
    (r1.0 <= r2.0 && r2.1 <= r1.1) || (r2.0 <= r1.0 && r1.1 <= r2.1)
}


fn is_ranges_overlap(r1: Range, r2: Range) -> bool {
    r1.0 <= r2.1 && r2.0 <= r1.1
}
