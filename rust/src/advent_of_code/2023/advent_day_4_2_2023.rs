use std::io;

fn main(){
    let mut total;
    let mut list: Vec<(i64, Vec<String>, Vec<String>)> = Vec::new();

    loop{
        let input = read_input();
        let input = input.split(":").map(|x| x.to_string()).collect::<Vec<String>>()[1].clone();
        let input = input.split("|").map(|x| x.to_string()).collect::<Vec<String>>();
        let winning_numbers = input[0].split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
        let have_numbers = input[1].split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
        list.push((1,winning_numbers,have_numbers));

        total = clac_cards(list.clone());
        println!("total = {}", total);
    }
}


fn clac_cards(mut list: Vec<(i64, Vec<String>, Vec<String>)>) -> i64 {
    let mut total = 0;

    for i in 0..list.len() {
        let mut t = 0;
        for j in 0..list[i].1.len() {
            if list[i].2.contains(&list[i].1[j]) && list[i].1[j].len() > 0 {
                t += 1;
            }
        }
        for j in 1..t+1 {
            if i+j < list.len() {
                list[i+j].0 += list[i].0;
            }
        }
        total += list[i].0;
    }

    total
}


fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}