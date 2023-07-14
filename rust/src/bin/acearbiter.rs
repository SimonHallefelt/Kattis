use std::io;

fn main(){
    let n = read_line().parse::<i128>().unwrap();

    let mut p1 = 0;
    let mut p2 = 0;
    let mut last_score = vec![0, 0];

    for i in 1..(n+1) {
        let input = read_line();
        let score = input.split("-").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<i128>>();

        if score.iter().sum::<i128>() > p1+p2 {
            if p1 == 11 || p2 == 11 {
                println!("error {}", i);
                break;
            }else if score[0] > 11 || score[1] > 11 {
                println!("error {}", i);
                break;
            }else if score[0] == 11 && score[1] == 11{
                println!("error {}", i);
                break;
            }

            let turns_past = score.iter().sum::<i128>();
            if turns_past % 4 == 0 || turns_past % 4 == 3 {
                if score[0] < p1 || score[1] < p2 {
                    println!("error {}", i);
                    break;
                }
                p1 = score[0];
                p2 = score[1];
            }else {
                if score[0] < p2 || score[1] < p1 {
                    println!("error {}", i);
                    break;
                }
                p1 = score[1];
                p2 = score[0];
            }
            last_score = score;

        }else if score != last_score{
            println!("error {}", i);
            break;
        }

        if i == n {
            println!("ok");
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}