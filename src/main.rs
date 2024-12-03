use std::fs::read_to_string;

fn main() {
    let data = parse_data("./src/day2/part 1/input.txt");
    let mut count =0;
    for row in data {
        if is_safe(row){
            count += 1;
        }
    }
    print!("Safe rows:{}", count);
}

fn parse_data(path: &str)->Vec<Vec<i32>>{
    let mut ret: Vec<Vec<i32>> = Vec::new();
    
    for line in read_to_string(path).unwrap().lines() {
        let line_split = line.split(" ");
        let mut row:Vec<i32> = Vec::new();
        
        for value in line_split{
            row.push(value.to_string().parse().unwrap());
        }

        ret.push(row);
    }

    return ret;
}

fn is_safe(data:Vec<i32>) -> bool{
    let mut index =0;
    let  decrases = (data[0] - data[1]) < 0;
    while index < data.len(){
        if index > 0 {
            let diff = data[index - 1] -data[index];
            if diff == 0 {
                println!("{:?} : UNSAFE", data);
                return false;
            }
            if (diff > 0) && decrases {
                println!("{:?} : UNSAFE", data);
                return false;
            }
            if (diff < 0) && !decrases {
                println!("{:?} : UNSAFE", data);
                return false;
            }

            if (diff > 3)||(diff < -3){
                println!("{:?} : UNSAFE", data);
                return false
            }
        }
        index += 1;
    }
    println!("{:?} : SAFE", data);
    return  true;
}