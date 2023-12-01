use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use regex::Regex;


fn main() {
    let mut sum = 0;
    // Read input.txt
    if let Ok(lines) = read_lines("./input.txt") {
        // consumes iter, returns opt String
        for line in lines {
            if let Ok(l) = line {
                println!("{:?}", l);
                let re = Regex::new(r"\d").unwrap();
                let digits: Vec<i32> = re.find_iter(&l)
                    .filter_map(|digits| digits.as_str().parse().ok())
                    .collect();
    // for each line get first and last number
                println!("digits: {:?}", &digits);
                let first: i32 = digits[0];
                let last: i32 = digits[digits.len() - 1];
                println!("First number: {}", &first);
                println!("Last number: {:?}", &last);
                let number = format!("{}{}",first,last);
                println!("number: {}", &number);
                sum = sum + number.parse::<i32>().unwrap();
            }
        }
        println!("Sum: {}", sum);
    } else {
        println!("Error");
    }

    // sum all lanes' first and last numbers
}

fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
where P: AsRef<Path>, {
    println!("read lines");
    let file = File::open(filename)?;
    println!("{:?}", file);
    Ok(std::io::BufReader::new(file).lines())
}
