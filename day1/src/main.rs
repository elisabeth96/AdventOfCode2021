use std::fs::File;
use std::io::{self, BufReader, BufRead, Error, ErrorKind};



fn read(path: &str) -> Result<Vec<i64>, io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = Vec::new();
    for line in br.lines() {
        let line = line?;
        let n = line   
            .trim() // trim "whitespaces"
            .parse() // call `str::parse::<i64>(&self)` method on the trimmed line, which parses integer
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?; // parse() can return error (e.g. for string "abc"), here if we got it, we convert it to `std::io::Error` type and return it as function result
        v.push(n);
    }
    Ok(v)
}

fn main() {
    let input = read("/home/elisabeth/AdventOfCode2021/day1/src/input.txt");
    let mut count = 0;
    if let Ok(vec) = input {
        for i in 3..vec.len() {
            let sum1=vec[i]+vec[i-1]+vec[i-2];
            let sum0=vec[i-3]+vec[i-1]+vec[i-2];
            if sum1 > sum0{
                count= count+1;
            }
        }
        println!("The result is {}", count);
    }
    else {
        println!("Something is wrong");
    }
    
}
