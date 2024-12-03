use std::fs::read_to_string;

pub fn main() {
    let (mut right, mut left) = read_lines("./src/1/input.txt");

    right.sort();
    left.sort();

    let mut sum = 0;
    let mut index =0;
    while index < 1000 {
        if right[index] > left[index]{
            sum += right[index] - left[index];
        } 
        
        if right[index] < left[index]{
            sum += left[index] - right[index];
        } 
        index +=1;
    }
    print!("sum: {}\n", sum);
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