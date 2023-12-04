use std::io::{BufRead, BufReader, Read};


pub fn run_part_1<R: Read>(io: R) {
    let bufread = BufReader::new(io);

    for line in bufread.lines().filter_map(|r| r.ok()) {
        let mut iter = line.split_ascii_whitespace().collect::<Vec<_>>().chunks(2);
        iter.next();

        for pull in iter {
            println!("{:?}", pull)
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::days::day_2::{run_part_1};
    use crate::input_file;
    use crate::RunType::Test;

    #[test]
    fn test_input() {
        let io = input_file(2, Test(true));
        assert_eq!(run_part_1(io), ());
    }
}