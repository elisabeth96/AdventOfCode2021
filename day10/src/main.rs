use std::fs::File;
use std::io::{self, BufReader, BufRead};


fn read(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = Vec::new();
    for line in br.lines() {
        let line = line?;
        v.push(line.to_string());
        }
    Ok(v)
}

fn main() {
    let input = read("/home/elisabeth/AdventOfCode2021/day10/src/input.txt");
    let mut sum =0;
    let mut scores = Vec::new();
    let mut broken;
    if let Ok(vec) = input {
        for line in vec {
            // corresponds to ()|[]|<>|{}
            let mut queue = Vec::new();
            broken = false;
            let mut result:i128 =0;
            for c in line.chars(){
                match c {
                    // Match a single value
                    '(' => queue.push(')'),
                    '[' => queue.push(']'),
                    '<' => queue.push('>'),
                    '{' => queue.push('}'),
                    ')' =>{
                        if queue.pop().unwrap() != c{
                            sum += 3;
                            broken = true;
                            break;
                        }
                    } 
                    ']' => {
                        if queue.pop().unwrap() != c{
                            sum += 57;
                            broken = true;
                            break;
                        }
                    } 
                    '>' => {
                        if queue.pop().unwrap() != c{
                            sum += 25137;
                            broken = true;
                            break;
                        }
                    } 
                    '}' => {
                        if queue.pop().unwrap() != c{
                            sum += 1197;
                            broken = true;
                            break;
                        }
                    } 
                    // Handle the rest of cases
                    _ => println!("Error"),
                }
            }
            while queue.len()>0 && !broken{
                let c = queue.pop().unwrap();
                result *=5;
                match c {
                    ')' => result += 1,
                    ']' => result += 2,
                    '}' => result += 3,
                    '>' => result += 4,
                    _ => println!("Error"),
                }
            }
            if !broken{
                scores.push(result);
            }
        }
        scores.sort();
        println!("The result of the first part is {}.", sum);
        println!("The result of the second part is {}.", scores[scores.len()/2]);
    }
    else {
        println!("Something is wrong");
    }
    
}
