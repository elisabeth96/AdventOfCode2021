use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::cmp;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Segment {
    tail: Point,
    tip: Point
}

type Grid = Vec<Vec<i32>>;

impl Segment {
    fn is_horizontal(&self) -> bool {
        self.tail.y == self.tip.y
    }
    fn is_vertical(&self) -> bool {
        self.tail.x == self.tip.x
    }

    fn mark(&self, grid: &mut Grid) {
        if self.is_horizontal() {
            let min_x = cmp::min(self.tail.x, self.tip.x);
            let max_x = cmp::max(self.tail.x, self.tip.x) + 1;
            let y = self.tail.y;
            for x in min_x..max_x  {
                grid[y as usize][x as usize] += 1;
            }
        }
        else if self.is_vertical() {
            let min_y = cmp::min(self.tail.y, self.tip.y);
            let max_y = cmp::max(self.tail.y, self.tip.y) + 1;
            let x = self.tail.x;
            for y in min_y..max_y  {
                grid[y as usize][x as usize] += 1;
            }
        }
        else {
            let size = (self.tip.x - self.tail.x).abs() + 1;
            let inc_x = if self.tip.x - self.tail.x > 0 { 1 } else { -1 };
            let inc_y = if self.tip.y - self.tail.y > 0 { 1 } else { -1 };
            let mut x = self.tail.x;
            let mut y = self.tail.y;
            for _ in 0..size {
                grid[y as usize][x as usize] += 1;
                x += inc_x;
                y += inc_y;
            }
        }
    }
}

fn read(path: &str) -> Result<Vec<Segment>, io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);

    let mut v = Vec::new();
    for line in br.lines() {
        let line = line?;
        let mut points = [Point{x : 0, y : 0}; 2];
        for (i,s) in line.split("->").enumerate() {
            let mut it = s.split(',');
            let a: i32 = it.next().unwrap().trim().parse().unwrap();
            let b: i32 = it.next().unwrap().trim().parse().unwrap();
            points[i] = Point{x: a, y : b};
        }
        v.push(Segment{tail: points[0], tip: points[1]});
    }

    Ok(v)
}

fn count_overlaps(grid: &Grid) -> i32{
    let mut sum = 0;
    for row in grid {
        for &value in row {
            if value >= 2 {
                sum += 1;
            } 
        }
    }
    sum
}

fn main() {
    let input = read("/home/elisabeth/AdventOfCode2021/day5/src/input.txt");
    if let Ok(segments) = input {
        let mut grid = vec![vec![0;1000]; 1000];
        for segment in &segments {
            segment.mark(&mut grid);
        }

        println!("result {}", count_overlaps(&grid));
    }
    else {
        println!("Something is wrong");
    }
    
}

