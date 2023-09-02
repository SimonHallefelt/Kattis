use std::io;

fn main() {
    let n = readline().parse::<i128>().unwrap();
    let mut player_com: i128 = 0;
    let mut player_num = 0;
    let mut teams: Vec<(i128, i128)> = Vec::new();
    for _ in 0..n {
        let input = readline();
        let input: Vec<i128> = input.trim().split(" ").map(|x| x.parse::<i128>().unwrap()).collect();
        teams.push((input[0], input[1]));
        player_com += player_num * input[0];
        player_num += input[0];
    }

    run(teams, player_com, player_num, n);
}

fn run(teams: Vec<(i128, i128)>, player_com: i128, player_num: i128, n: i128) -> i128{
    let mut sum = 0;

    for i in 0..n {
        let team1 = teams[i as usize];
        sum += (player_com - (player_num - team1.0) * team1.0) * team1.1;
    }
    
    println!("{}", sum);
    sum
}

fn readline() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

//tests:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        //let input = ["3", "2 3", "0 1", "1 2"];
        let n = 3;
        let teams: Vec<(i128, i128)> = vec![(2, 3), (0, 1), (1, 2)];
        let player_com: i128 = 2*0+0*2+1*2;
        let player_num: i128 = 2+0+1;

        assert_eq!(run(teams, player_com, player_num, n), 2);
    }

    #[test]
    fn test2() {
        //let input = ["3", "2 1", "2 1", "2 1"];
        let n = 3;
        let teams: Vec<(i128, i128)> = vec![(2, 1), (2, 1), (2, 1)];
        let player_com: i128 = 2*0+2*2+2*4;
        let player_num: i128 = 2+2+2;

        assert_eq!(run(teams, player_com, player_num, n), 12);
    }

    #[test]
    fn test3() {
        //let input = ["3", "5 0", "5 0", "0 5"];
        let n = 3;
        let teams: Vec<(i128, i128)> = vec![(5, 0), (5, 0), (0, 5)];
        let player_com: i128 = 5*0+5*5+0*10;
        let player_num: i128 = 5+5;

        assert_eq!(run(teams, player_com, player_num, n), 125);
    }

    #[test]
    fn test4() {
        //let input = ["4", "5 0", "5 0", "0 5", "0 5"];
        let n = 4;
        let teams: Vec<(i128, i128)> = vec![(5, 0), (5, 0), (0, 5), (0, 5)];
        let player_com: i128 = 5*0+5*5+0*10+0*15;
        let player_num: i128 = 5+5;

        assert_eq!(run(teams, player_com, player_num, n), 250);
    }

    #[test]
    fn test5() {
        //let input = ["3", "5 5", "5 0", "0 0"];
        let n = 3;
        let teams: Vec<(i128, i128)> = vec![(5, 5), (5, 0), (0, 0)];
        let player_com: i128 = 5*0+5*5+0*10;
        let player_num: i128 = 5+5;

        assert_eq!(run(teams, player_com, player_num, n), 0);
    }

    #[test]
    fn test6() {
        //let input = ["3", "1 0", "1 0", "0 1"];
        let n = 3;
        let teams: Vec<(i128, i128)> = vec![(1, 0), (1, 0), (0, 1)];
        let player_com: i128 = 1*0+1*1+0*2;
        let player_num: i128 = 1+1;

        assert_eq!(run(teams, player_com, player_num, n), 1);
    }

    #[test]
    fn test7() {
        //let input = ["5", "1 0", "1 0", "1 0", "0 1", "0 1"];
        let n = 5;
        let teams: Vec<(i128, i128)> = vec![(1, 0), (1, 0), (1, 0), (0, 1), (0, 1)];
        let player_com: i128 = 1*0+1*1+1*2+0*3+0*3;
        let player_num: i128 = 1+1+1;

        assert_eq!(run(teams, player_com, player_num, n), 6);
    }

}
