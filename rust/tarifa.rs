use std::io;

fn main(){
    let mut x = String::new();
    let mut n = String::new();
    //let mut temp1 = String::new();
    //let mut temp2: i32;
    let max: i32;
    let mut used = 0;

    io::stdin().read_line(&mut x).expect("Failed to read line");
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let x: i32 = x.trim().parse().expect("Please type a number!");
    let n: i32 = n.trim().parse().expect("Please type a number!");

    max = x * (n+1);

    for _i in 0..n {
        let mut temp1 = String::new();
        io::stdin().read_line(&mut temp1).expect("bla");
        let temp2: i32 = temp1.trim().parse().expect("bla2");
        used += temp2;
    }
    
    println!("{}", max - used); 
}