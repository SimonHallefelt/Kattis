use std::io;

fn main(){
    let n = read();
    for _ in 0..n {
        run()
    }

}


// ax^2 + bx + c
// (dx + e) * (fx + g)
// a = d*f
// b = d*g + e*f
// c = e*g

fn run() {
    let values = read_vec();

    

}


fn read() -> i64{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    input.parse::<i64>().unwrap()
}


fn read_vec() -> Vec<i64>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    input.split(" ").map(|x| x.parse::<i64>().unwrap()).collect()
}