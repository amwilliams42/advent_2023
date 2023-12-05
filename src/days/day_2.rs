use std::io::{BufRead, BufReader, Read};


pub fn run_part_1<R: Read>(io: R) -> i32{


    BufReader::new(io).lines()
        .filter_map(|r| r.ok())
        .map(|mut line| {
            line = line.trim().strip_prefix("Game ").unwrap().parse().unwrap();
            let (game, draws) = line.split_once(": ").expect("Parse Failure");
            let game_num = game.parse::<i32>().unwrap();

            let poss = draws.split("; ").all(|draw| {
                draw.split(", ").all(|drawn_cube| {
                    let cubes = drawn_cube.split(" ").collect::<Vec<&str>>();
                    let cube_num = cubes[0].parse::<i32>().unwrap();
                    let cube_col = cubes[1];
                    match cube_col{
                        "red" => cube_num <= 12,
                        "green" => cube_num <= 13,
                        "blue" => cube_num <= 14,
                        &_ => panic!("Parse Failure")
                    }
                })
            });

            match poss {
                true => game_num,
                false => 0
            }

        }).sum()

}

struct Mins {
    red: i32,
    green: i32,
    blue: i32,
}

impl Mins {
    pub fn power(self) -> i32{
        self.red * self.blue * self.green
    }

    pub fn update_min(&mut self, color: &str, num: i32) {
        match color{
            "red" => {if self.red < num {
                self.red = num
            }},
            "green" => {if self.green < num {
                self.green = num
            }},
            "blue" => {if self.blue < num {
                self.blue = num
            }},
            &_ => panic!("Failure updating mins, incorrect color")
        };
    }
}

pub fn run_part_2<R: Read>(io: R) -> i32{

    BufReader::new(io).lines().filter_map(|r| r.ok()).map(|mut line| {
        line = line.trim().strip_prefix("Game ").unwrap().parse().unwrap();

        let (_, draws) = line.split_once(": ").expect("Parse Failure");
        let mut mins = Mins{ //Rolling minimum per game
            red: 0,
            green: 0,
            blue: 0,
        };

        for draw in draws.split("; ") {
            for cube in draw.split(", ") {
                let c = cube.split(" ").collect::<Vec<&str>>();
                mins.update_min(c[1], c[0].parse::<i32>().unwrap());
            }
        }

        mins.power()

    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::days::day_2::{Mins, run_part_1, run_part_2};
    use crate::input_file;
    use crate::RunType::Test;

    #[test]
    fn test_input() {
        let io = input_file(2, Test(true));
        assert_eq!(run_part_1(io), 8);
    }

    #[test]
    fn test_part_2() {
        let io = input_file(2, Test(true));
        assert_eq!(run_part_2(io), 2286);
    }

    #[test]
    fn test_min_update() {

        let mut mins = Mins{
            red: 0,
            green: 0,
            blue: 0,
        };

        assert_eq!(mins.red, 0);
        mins.update_min("red", 5);
        assert_eq!(mins.red, 5);
        mins.update_min("red", 3);
        assert_eq!(mins.red, 5);
    }
}