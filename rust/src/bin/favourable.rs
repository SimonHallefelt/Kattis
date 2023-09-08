use std::{io, collections::HashMap};

fn main() {
    let t = read_line().parse::<i128>().unwrap();
    for _ in 0..t {
        case();
    }
}

fn case() {
    let n = read_line().parse::<i128>().unwrap();
    let mut pages: HashMap<i128, Vec<i128>> = HashMap::new();
    let mut good_endings: HashMap<i128, i128> = HashMap::new();
    for _ in 0..n {
        let input = read_line();
        let input: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();
        let mut page = Vec::new();
        if input.len() == 4{
            for i in input {
                page.push(i.parse::<i128>().unwrap());
            }
            pages.insert(page[0], page);
        }else if input[1] == "favourably" {
            page.push(input[0].parse::<i128>().unwrap());
            page.push(1);
            pages.insert(page[0], page);
        }else {
            page.push(input[0].parse::<i128>().unwrap());
            page.push(0);
            pages.insert(page[0], page);
        }
    }

    println!("{}", favourable(&pages, &mut good_endings, pages.get(&1).unwrap().clone()));
}

fn favourable(pages: &HashMap<i128, Vec<i128>>, good_endings: &mut HashMap<i128, i128>, page: Vec<i128>) -> i128 {
    if good_endings.contains_key(&page[0]) {
        return good_endings.get(&page[0]).unwrap().clone();
    }

    let mut sum = 0;
    if page.len() == 2 {
        sum += page[1];
    }else {
        sum += favourable(&pages, good_endings, pages.get(&page[1]).unwrap().clone());
        sum += favourable(&pages, good_endings, pages.get(&page[2]).unwrap().clone());
        sum += favourable(&pages, good_endings, pages.get(&page[3]).unwrap().clone());
    }

    good_endings.insert(page[0], sum);
    return sum;
}

fn read_line() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.trim().to_string();
}