use common;

fn main() {
    let cli_args = common::parse_cli_args();
    let lines = common::read_lines(&cli_args.input_path).unwrap();
    let (rows, columns) = parse_grid(lines);

    let mut nb_visible = 0;
    let max_x = rows.clone().len() - 1;
    let max_y = columns.clone().len() - 1;
    let mut max_scenic_score = 0;
    rows.into_iter().enumerate().for_each(|(y, row)| {
        row.clone().into_iter().enumerate().for_each(|(x, cur_height)| {
            let mut is_on_edge = false;
            if x == 0 || x == max_x || y == 0 || y == max_y {
                is_on_edge = true;
            }

            let mut row_before: Vec<i8> = row[0..x].to_vec();
            let row_after: Vec<i8> = row[x + 1..max_x + 1].to_vec();
            let mut col_before: Vec<i8> = columns[x][0..y].to_vec();
            let col_after: Vec<i8> = columns[x][y + 1..max_y + 1].to_vec();

            if is_on_edge ||
                !is_contains_upper_tree(&row_before, &cur_height) ||
                !is_contains_upper_tree(&row_after, &cur_height) ||
                !is_contains_upper_tree(&col_before, &cur_height) ||
                !is_contains_upper_tree(&col_after, &cur_height) {
                nb_visible += 1;
            }

            row_before.reverse();
            col_before.reverse();
            let scenic_score = nb_tree_visible(row_before, cur_height) *
                nb_tree_visible(row_after, cur_height) *
                nb_tree_visible(col_before, cur_height) *
                nb_tree_visible(col_after, cur_height);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        })
    });
    dbg!(nb_visible);
    dbg!(max_scenic_score);
}

fn parse_grid(lines: Vec<String>) -> (Vec<Vec<i8>>, Vec<Vec<i8>>) {
    let mut columns: Vec<Vec<i8>> = Vec::new();
    let rows: Vec<Vec<i8>> = lines.clone().into_iter()
        .map(|l|
            l.chars().collect::<Vec<char>>()
                .into_iter()
                .map(|c| c.to_string().parse::<i8>().unwrap()).collect::<Vec<i8>>()
        ).collect();
    for i in 0..lines.len() {
        let mut column: Vec<i8> = Vec::new();
        rows.clone().into_iter().for_each(|row| {
            column.push(row[i]);
        });
        columns.push(column);
    }
    (rows, columns)
}

fn is_contains_upper_tree(trees: &Vec<i8>, height: &i8) -> bool {
    trees.into_iter().filter(|&h| h >= height).collect::<Vec<&i8>>().len() > 0
}

fn nb_tree_visible(trees: Vec<i8>, height: i8) -> usize {
    let mut nb_visible = 0;
    for i in 0..trees.clone().len() {
        nb_visible += 1;
        if trees[i] >= height {
            break;
        }
    }
    nb_visible
}
