use std::io;
// fel alla möjligheter tillsammans måste beräknas 
fn main(){
    let n = read_line()[0];
    let mut input = read_line();
    input.sort();
    input.reverse();

    let mut all = Vec::new();
    let mut higest = 0 as f64;
    let mut mul: f64 = 1 as f64;
    for i in 0..n as usize {
        let a = input[i];
        mul = mul * (a as f64/100 as f64);
        let ac:f64 = (mul*(i+1) as f64);
        let temp = ac.powf(ac/(i+1)as f64);
        all.push(temp);
        if temp > higest {
            higest = temp;
        }
    }
    println!("{:?}", all);
    println!("{:?}", input);

}

fn read_line() -> Vec<i64>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no input");
    input.trim().to_string().split(" ").map(|x| x.parse::<i64>().unwrap()).collect()
}