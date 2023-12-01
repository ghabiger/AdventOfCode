use std::{env, fs};

struct WrittenNumber {
    representation: String,
    value: u32,
}

impl WrittenNumber {
    fn new(representation: String, value: u32) -> Self {
        WrittenNumber {
            representation,
            value,
        }
    }
}

fn read_file_as_string(path: String) -> String {
    fs::read_to_string(path).expect("File should be readable")
}

fn extract_int_from_line(line: &str, start_index: usize, end_index: usize) -> u32 {
    line.get(start_index..end_index).unwrap().parse().unwrap()
}

fn find_digits_in_line(line: &str, number_representations: &Vec<WrittenNumber>) -> (u32, u32) {
    dbg!(line);

    let mut first_digit_index = line.find(char::is_numeric).unwrap();
    let mut first_digit = extract_int_from_line(line, first_digit_index, first_digit_index + 1);
    let mut second_digit_index = line.rfind(char::is_numeric).unwrap();
    let mut second_digit = extract_int_from_line(line, second_digit_index, second_digit_index + 1);

    for rep in number_representations {
        let tmp_index1 = line.find(&rep.representation).unwrap_or(line.len() + 1);
        if tmp_index1 < first_digit_index {
            first_digit_index = tmp_index1;
            first_digit = rep.value;
        }

        let tmp_index2 = line.rfind(&rep.representation).unwrap_or(0);
        if tmp_index2 > second_digit_index {
            second_digit_index = tmp_index2;
            second_digit = rep.value;
        }
    }

    (first_digit, second_digit)
}

fn main() {
    let number_representations = vec![
        WrittenNumber::new(String::from("one"), 1),
        WrittenNumber::new(String::from("two"), 2),
        WrittenNumber::new(String::from("three"), 3),
        WrittenNumber::new(String::from("four"), 4),
        WrittenNumber::new(String::from("five"), 5),
        WrittenNumber::new(String::from("six"), 6),
        WrittenNumber::new(String::from("seven"), 7),
        WrittenNumber::new(String::from("eight"), 8),
        WrittenNumber::new(String::from("nine"), 9),
        WrittenNumber::new(String::from("zero"), 0),
    ];

    let args: Vec<String> = env::args().collect();
    let contents = read_file_as_string(args[1].clone());

    let mut sum = 0;
    for line in contents.lines() {
        let (first_digit, second_digit) = find_digits_in_line(line, &number_representations);
        println!("Found digits in line: {}, {}", first_digit, second_digit);
        sum += 10 * first_digit + second_digit;
    }

    println!("Sum is: {}", sum);
}
