use std::fs::read_to_string;
use std::collections::HashMap;

pub fn main() {
    let mut sum = 0;
    let mut index = 0;

    let (mut right, mut left) = read_lines("./src/day1/part2/input.txt");

    right.sort();
    left.sort();
    let hist_right = histogram(&right); 

    while index < left.len() {
        if hist_right.contains_key(&left[index]){
            println!("index: {}",index);
            let left_val = left[index];
            let count  = hist_right.get(&left_val).unwrap();
            sum += count * left_val;
            //sum += hist_right.get(&left[index]).unwrap() * left[index];
        }
        
        index +=1;
    }
    print!("sum: {}\n", sum);
}

fn histogram(data: &Vec<u32>) -> HashMap<u32,u32>{
    let mut count: u32;
    let mut ret: HashMap<u32, u32> = HashMap::new();

    for element in data {
        count = 0;
        for count_element in data {
            if element == count_element {
                count += 1;
            }
        }
        ret.insert(*element, count);
    }
    return ret;
}

fn read_lines(filename: &str) -> (Vec<u32>, Vec<u32>) {
    let mut right: Vec<u32> = Vec::new();
    let mut left: Vec<u32> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let mut line_split = line.split("   ");
        let first = line_split.next().unwrap();
        let second = line_split.next().unwrap();

        right.push(first.to_string().parse().unwrap());
        left.push(second.to_string().parse().unwrap());
    }

    return (right, left)
}