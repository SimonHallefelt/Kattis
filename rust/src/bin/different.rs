use std::io;

fn main() {
    loop{
        let mut i = String::new();
        io::stdin().read_line(&mut i).expect("");
        if i.trim().is_empty(){
            break;
        }
        let i: Vec<i128> = i.trim().split(" ").map(|x| x.parse::<i128>().unwrap()).collect();
        println!("{}", (i[0]-i[1]).abs());
    }
}