use std::io;

fn main(){
    let input = read_line();
    let input: Vec<&str> = input.trim().split(" ").collect();
    let q = input[1].to_string().parse::<i128>().unwrap();

    let input = read_line();
    let mut v: Vec<i128> = input.trim().split(" ").map(|x| x.parse::<i128>().unwrap()).collect();
    v.sort();

    let mut queries: Vec<Vec<i128>> = Vec::new();
    for _ in 0..q {
        let input = read_line();
        queries.push(input.trim().split(" ").map(|x| x.parse::<i128>().unwrap()).collect());
    }

    run(v, queries);
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}



fn run(mut v: Vec<i128>, queries: Vec<Vec<i128>>){
    let mut index;
    for q in queries {
        if v.len() > 0 {
            if q[0] == 1 {
                index = binary_search_larger(&v, q[1]);
            } else {
                index = binary_search_smaller(&v, q[1]);
            }

            if index >= v.len() {
                println!("-1");
            } else if q[0] == 1 && v[index] > q[1] {
                println!("{}", v[index]);
                v.remove(index);
            } else if q[0] == 2 && v[index] <= q[1] {
                println!("{}", v[index]);
                v.remove(index);
            } else {
                println!("-1");
            }
        }else {
            println!("-1");
        }
    }
}

fn binary_search_larger(v: &Vec<i128>, target: i128) -> usize{
    let mut left: i64 = 0;
    let mut right: i64 = v.len() as i64 - 1;
    let mut mid: i64;
    while left <= right {
        mid = (left + right) / 2;
        if v[mid as usize] <= target {
            left = mid + 1;
        } else {
            if mid - 1 < 0 {
                return 0;
            }
            right = mid - 1;
        }
    }
    for i in left as usize..v.len() {
        if v[i] > target {
            return i;
        }
    }
    return left as usize;
}

fn binary_search_smaller(v: &Vec<i128>, target: i128) -> usize{
    let mut left: i64 = 0;
    let mut right: i64 = v.len() as i64 - 1;
    let mut mid: i64;
    while left <= right {
        mid = (left + right) / 2;
        if v[mid as usize] < target {
            left = mid + 1;
        } else {
            if mid - 1 < 0 {
                return 0;
            }
            right = mid - 1;
        }
    }
    for i in (0..(left+1) as usize).rev() {
        if i < v.len() {
            if v[i] <= target {
                return i;
            }
        }
    }
    return left as usize;
}