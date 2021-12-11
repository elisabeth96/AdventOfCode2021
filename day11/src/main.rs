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

fn reset_board (board:&mut Vec<Vec<u32>>, flashed:&mut Vec<Vec<bool>>)->i32{
    let mut counter =0;
    for i in 0..board.len(){
        for j in 0..board[0].len(){
            if flashed[i][j]{
                board[i][j]=0;
                flashed[i][j]=false;
                counter += 1;
            }
        }
    }
    counter
}

fn increase_neighbours (board:&mut Vec<Vec<u32>>, flashed:&mut Vec<Vec<bool>>, index_first:Index){
    let mut queue = Vec::new();
    flashed[index_first.x][index_first.y] = true;
    queue.push(index_first);
    while queue.len() >0{
        let index = queue.pop().unwrap();
        if index.x>0 {
            board[index.x-1][index.y] += 1;
            if board[index.x-1][index.y] == 10 && flashed[index.x-1][index.y] == false{
                queue.push(Index{x:index.x-1, y:index.y});
                flashed[index.x-1][index.y] = true;
            }
        }
        if index.x <board.len()-1 {
            board[index.x+1][index.y] += 1;
            if board[index.x+1][index.y] == 10 && flashed[index.x+1][index.y] == false{
                queue.push(Index{x:index.x+1, y:index.y});
                flashed[index.x+1][index.y] = true;
            }
        }
        if index.y>0 {
            board[index.x][index.y-1] += 1;
            if board[index.x][index.y-1] == 10 && flashed[index.x][index.y-1] == false{
                queue.push(Index{x:index.x, y:index.y-1});
                flashed[index.x][index.y-1] = true;
            }
        }
        if index.y+1 <board[0].len() {
            board[index.x][index.y+1] += 1;
            if board[index.x][index.y+1] == 10 && flashed[index.x][index.y+1] == false{
                queue.push(Index{x:index.x, y:index.y+1});
                flashed[index.x][index.y+1] = true;
            }
        }
        if index.x <board.len()-1 && index.y>0 {
            board[index.x+1][index.y-1] += 1;
            if board[index.x+1][index.y-1] == 10 && flashed[index.x+1][index.y-1] == false{
                queue.push(Index{x:index.x+1, y:index.y-1});
                flashed[index.x+1][index.y-1] = true;
            }
        }
        if index.x <board.len()-1 && index.y+1 <board[0].len(){
            board[index.x+1][index.y+1] += 1;
            if board[index.x+1][index.y+1] == 10 && flashed[index.x+1][index.y+1] == false{
                queue.push(Index{x:index.x+1, y:index.y+1});
                flashed[index.x+1][index.y+1] = true;
            }
        }
        if index.x>0 && index.y+1 <board[0].len(){
            board[index.x-1][index.y+1] += 1;
            if board[index.x-1][index.y+1] == 10 && flashed[index.x-1][index.y+1] == false{
                queue.push(Index{x:index.x-1, y:index.y+1});
                flashed[index.x-1][index.y+1] = true;
            }
        }
        if index.x>0 && index.y>0 {
            board[index.x-1][index.y-1] += 1;
            if board[index.x-1][index.y-1] == 10 && flashed[index.x-1][index.y-1] == false{
                queue.push(Index{x:index.x-1, y:index.y-1});
                flashed[index.x-1][index.y-1] = true;
            }
        }
    }

}

fn perform_step (board:&mut Vec<Vec<u32>>, flashed:&mut Vec<Vec<bool>>)->i32{
    for i in 0..board.len(){
        for j in 0..board[0].len(){
            board[i][j] += 1;
            if board[i][j]==10{
                increase_neighbours(board, flashed, Index{x:i, y:j});
            }
        }
    }
    let counter = reset_board(board, flashed);
    counter
    
}


fn main() {
    let input = read("/home/elisabeth/AdventOfCode2021/day11/src/input.txt");
    if let Ok(inputs) = input {
        let mut board = inputs.clone();
        //mark which octopus has flashed
        let mut flashed: Vec<Vec<bool>> = Vec::new();
        for i in 0..inputs.len(){
            flashed.push(Vec::new());
            for _ in 0..inputs[0].len(){
                flashed[i].push(false);
            }
        }
        let mut simultatious = false;
        let mut counter =0;
        while !simultatious{
            counter += 1;
            if perform_step(&mut board, &mut flashed) == (board.len()*board[0].len()) as i32{
                simultatious=true;
            }
        }
        println!("{}", counter);
    }
    else {
        println!("Something is wrong");
    }
    
}

