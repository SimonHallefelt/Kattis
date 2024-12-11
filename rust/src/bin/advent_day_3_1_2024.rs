use std::fs::read_to_string;



fn main() {
    let file_path = "src\\advent_of_code\\2024\\data\\day_3.txt"; //159892596
    let contents = read_input(file_path);

    let mut tot = 0;
    let mut a = contents.split("mul");
    for b in a {
        let mut n1 = 0;
        let mut n2 = 0;
        let bb = b.split(")").collect::<Vec<&str>>();
        let bb = *bb.first().unwrap();
        let mut chars = bb.chars();
        let mut c = chars.next();
        if c != Some('(') {
            continue;
        }
        let temp = bb.replacen("(", "", 1);
        let bb = temp.split(",").collect::<Vec<&str>>();
        if bb.len() != 2 || bb.get(0).unwrap().len() > 3 || bb.get(1).unwrap().len() > 3 {
            continue;
        }
        for i in bb.get(0).unwrap().chars() {
            let temp = i.to_digit(10);
            if temp.is_none() {
                n1 = 0;
                break;
            }
            n1 = n1*10 + temp.unwrap();
        }
        for i in bb.get(1).unwrap().chars() {
            let temp = i.to_digit(10);
            if temp.is_none() {
                n2 = 0;
                break;
            }
            n2 = n2*10 + temp.unwrap();
        }
        tot += n1*n2;
    }

    println!("{}", tot);
}

fn read_input(file_path: &str) -> String {
    let contents = read_to_string(file_path).expect("file not found");
    contents
}