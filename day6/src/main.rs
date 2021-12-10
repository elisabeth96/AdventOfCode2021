use std::fs::File;
use std::io::{self, BufReader, BufRead};



fn change_time(individuals: &mut [u128;9]){
    let temp = individuals.clone();
    for i in 1..9{
        individuals[i-1] = temp[i];
    }
    individuals[6] += temp[0];
    individuals[8] = temp[0];
}

fn read(path: &str) -> Result<Vec<i32>, io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);

    let mut v = Vec::new();
    for line in br.lines() {
        let line = line?;
        for s in line.split(',') {
            v.push( s.trim().parse().unwrap());
        }
    }
    Ok(v)
}

fn main() {
    let input = read("/home/elisabeth/AdventOfCode2021/day6/src/input.txt");
    if let Ok(inputs) = input {
        let mut individuals:[u128;9] =[0;9];
        for i in inputs{
            individuals[i as usize] += 1;
        }

        for _ in 0..256{
            change_time(&mut individuals);
        }
        let mut sum:u128 = 0;
        for i in 0..9{
            sum += individuals[i as usize];
        }

        println!("result {}", sum);
    }
    else {
        println!("Something is wrong");
    }
    
}

