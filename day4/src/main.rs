use std::fs::File;
use std::io::{self, BufReader, BufRead};

struct Board{
    entries: [[u32; 5]; 5],
    occupied: [[bool; 5]; 5],
}

// struct specific functions
impl Board {
    fn exists_bingo(&self)->bool {
        for i in 0..5{
            let mut bingo_col = true;
            let mut bingo_row = true;
            for j in 0..5{
                bingo_col = bingo_col && self.occupied[i][j];
                bingo_row = bingo_row && self.occupied[j][i];
            }
            if bingo_col || bingo_row{
                return true;
            }
        }
        false
    }

    fn play(&mut self, n:u32)->bool {
        let mut changed = false;
        for i in 0..5{
            for j in 0..5{
                if self.entries[i][j]== n{
                    self.occupied[i][j]=true;
                    changed= true;
                }
            }
        }
        changed
    }

    fn compute_result(&self, n:u32)->u32{
        let mut sum=0;
        for i in 0..5{
            for j in 0..5{
                if !self.occupied[i][j]{
                    sum +=self.entries[i][j];
                }
            }
        }
        sum*n
    }
}

fn read(path: &str) -> Result<(Vec<u32>,Vec<Board>), io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut lines = br.lines();

    let first_line = lines.next().unwrap().unwrap();

    let v = first_line.split(',')
                               .map(|num| num.parse().unwrap() )
                               .collect();

    let mut bingos =  Vec::<Board>::new();
    while let Some(line)= lines.next() {
        if line.unwrap().is_empty() {
            let mut bingo = Board{entries : [[0;5];5], occupied : [[false;5];5]};
            for i in 0..5 {
                let line = lines.next().unwrap()?;
                for (j, num) in line.split_whitespace().enumerate() {
                    bingo.entries[i][j]=num.parse().unwrap();
                }
            }
            bingos.push(bingo);
        } 
    }
    Ok((v, bingos))

}

fn main() {
    let input = read("/home/elisabeth/AdventOfCode2021/day4/src/input.txt");
    if let Ok((number, mut boards)) = input {
        let mut indices: Vec<usize> = (0..boards.len()).collect();
        for n in number {
            if indices.len() > 1 {
                for &i in &indices {
                    boards[i].play(n);
                }
                indices.retain(|&i| !boards[i].exists_bingo());
            }
            else {
              let i = indices[0];
              if boards[i].play(n) && boards[i].exists_bingo() {
                  println!("result is {}", boards[i].compute_result(n));
                  return;
              }  
            }
        }
        println!("Something went wrong :(");
    }
    else {
        println!("Something is wrong");
    }
    
}

