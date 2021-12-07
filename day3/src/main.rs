use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn read(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = Vec::new();
    for line in br.lines() {
        let line = line?;
        v.push(line);
    }
    Ok(v)
}

fn filter(mut lines: Vec<String>, mode: i32)-> String {
    let n = lines[0].len();
    for i in 0..n {

        let one_count: usize = (&lines).into_iter()
                                       .filter(|&line| line.chars().nth(i).unwrap() == '1' )
                                       .count();

        
        let item = {
        if mode == 1 {
            if one_count as f32 >= lines.len() as f32 / 2. { '1' } else { '0' }
        }
        else {
            if (one_count as f32) < lines.len() as f32 / 2. { '1' } else { '0' }
        }
        };

        lines.retain(|line| line.chars().nth(i).unwrap() == item);
        if lines.len() == 1 {
            break;
        }
    }
    return lines.pop().unwrap();
}

fn to_number(binary : &str) -> i32 {
    let mut sum = 0;
    let mut pow = 1;
    for c in binary.chars().rev() {
        if c == '1' {
            sum += pow;
        } 
        pow *= 2;
    }
    sum
}

fn main() {
    let input = read("/home/elisabeth/AdventOfCode2021/day3/src/input.txt");
    if let Ok(lines) = input {

        let oxy = filter(lines.clone(), 1);
        let co2 = filter(lines, 0);

        println!("{}", oxy);
        println!("{}", co2);

        println!("The result is {}", to_number(&oxy)*to_number(&co2));
    }
    else {
        println!("Something is wrong");
    }
    
}
