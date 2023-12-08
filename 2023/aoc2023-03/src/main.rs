use std::{fs, str};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position {
    line: usize,
    index: usize,
}

// given any position in the schematic, finds the start of a contiguous number and then returns it
// as a single correct value (e.g., with schematic ..345.. and pos 4 given, it will find the start
// at position 2, then return 345 as a usize). Returns None when no number is found at position.
// Also returns the modified line (with the number extracted and replaced with '.')
fn extract_num_from_position(line: &str, from_position: Position) -> (String, Option<usize>) {
    let mut new_line = line.to_string();
    if from_position.index + 1 > line.len() {
        return (new_line, None);
    }
    // find start of number
    let starting_pos =
        match line[..from_position.index + 1].rfind(|c| char::is_ascii_punctuation(&c)) {
            Some(n) => {
                if n == from_position.index {
                    return (new_line, None);
                } else {
                    n + 1
                }
            }
            None => {
                if char::is_ascii_digit(&line.chars().nth(0).unwrap()) {
                    // rfind in the match didn't find an index because the number starts at index 0
                    0
                } else {
                    return (new_line, None);
                }
            }
        };

    // find end of number
    let ending_pos: usize = match line[starting_pos..].find(|c| !char::is_ascii_digit(&c)) {
        Some(n) => {
            if n + starting_pos >= line.len() {
                line.len()
            } else {
                n + starting_pos
            }
        }
        None => {
            if char::is_ascii_digit(&line.chars().nth(line.len() - 1).unwrap()) {
                line.len()
            } else {
                panic!(
                    "Couldn't find end of number in {} with start at {}",
                    line, starting_pos
                );
            }
        }
    };

    // extract number
    let num: usize = line[starting_pos..ending_pos].parse().unwrap();
    let replace = vec![46; ending_pos - starting_pos];
    let replace = str::from_utf8(&replace).unwrap();
    new_line.replace_range(starting_pos..ending_pos, replace);

    (new_line, Some(num))
}

fn find_next_symbol(schematic: &Vec<String>, starting_position: Position) -> Option<Position> {
    let mut cur_pos = starting_position.index;
    let mut found_line: usize = 0;
    let mut found_pos: usize = 0;
    for line_no in starting_position.line..schematic.len() {
        match schematic[line_no][cur_pos..].find(|c| c != '.' && char::is_ascii_punctuation(&c)) {
            Some(n) => {
                found_pos = n + cur_pos;
                found_line = line_no;
                break;
            }
            None => {
                if line_no < schematic.len() - 1 {
                    cur_pos = 0;
                    continue;
                } else {
                    return None;
                }
            }
        };
    }

    Some(Position {
        line: found_line,
        index: found_pos,
    })
}

fn calc_gear(schematic: &Vec<String>, adjacent_pos: &Vec<Position>) -> Option<usize> {
    let mut found_numbers: Vec<usize> = vec![];
    let mut cloned_schem = schematic.clone();
    for pos in adjacent_pos {
        let (new_line, n) = extract_num_from_position(&cloned_schem[pos.line], *pos);
        cloned_schem[pos.line] = new_line;
        if n.is_some() {
            found_numbers.push(n.unwrap());
        }
    }

    if found_numbers.len() == 2 {
        found_numbers.into_iter().reduce(|acc, e| acc * e)
    } else {
        None
    }
}

fn get_adjacent_positions(schematic: &Vec<String>, from_position: Position) -> Vec<Position> {
    let mut valid_pos: Vec<Position> = Vec::new();

    if from_position.line > 0 && from_position.index > 0 {
        valid_pos.push(Position {
            line: from_position.line - 1,
            index: from_position.index - 1,
        });
    }

    if from_position.line > 0 {
        valid_pos.push(Position {
            line: from_position.line - 1,
            index: from_position.index,
        });
    }

    if from_position.line > 0 && from_position.index < schematic[0].len() - 1 {
        valid_pos.push(Position {
            line: from_position.line - 1,
            index: from_position.index + 1,
        });
    }

    if from_position.index > 0 {
        valid_pos.push(Position {
            line: from_position.line,
            index: from_position.index - 1,
        });
    }

    if from_position.index < schematic[0].len() - 1 {
        valid_pos.push(Position {
            line: from_position.line,
            index: from_position.index + 1,
        });
    }

    if from_position.line < (schematic.len() - 1) && from_position.index > 0 {
        valid_pos.push(Position {
            line: from_position.line + 1,
            index: from_position.index - 1,
        });
    }

    if from_position.line < (schematic.len() - 1) {
        valid_pos.push(Position {
            line: from_position.line + 1,
            index: from_position.index,
        });
    }

    if from_position.line < (schematic.len() - 1) && from_position.index < (schematic[0].len() - 1)
    {
        valid_pos.push(Position {
            line: from_position.line + 1,
            index: from_position.index + 1,
        });
    }

    valid_pos
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File should be readable");

    let mut schematic: Vec<String> = Vec::new();
    let mut new_line: String;
    let mut n: Option<usize>;
    for line in input.lines() {
        schematic.push(line.to_string());
    }

    // TODO loop through schematic with find_next_symbol starting at (0,0)
    // then when a symbol is found, search all adjacent positions for numbers and remember those
    // for each found number, extract it (= modify schematic so the numbers are replace by .)
    // sum up extracted numbers, proceed
    let mut cur_pos = Position { line: 0, index: 0 };
    let mut sum: usize = 0;
    let mut sum_gears: usize = 0;
    'outer: loop {
        // find all valid adjacent positions for next symbol in schematic, replacing said symbol
        // handle gears on the fly
        let adjacent_pos = match find_next_symbol(&schematic, cur_pos) {
            Some(found_pos) => {
                cur_pos = found_pos;

                // get all positions adjacent to found pos
                let valid_pos = get_adjacent_positions(&schematic, cur_pos);
                let adjacent = valid_pos.clone();
                let symbol = &schematic[cur_pos.line][cur_pos.index..cur_pos.index + 1];
                // handle gears
                if symbol == "*" {
                    let gear_ratio = calc_gear(&schematic, &valid_pos);
                    if gear_ratio.is_some() {
                        // println!(
                        //     "Found gear ({}) with ratio {} at {} {}",
                        //     symbol,
                        //     gear_ratio.unwrap(),
                        //     cur_pos.line,
                        //     cur_pos.index
                        // );
                        // it is a gear, so has exactly two adjacent numbers we can multiply
                        sum_gears += gear_ratio.unwrap();
                    }
                }

                // replace found symbol with '.'
                new_line = schematic[cur_pos.line].clone();
                new_line.replace_range(cur_pos.index..cur_pos.index + 1, ".");
                schematic[cur_pos.line] = new_line;

                // yield adjacent pos to outer variable adjacent_pos
                adjacent
            }
            // the find_next_symbol method will return None when it can't find any remaining symbol
            None => break 'outer,
        };

        // replace and sum all numbers in valid adjacent positions
        for pos in adjacent_pos {
            (new_line, n) = extract_num_from_position(&schematic[pos.line], pos);
            schematic[pos.line] = new_line;
            if n.is_some() {
                sum += n.unwrap()
            }
        }
    }

    println!("Sum of all parts adjacent to a symbol: {}", sum);
    println!("Sum of all gear ratios: {}", sum_gears);
}

