use std::fs::File;
use std::io::{self, BufReader, BufRead};

struct Index{
    x: usize,
    y: usize,
}

fn read(path: &str) -> Result<Vec<Vec<u32>>, io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);

    let mut v = Vec::new();
    let mut index =0;
    for line in br.lines() {
        v.push(Vec::new());
        let line = line?;
        for c in line.chars() {
            v[index].push( c.to_digit(10).unwrap());
        }
        index += 1;
    }
    Ok(v)
}


fn check_lowest_point (board:&Vec<Vec<u32>>, index:Index)->bool{
    if index.x>0 && board[index.x][index.y] >= board[index.x-1][index.y]{
        return false;
    }
    if index.x<board.len()-1 && board[index.x][index.y] >= board[index.x+1][index.y]{
        return false;
    }
    if index.y > 0 && board[index.x][index.y] >= board[index.x][index.y-1]{
        return false;
    }
    if index.y+1<board[0].len() && board[index.x][index.y] >= board[index.x][index.y+1]{
        return false;
    }
    else{
        return true;
    }
}

fn size_basin(board:&Vec<Vec<u32>>, index:Index)->i32{
    let mut occupied: Vec<Vec<bool>> = Vec::new();
    for i in 0..board.len(){
        occupied.push(Vec::new());
        for _ in 0..board[0].len(){
            occupied[i].push(false);
        }
    }
    let mut to_check:Vec<Index> = Vec::new();
    let mut size=1;
    occupied[index.x][index.y]=true;
    to_check.push(index);
    while to_check.len()>0{
        let middle_index = to_check.pop().unwrap();
        if middle_index.x>0 && board[middle_index.x][middle_index.y] < board[middle_index.x-1][middle_index.y] && board[middle_index.x-1][middle_index.y]<9 && !occupied[middle_index.x-1][middle_index.y]{
            size +=1;
            to_check.push(Index{x:middle_index.x-1, y:middle_index.y});
            occupied[middle_index.x-1][middle_index.y] = true;
        }
        if middle_index.x+1<board.len() && board[middle_index.x][middle_index.y] < board[middle_index.x+1][middle_index.y] && board[middle_index.x+1][middle_index.y]<9 &&!occupied[middle_index.x+1][middle_index.y]{
            size +=1;
            to_check.push(Index{x:middle_index.x+1, y:middle_index.y});
            occupied[middle_index.x+1][middle_index.y] = true;
        }
        if middle_index.y > 0 && board[middle_index.x][middle_index.y] < board[middle_index.x][middle_index.y-1] && board[middle_index.x][middle_index.y-1]<9 && !occupied[middle_index.x][middle_index.y-1]{
            size +=1;
            to_check.push(Index{x:middle_index.x, y:middle_index.y-1});
            occupied[middle_index.x][middle_index.y-1] = true;
        }
        if middle_index.y+1<board[0].len() && board[middle_index.x][middle_index.y] < board[middle_index.x][middle_index.y+1] && board[middle_index.x][middle_index.y+1]<9 && !occupied[middle_index.x][middle_index.y+1]{
            size +=1;
            to_check.push(Index{x:middle_index.x, y:middle_index.y+1});
            occupied[middle_index.x][middle_index.y+1] = true;
        }
    }
    size
}
fn main() {
    let input = read("/home/elisabeth/AdventOfCode2021/day9/src/input.txt");
    if let Ok(inputs) = input {
        let mut biggest =[0;3];
        for i in 0..inputs.len(){
            for j in 0..inputs[0].len(){
                if check_lowest_point(&inputs, Index{x:i, y:j}){
                    let size = size_basin(&inputs, Index{x:i, y:j});
                    if size> biggest[0]{
                        biggest[2]=biggest[1];
                        biggest[1]=biggest[0];
                        biggest[0]=size;
                    }
                    else if size>biggest[1]{
                        biggest[2]=biggest[1];
                        biggest[1]=size;
                    }
                    else if size>biggest[2] {
                        biggest[2]=size;
                    }
                }
            }
        }
        let product:i128 = (biggest[0]*biggest[1]*biggest[2]) as i128;
        println!("{}, {}, {}", biggest[0],biggest[1],biggest[2]);
        println!("{}", product);
    }
    else {
        println!("Something is wrong");
    }
    
}

