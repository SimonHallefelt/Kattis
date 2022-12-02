//A Rock, B Paper, C Scissors
//X Rock, Y Paper, Z Scissors.

//1 Rock, 2 Paper, 3 Scissors
//0 if you lost, 3 if the round was a draw, and 6 if you won

use std::io;

fn main(){
    let mut total: i32 = 0;

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();
        let s: Vec<&str>  = input.split(" ").collect();
        //println!("{:?}", input);
        //println!("{:?}", s);

        /* if s[0] == "A"{
            if s[1] == "X"{
                total += 1 + 3;
            }
            else if s[1] == "Y"{
                total += 2 + 6;
            }
            else{
                total += 3 + 0;
            }
        }
        else if s[0] == "B"{
            if s[1] == "X"{
                total += 1 + 0;
            }
            else if s[1] == "Y"{
                total += 2 + 3;
            }
            else{
                total += 3 + 6;
            }
        }
        else{
            if s[1] == "X"{
                total += 1 + 6;
            }
            else if s[1] == "Y"{
                total += 2 + 0;
            }
            else{
                total += 3 + 3;
            }
        } */

//X means you need to lose, Y means you need a draw, and Z means you need to win

        if s[1] == "X"{
            if s[0] == "A"{
                total += 0 + 3;
            }
            else if s[0] == "B"{
                total += 0 + 1;
            }
            else{
                total += 0 + 2;
            }
        }
        else if s[1] == "Y"{
            if s[0] == "A"{
                total += 3 + 1;
            }
            else if s[0] == "B"{
                total += 3 + 2;
            }
            else{
                total += 3 + 3;
            }
        }
        else{
            if s[0] == "A"{
                total += 6 + 2;
            }
            else if s[0] == "B"{
                total += 6 + 3;
            }
            else{
                total += 6 + 1;
            }
        }


        println!("{}", total);
    }

}

