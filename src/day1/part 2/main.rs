use std::fs::read_to_string;
use std::collections::HashMap;

pub fn main() {
    let mut sum = 0;
    let mut index = 0;

    let (mut right, mut left) = read_lines("./src/1/input.txt");

    right.sort();
    left.sort();
    let hist_right = histogram(&right); 

    while index < left.len() {
        sum += hist_right[&left[index]];
        index +=1;
    }
    print!("sum: {}\n", sum);
}

fn histogram(data: &Vec<u32>) -> HashMap<u32,u32>{
    let mut count = 0;
    let mut ret = HashMap::new();

    for element in data {
        for count_element in data {
            if element == count_element {
                count += 1;
            }
        }
        ret.insert(element, count);
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