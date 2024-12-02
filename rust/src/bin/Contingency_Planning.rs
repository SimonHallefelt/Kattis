use std::io;

fn main(){
    let n = read();
    
    let total = run(n, 1, 0);
    if total > 1000 * 1000 * 1000 {
        println!("JUST RUN!!")

    } else {
        println!("{}", total)
    }

}

fn run(n: i128, last: i128, total: i128) -> i128 {
    if n == 0 {
        return total;
    }
    if total > 1000 * 1000 * 1000 {
        return total;
    }
    run(n-1, n*last, total+last*n)
}


fn read() -> i128 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    input.parse::<i128>().unwrap()
}