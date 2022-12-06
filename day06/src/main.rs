use common;
use itertools::Itertools;

fn main() {
    let cli_args = common::parse_cli_args();
    common::read_lines(&cli_args.input_path).unwrap()
        .into_iter()
        .for_each(|l| {
            let signs: Vec<char> = l.chars().collect();
            println!("Packet start index is {:?}", get_packet_index(signs.clone(), 4));
            println!("Packet message index is {:?}", get_packet_index(signs.clone(), 14));
        });
}

fn get_packet_index(signs: Vec<char>, packet_length: usize) -> usize {
    let mut index: usize = 0;
    for i in 0..signs.len() - 3 {
        let packet_idx = i + packet_length;
        let slice_start = &signs[i..packet_idx].into_iter().unique().collect::<Vec<&char>>();
        if slice_start.len() >= packet_length {
            index = packet_idx;
            break;
        }
    }
    index
}
