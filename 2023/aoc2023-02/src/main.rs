use std::fs;

#[derive(Debug)]
struct Game {
    game_number: u32,
    power: u32,
    is_possible: bool,
}

impl Game {
    fn new(
        game_number: u32,
        min_red: u32,
        min_green: u32,
        min_blue: u32,
        is_possible: bool,
    ) -> Game {
        let power = min_red * min_green * min_blue;
        Game {
            game_number,
            power,
            is_possible,
        }
    }
}

fn decode_line_to_game(
    line: &str,
    red_contained: u32,
    green_contained: u32,
    blue_contained: u32,
) -> Game {
    let prefix_end = line.find(": ").unwrap();
    let game = line.get(0..prefix_end).unwrap();
    let game_number: u32 = game[(game.find(" ").unwrap() + 1)..].parse().unwrap();
    let all_draws = &line[(prefix_end + 2)..];

    let mut is_possible = true;
    let mut max_red = 0u32;
    let mut max_green = 0u32;
    let mut max_blue = 0u32;
    let draws = all_draws.split("; ");
    for draw in draws {
        let cubes = draw.split(", ");

        for color_with_count in cubes {
            let count_and_color: Vec<&str> = color_with_count.split(" ").collect();
            let count: u32 = count_and_color[0].parse().unwrap();
            let color = count_and_color[1];
            match color {
                "red" => {
                    if count > red_contained {
                        is_possible = false;
                    }
                    if count > max_red {
                        max_red = count;
                    }
                }
                "green" => {
                    if count > green_contained {
                        is_possible = false;
                    }
                    if count > max_green {
                        max_green = count;
                    }
                }
                "blue" => {
                    if count > blue_contained {
                        is_possible = false;
                    }
                    if count > max_blue {
                        max_blue = count;
                    }
                }
                _ => (),
            }
        }
    }

    Game::new(game_number, max_red, max_green, max_blue, is_possible)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File should be readable");

    let red_contained: u32 = 12;
    let green_contained: u32 = 13;
    let blue_contained: u32 = 14;

    let mut sum_possible_games = 0;
    let mut sum_power_all_games = 0;
    for line in input.lines() {
        let game = decode_line_to_game(line, red_contained, green_contained, blue_contained);
        sum_power_all_games += game.power;
        println!("{line}");
        if game.is_possible {
            sum_possible_games += game.game_number;
            println!("== Game is possible!");
            dbg!(game);
        }
    }

    println!(
        "Sum of game numbers of possible games: {}",
        sum_possible_games
    );
    println!("Sum of powers in all games: {}", sum_power_all_games);
}
