use std::fs::File;
use std::io::{self, BufReader, BufRead};
use itertools::Itertools;
struct Display{
    pattern: Vec<String>,
    output: Vec<String>,
}


fn read(path: &str) -> Result<Vec<Display>, io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = Vec::new();
    for line in br.lines() {
        let line = line?;
        let tokens: Vec<&str> = line.split('|').collect();
        let mut pattern: Vec<String> = tokens[0].split(' ').map(|s| s.to_string()).collect();
        for i in 0..9{
            pattern[i] = pattern[i].chars().sorted().collect::<String>();
        }
        let mut output: Vec<String> = tokens[1].split(' ').map(|s| s.to_string()).collect();
        for i in 0..4{
            output[i] = output[i].chars().sorted().collect::<String>();
        }
        v.push(Display{pattern:pattern, output:output});
        }
    Ok(v)
}

fn contains_string(longer: &String, shorter: &String)->bool{
    for c in shorter.chars(){
        if !longer.contains(c){
            return false;
        }
    }
    true
}

fn main() {
    let input = read("/home/elisabeth/AdventOfCode2021/day8/src/input.txt");
    let mut counter =0;
    if let Ok(vec) = input {
        for display in vec {
            let mut zahlen = vec![" "; 10];
            let mut length_6 = Vec::new();
            let mut length_5 = Vec::new();
            // Find the numbers that we can immediately recognise by length of the string
            for i in 0.. display.pattern.len(){
                if display.pattern[i].len()==7 {
                    zahlen[8] = &display.pattern[i];
                }
                else if display.pattern[i].len()==4{
                    zahlen [4] = &display.pattern[i];
                }
                else if display.pattern[i].len() == 3{
                    zahlen [7] = &display.pattern[i];
                }
                else if display.pattern[i].len()==2{
                    zahlen[1] = &display.pattern[i];
                }
                else if display.pattern[i].len()==5{
                    length_5.push(&display.pattern[i]);
                }
                else if display.pattern[i].len()==6{
                    length_6.push(&display.pattern[i]);
                }
            }
            // Conclue what the other strings correspond to
            for i in 0..3{
                if contains_string(&length_6[i].to_string(), &zahlen[7].to_string()) && contains_string(&length_6[i].to_string(), &zahlen[4].to_string()) {
                    zahlen[9] = length_6[i];
                }
                else if ! contains_string(&length_6[i].to_string(), &zahlen[1].to_string()) {
                    zahlen[6] = length_6[i];
                }
                else{
                    zahlen[0] = length_6[i];
                }
            }
            for i in 0..3{
                if contains_string(&zahlen[6].to_string(), &length_5[i].to_string()){
                    zahlen[5] = length_5[i];
                }
                else if contains_string(&length_5[i].to_string(), &zahlen[7].to_string()){
                    zahlen[3] = length_5[i];
                }
                else {
                    zahlen[2] = length_5[i];
                }
            }
            for i in 0..10{
                print!("{} ist {}",i, zahlen[i]);
            }
            print!("\n");
            let mut coef = 1000;
            for i in 0..display.output.len(){
                for j in 0..10{
                    if contains_string(&display.output[i], &zahlen[j].to_string()) && contains_string(&zahlen[j].to_string(),&display.output[i]){
                        counter += coef*j;
                        coef = coef/10;
                    }
                }
            }
        }
        println!("The result is {}", counter);
    }
    else {
        println!("Something is wrong");
    }
    
}
