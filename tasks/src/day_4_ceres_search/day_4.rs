#[derive(PartialEq, Debug)]
enum Directions {
    DLT, //diagonal left top
    DLB, //diagonal left bottom
    DRT, //diagonal right top
    DRB, //diagonal right bottom
    VT,  //vertical top
    VB,  //vertical bottom
    HR,  //horizontal right
    HL,  //horizontal left
}

const TOP_DIRECTIONS: [Directions; 3] = [Directions::DLT, Directions::DRT, Directions::VT];
const BOTTOM_DIRECTIONS: [Directions; 3] = [Directions::DLB, Directions::DRB, Directions::VB];
const RIGHT_DIRECTIONS: [Directions; 3] = [Directions::DRB, Directions::DRT, Directions::HR];
const LEFT_DIRECTIONS: [Directions; 3] = [Directions::DLB, Directions::DLT, Directions::HL];

fn check_if_overflow(
    index_row: usize,
    index_column: usize,
    row_len: usize,
    column_len: usize,
    direction: &Directions,
) -> bool {
    (index_row == 0 && (TOP_DIRECTIONS.contains(direction)))
        || (index_row == row_len - 1 && BOTTOM_DIRECTIONS.contains(direction))
        || (index_column == 0 && LEFT_DIRECTIONS.contains(direction))
        || (index_column == column_len - 1 && RIGHT_DIRECTIONS.contains(direction))
}

fn get_next_row_and_column_by_direction(
    index_row: usize,
    index_column: usize,
    direction: &Directions,
) -> (usize, usize) {
    match &direction {
        Directions::DLT => (index_row - 1, index_column - 1),
        Directions::DLB => (index_row + 1, index_column - 1),
        Directions::DRT => (index_row - 1, index_column + 1),
        Directions::DRB => (index_row + 1, index_column + 1),
        Directions::VT => (index_row - 1, index_column),
        Directions::VB => (index_row + 1, index_column),
        Directions::HR => (index_row, index_column + 1),
        Directions::HL => (index_row, index_column - 1),
    }
}

fn is_word_assembled(
    data: &[Vec<char>],
    index_row: usize,
    index_column: usize,
    seeking_char: char,
    direction: Directions,
) -> bool {
    if seeking_char == 'S' && data[index_row][index_column] == 'S' {
        return true;
    }

    if (data[index_row][index_column] != seeking_char)
        || check_if_overflow(
            index_row,
            index_column,
            data.len(),
            data[0].len(),
            &direction,
        )
    {
        return false;
    }

    let next_char = match seeking_char {
        'M' => 'A',
        'A' => 'S',
        _ => 'X',
    };

    let (next_row, next_column) =
        get_next_row_and_column_by_direction(index_row, index_column, &direction);

    is_word_assembled(data, next_row, next_column, next_char, direction)
}

pub fn part_1() -> usize {
    let data = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = 0_usize;
    let row_len = data.len();
    let column_len = data[0].len();

    for (index_row, row) in data.iter().enumerate() {
        for (index_column, &item) in row.iter().enumerate() {
            if item == 'X' {
                res += [
                    Directions::DLT,
                    Directions::DLB,
                    Directions::DRT,
                    Directions::DRB,
                    Directions::VT,
                    Directions::VB,
                    Directions::HR,
                    Directions::HL,
                ]
                .into_iter()
                .fold(0, |acc, direction| {
                    if check_if_overflow(index_row, index_column, row_len, column_len, &direction) {
                        return acc;
                    }
                    let (next_row, next_column) =
                        get_next_row_and_column_by_direction(index_row, index_column, &direction);
                    if is_word_assembled(&data, next_row, next_column, 'M', direction) {
                        return acc + 1;
                    }
                    acc
                });
            }
        }
    }

    res
}

fn check_is_word_assembled(
    left_top: char,
    right_top: char,
    right_bottom: char,
    left_bottom: char,
) -> bool {
    if left_top == right_bottom {
        return false;
    }

    let corner_letters_arr = [left_top, right_top, right_bottom, left_bottom];

    let mut count_m = 0;
    let mut count_s = 0;

    for &c in corner_letters_arr.iter() {
        match c {
            'M' => count_m += 1,
            'S' => count_s += 1,
            _ => return false,
        }
    }

    count_m == count_s
}

pub fn part_2() -> usize {
    let data = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = 0_usize;
    let row_len = data.len();
    let column_len = data[0].len();

    for (index_row, row) in data.iter().enumerate().skip(1) {
        if index_row == row_len - 1 {
            break;
        }
        for (index_column, &item) in row.iter().enumerate().skip(1) {
            if index_column == column_len - 1 {
                break;
            }
            if item == 'A'
                && check_is_word_assembled(
                    data[index_row - 1][index_column - 1],
                    data[index_row - 1][index_column + 1],
                    data[index_row + 1][index_column + 1],
                    data[index_row + 1][index_column - 1],
                )
            {
                res += 1;
            }
        }
    }

    res
}
