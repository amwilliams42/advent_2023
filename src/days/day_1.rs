use std::io::{BufRead, BufReader, Read};


const NUMS: [&str; 9] = [
  "one","two","three","four","five","six","seven","eight","nine"
];

fn get_number_from_line(line: &str) -> Option<u32> {

    let mut numbers = line.chars().filter(|x| x.is_ascii_digit() == true)
                .map(|c| c.to_digit(10).unwrap());

    let first = match numbers.next() {
        None => return None,
        Some(n) => n
    };

    let last = match numbers.last() {
        None => first,
        Some(n) => n
    };

    Some(vec![first, last].iter().fold(0, |a,i| a*10+i))

}

pub fn run_part_1<R: Read>(io: R) -> u32 {
    let bufread = BufReader::new(io);

    bufread.lines()
        .filter_map(|result| result.ok())
        .map(|l|{
            let first = l.as_bytes().iter().position(|c| c.is_ascii_digit()).unwrap();
            let last = l.as_bytes().iter().rposition(|c|c.is_ascii_digit()).unwrap();

            let mut a = (l.as_bytes()[first]-b'0') as u32;
            let mut b = (l.as_bytes()[last]-b'0') as u32;

            let mut a_pos = first;
            let mut b_pos = last;

            for (idx, num) in NUMS.iter().enumerate() {
                if let Some(pos) = l[..first].find(num) {
                    if pos < a_pos {
                        a_pos = pos;
                        a = (idx + 1) as u32;
                    }
                }
            }

            for (idx, num) in NUMS.iter().enumerate() {
                if let Some(pos) = l[last..].rfind(num) {
                    if pos + last > b_pos {
                        b_pos = pos + last;
                        b = (idx + 1) as u32;
                    }
                }
            }

            a * 10+b
        }).sum()


}

#[cfg(test)]
mod tests {
    use crate::days::day_1::{get_number_from_line, run_part_1};
    use crate::input_file;
    use crate::RunType::Test;

    #[test]
    fn test_get_number_from_line() {
        assert_eq!(get_number_from_line("a1b2c3d4e5f"), Some(15));
        assert_eq!(get_number_from_line("treb7uchet"), Some(77));
    }
    #[test]
    fn test_input() {
        let io = input_file(1, Test(true));
        let io2 = input_file(1, Test(false));
        assert_eq!(run_part_1(io), 142);
        assert_eq!(run_part_1(io2), 281);
    }
}