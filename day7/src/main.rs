use std::fs::File;
use std::io::{self, BufReader, BufRead};

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
    let input = read("/home/elisabeth/AdventOfCode2021/day7/src/input.txt");
    if let Ok(inputs) = input {
        let mut num_buckets:usize = 0;
        for &i in &inputs {
            if num_buckets < i as usize{
                num_buckets = i as usize;
            }
        }
        num_buckets += 1;
        let mut individuals =vec![0;num_buckets as usize];
        let mut sum:i128 = 0;
        let mut sum_squared:i128 = 0;
        for &i in &inputs{
            individuals[i as usize] += 1;
            sum += i as i128;
            sum_squared += i as i128*i as i128;
        }
        let mut prefix_sum : Vec<i128> =vec![0;num_buckets as usize];
        let mut sum_smaller : Vec<i128> =vec![0;num_buckets as usize];
        prefix_sum[0]=individuals[0];
        for k in 1..num_buckets {
            prefix_sum[k] = prefix_sum[k-1] + individuals[k];
            sum_smaller[k] = sum_smaller[k-1] + individuals[k-1]*(k as i128 - 1);
        }
        let mut min_cost:i128 = (sum + sum_squared)/2;
        for k in 1..num_buckets{
            let cost1:i128 = sum - sum_smaller[k];
            let cost2:i128 = k as i128 * (inputs.len() as i128 - prefix_sum[k - 1]);
            let cost3:i128 = sum_smaller[k];
            let cost4:i128 = k as i128 * prefix_sum[k - 1];
            let cost_old:i128 = cost1 - cost2 - cost3 + cost4;

            let a:i128 = (k*k*inputs.len()) as i128;
            let b:i128 = k as i128 * sum;
            
            let cost: i128 = (a as i128 + sum_squared + cost_old)/2 - b;

            //println!("{}", cost1 - cost2);
            //println!("{}", -cost3 + cost4);
            if cost < min_cost {
                min_cost = cost;
            }
        }

        println!("{}", min_cost);
    }
    else {
        println!("Something is wrong");
    }
    
}

