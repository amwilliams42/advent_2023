mod days;

use std::fs::File;
use std::path::{Path, PathBuf};
use crate::RunType::Validate;

enum RunType {
    Test(bool),
    Validate
}

pub(crate) fn input_file(day: i32, run_type: RunType) -> File {
    let folder: &str = match run_type {
        RunType::Test(part_one) => {match part_one {
            true => {"test_1"}
            false => {"test_2"}
            }
        }
        RunType::Validate => {"validate"}
    };
    let path: PathBuf = Path::new(".").join("inputs")
        .join(folder)
        .join(format!("day_{}.txt",day));

    let file = match File::open(path){
        Ok(file) => file,
        Err(error) => panic!("Problem opening file for day {:?}. {:?}",day, error )
    };

    return file;
}

fn main() {
    println!("{:?}", days::day_1::run_part_1(input_file(1, Validate)))
}
