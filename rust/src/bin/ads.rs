use std::{io, collections::HashSet};

fn main(){
    let input = read_line();
    let input: Vec<i128> = input.trim().split(" ").map(|x| x.parse::<i128>().unwrap()).collect();
    let h = input[0];
    let w = input[1];

    let mut web_page = Vec::new();
    let mut temp = String::new();
    for _ in 0..(w+1) {
        temp.push('A');
    }
    web_page.push(temp);
    for _ in 0..h {
        let mut input = read_line();
        if input.len() >= w as usize {
            input = input[0..w as usize].to_string();
        }
        let input = "A".to_string() + &input;
        web_page.push(input);
    }

    let not_ad1 = 
    ["a", "b", "c", "d", "e", 
    "f", "g", "h", "i", "j", 
    "k", "l", "m", "n", "o", 
    "p", "q", "r", "s", "t", 
    "u", "v", "w", "x", "y", 
    "z"].map(|x| x.to_uppercase());
    let not_ad2 = 
    ["a", "b", "c", "d", "e", 
    "f", "g", "h", "i", "j", 
    "k", "l", "m", "n", "o", 
    "p", "q", "r", "s", "t", 
    "u", "v", "w", "x", "y", 
    "z", 
    "?", "!", ",", ".", " ", "+"];
    let mut not_ad: HashSet<String> = not_ad1.iter().map(|x| x.to_string()).collect();
    for s in not_ad2 {
        not_ad.insert(s.to_string());
    }

    let pictures: Vec<(i128, i128, i128, i128, i128)> = find_pictures(web_page.clone());

    remove_pictures(&mut web_page, pictures, not_ad);

    for i in 1..web_page.len() {
        for j in 1..web_page[i].len() {
            print!("{}", web_page[i].chars().nth(j).unwrap());
        }
        println!("");
    }
}

fn remove_pictures(mut web_page: &mut Vec<String>, pictures: Vec<(i128, i128, i128, i128, i128)>, not_ad: HashSet<String>){
    for i in 1..web_page.len() {
        for j in 1..web_page[i].len() {
            if !not_ad.contains(&web_page[i][j..j+1].to_string()) {
                for p in &pictures {
                    if (i as i128) > p.0 && (i as i128) < p.2 && (j as i128) > p.1 && (j as i128) < p.3 {
                        remove_pricture(&mut web_page, p);
                        break;
                    }
                }
            }
        }
    }
}

fn remove_pricture(web_page: &mut Vec<String>, p: &(i128, i128, i128, i128, i128)) {
    for i in p.0..=p.2 {
        for j in p.1..=p.3 {
            web_page[i as usize].replace_range(j as usize..j as usize+1, " ");
        }
    }
}

fn find_pictures(web_page: Vec<String>) -> Vec<(i128, i128, i128, i128, i128)> {
    let mut pictures = Vec::new();
    let mut start: (i128, i128);
    let mut end = (0, 0);

    for i in 1..web_page.len() {
        for j in 1..web_page[i].len() {
            if web_page[i][j..j+1] == *"+" && web_page[i-1][j..j+1] != *"+" && web_page[i][j-1..j] != *"+"{
                start = (i as i128, j as i128);
                for k in (i+1)..web_page.len() {
                    if web_page[k][j..j+1] == *"+" {
                        end.0 = k as i128;
                    }else {
                        break;
                    }
                }
                for k in (j+1)..web_page[i].len() {
                    if web_page[i][k..k+1] == *"+" {
                        end.1 = k as i128;
                    }else {
                        break;
                    }
                }
                pictures.push((start.0, start.1, end.0, end.1, (end.0-start.0)*(end.1-start.1)));
            }
        }
    }
    pictures.sort_by(|a, b| a.4.cmp(&b.4));
    return pictures;
}

fn read_line() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.to_string();
}