#[cfg(test)]
mod tests {
    use crate::{extract_num_from_position, find_next_symbol, get_adjacent_positions, Position};

    #[test]
    fn test_extract_num() {
        let test_str = String::from("1234567890...573...2...*12...999");
        let line = &test_str[..];
        let test_pos1 = Position { line: 0, index: 0 };
        let test_pos2 = Position { line: 0, index: 0 };
        let test_pos3 = Position { line: 0, index: 4 };
        let test_pos4 = Position { line: 0, index: 14 };
        let test_pos5 = Position { line: 0, index: 19 };
        let test_pos6 = Position { line: 0, index: 24 };
        let test_pos7 = Position { line: 0, index: 29 };
        let test_pos8 = Position { line: 0, index: 33 };
        let (new_line, n) = extract_num_from_position(line, test_pos1);
        assert_eq!(
            (new_line.clone(), n),
            (
                String::from(".............573...2...*12...999"),
                Some(1234567890)
            )
        );
        let (new_line2, n2) = extract_num_from_position(&new_line, test_pos2);
        assert_eq!(
            (new_line2.clone(), n2),
            (String::from(".............573...2...*12...999"), None)
        );
        let (new_line3, n3) = extract_num_from_position(&new_line2, test_pos3);
        assert_eq!(
            (new_line3.clone(), n3),
            (String::from(".............573...2...*12...999"), None)
        );
        let (new_line4, n4) = extract_num_from_position(&new_line3, test_pos4);
        assert_eq!(
            (new_line4.clone(), n4),
            (String::from("...................2...*12...999"), Some(573))
        );
        let (new_line5, n5) = extract_num_from_position(&new_line4, test_pos5);
        assert_eq!(
            (new_line5.clone(), n5),
            (String::from(".......................*12...999"), Some(2))
        );
        let (new_line6, n6) = extract_num_from_position(&new_line5, test_pos6);
        assert_eq!(
            (new_line6.clone(), n6),
            (String::from(".......................*.....999"), Some(12))
        );
        let (new_line7, n7) = extract_num_from_position(&new_line6, test_pos7);
        assert_eq!(
            (new_line7.clone(), n7),
            (String::from(".......................*........"), Some(999))
        );
        let (new_line8, n8) = extract_num_from_position(&new_line7, test_pos8);
        assert_eq!(
            (new_line8.clone(), n8),
            (String::from(".......................*........"), None)
        );
    }

    #[test]
    fn test_valid_pos() {
        let line0 = String::from("*..");
        let line1 = String::from("...");
        let line2 = String::from("...");
        let schematic = vec![line0, line1, line2];
        let pos = Position { line: 0, index: 0 };

        let valid_pos = vec![
            Position { line: 0, index: 1 },
            Position { line: 1, index: 0 },
            Position { line: 1, index: 1 },
        ];

        assert_eq!(get_adjacent_positions(&schematic, pos), valid_pos);
    }

    #[test]
    fn test_find_next_symbol() {
        let line0 = String::from("...");
        let line1 = String::from("*..");
        let line2 = String::from("../");
        let mut schematic = vec![line0, line1, line2];
        let pos1 = Position { line: 0, index: 0 };
        let pos2 = Position { line: 1, index: 0 };
        let pos3 = Position { line: 2, index: 2 };

        dbg!(&schematic);
        assert_eq!(
            find_next_symbol(&schematic, pos1),
            Some(Position { line: 1, index: 0 })
        );
        schematic[1].replace_range(0..1, ".");
        dbg!(&schematic);
        assert_eq!(
            find_next_symbol(&schematic, pos2),
            Some(Position { line: 2, index: 2 })
        );
        schematic[2].replace_range(2..3, ".");
        dbg!(&schematic);
        assert_eq!(find_next_symbol(&schematic, pos3), None);
    }
}
