use std::io;

fn main() {
    let mut value = 1;
    let mut cycle = 0;
    let mut scren: Vec<Vec<String>> = Vec::new();
    let mut row = 0;

    for _ in 0..6 {
        scren.push(Vec::new());
    }

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();
        let input: Vec<&str> = input.split(" ").collect();

        let mut add = 0;
        let mut cycels_to_run = 1;
        if input[0] == "addx" {
            cycels_to_run = 2;
            add = input[1].to_string().parse::<i32>().unwrap();
        }

        for _ in 0..cycels_to_run {

            if cycle >= value-1 && cycle <= value+1 {
                scren[row].push("#".to_string());
            }else{
                scren[row].push(".".to_string());
            }
            cycle += 1;

            if cycle == 40 {
                cycle = 0;
                row += 1;
            }
        }
        value += add;

        if 6 == row{
            for i in 0..6 {
                println!("{}", scren[i].join(""))
            }
        }
    }
}
