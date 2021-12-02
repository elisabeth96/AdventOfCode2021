use std::fs::File;
use std::io::{self, BufReader, BufRead, Error, ErrorKind};
enum Direction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn read(path: &str) -> Result<Vec<Direction>, io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = Vec::new();
    for line in br.lines() {
        let line = line?;
        let tokens: Vec<&str> = line.split(' ').collect();
        let n = tokens[1].parse()
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        match tokens[0] {
            "up"=> v.push(Direction::Up(n)),
            "down"=> v.push(Direction::Down(n)),
            "forward"=> v.push(Direction::Forward(n)),
            &_=> panic!()

        }
    }
    Ok(v)
}

fn main() {
    let input = read("/home/elisabeth/AdventOfCode2021/day2/src/input.txt");
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    if let Ok(vec) = input {
        for dir in vec {
            match dir {
                Direction::Up(n)=> aim-=n,
                Direction::Down(n)=> aim+=n,
                Direction::Forward(n)=> {horizontal+=n; depth+=aim*n},
            }
        }
        println!("The result is {}", depth*horizontal);
    }
    else {
        println!("Something is wrong");
    }
    
}
