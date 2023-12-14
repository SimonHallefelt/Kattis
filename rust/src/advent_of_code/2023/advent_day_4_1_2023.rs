use std::io;

fn main(){
    let mut total = 0;

    loop{
        let input = read_input();
        let input = input.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>();
        let winning_numbers = input[0].split(" ").collect::<Vec<&str>>();
        let have_numbers = input[1].split(" ").collect::<Vec<&str>>();

        let mut t = 1;
        for i in 0..winning_numbers.len() {
            if have_numbers.contains(&winning_numbers[i]) && winning_numbers[i].len() > 0 {
                t *= 2;
            }
        }
        total += t/2;

        println!("total = {}", total);
    }
}



fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}