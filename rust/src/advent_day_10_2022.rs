use std::io;

fn main() {
    let mut vec_signal_strength: Vec<i32> = vec![0];
    let mut value = 1;
    let mut cycle = 0;
    let mut loking_for = 20;

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();
        let input: Vec<&str> = input.split(" ").collect();

        let mut add = 0;
        if input[0] == "noop" {
            cycle += 1;
        }else{
            cycle += 2;
            add = input[1].to_string().parse::<i32>().unwrap();
        }

        if loking_for <= cycle {
            vec_signal_strength.push(value*loking_for);
            loking_for += 40;
        }
        value += add;

        let signal_strength:i32 = vec_signal_strength.iter().sum();
        println!("signal strength = {}", signal_strength)
    }
}
