use std::io;

fn main(){
    let n: i128;
    let mut min_value: i128;
    let mut sides: i128;
    let mut needed_wins: i128;
    let mut rolls: i128;
    let mut w: i128;
    let mut input1 = String::new();

    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let input1 = input1.trim().to_string();
    n = input1.parse::<i128>().unwrap();

    for _ in 0..n {
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read line");
        let input2: Vec<&str> = input2.trim().split(" ").collect();
        min_value = input2[0].to_string().parse::<i128>().unwrap();
        sides = input2[1].to_string().parse::<i128>().unwrap();
        needed_wins = input2[2].to_string().parse::<i128>().unwrap();
        rolls = input2[3].to_string().parse::<i128>().unwrap();
        w = input2[4].to_string().parse::<i128>().unwrap();

        
        let amount_of_wins = calculate_wins(sides - min_value +1, min_value-1, needed_wins, rolls, 0, 0, 1);


        println!("{} {}", amount_of_wins, sides.pow(rolls as u32));
        if amount_of_wins * (w-1) * 2 > sides.pow(rolls as u32) && w > 1 {
            println!("yes");
        }else{
            println!("no");
        }
    }
}

fn calculate_wins(win: i128, lose: i128, needed_wins: i128, max_rolls: i128, rolls: i128, wins: i128, mul: i128) -> i128 {
    let mut amount_of_wins: i128 = 0;

    if wins >= needed_wins {
        //return mul;
        if rolls == max_rolls {
            return mul;
        }
        return mul*((win+lose)*(max_rolls - rolls));
    }
    if rolls == max_rolls {
        
        return amount_of_wins;
    }

    //won
    amount_of_wins += calculate_wins(win, lose, needed_wins, max_rolls, rolls+1, wins+1, mul*win);
    //lose
    amount_of_wins += calculate_wins(win, lose, needed_wins, max_rolls, rolls+1, wins, mul*lose);

    return amount_of_wins;
}