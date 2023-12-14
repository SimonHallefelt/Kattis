use std::io;

fn main(){
    let mut total: i32 = 0;

    loop{
        let input = read_input();
        let input: Vec<_> = input.split(":").collect();
        let game: i32 = input[0].split(" ").collect::<Vec<_>>()[1].parse::<i32>().unwrap();
        let input: Vec<_> = input[1].split(";").collect();
        let mut b = true;
        for g in input {
            let p: Vec<_> = g.split(",").map(|f| f.trim()).collect();
            for i in 0..p.len() {
                let a: Vec<_> = p[i].split(" ").collect();
                if a[1] == "red" && a[0].parse::<i32>().unwrap() > 12 {
                    b = false
                }
                else if a[1] == "green" && a[0].parse::<i32>().unwrap() > 13 {
                    b = false
                }
                else if a[1] == "blue" && a[0].parse::<i32>().unwrap() > 14 {
                    b = false
                }
            }
        }
        if b {
            total += game;
        }

        println!("total = {}", total);
    }
}


fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}