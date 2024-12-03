fn main() {
    lines = read_lines();
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string("inptu.txt").unwrap().lines() {
        result.push(line.to_string())
    }

    result
}