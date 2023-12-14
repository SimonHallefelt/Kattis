use std::{io, collections::HashMap};

fn main(){
    let mut total: i64;
    
    let mut hands = Vec::new();

    loop {
        let input: Vec<String> = read_input().split(" ").map(|x| x.to_string()).collect();
        hands.push((input[0].clone(), 
                    input[1].parse::<i64>().unwrap(), 
                    hand_type(input[0].clone()), 
                    hand_value(input[0].clone())));

        hands.sort_by(|a,b| (a.3).cmp(&(b.3)));
        hands.sort_by(|a,b| (a.2).cmp(&(b.2)));

        total = 0;
        for i in 0..hands.len() {
            total += hands[i].1 * (i + 1) as i64;
        }
        //println!("total = {:?}", hands);
        println!("total = {}", total);
    }
}


fn hand_type(hand: String) -> i64 {
    let mut cards = HashMap::new();
    for c in hand.chars() {
        if cards.contains_key(&c) {
            cards.insert(c, cards.get(&c).unwrap()+1);
        } else {
            cards.insert(c, 1);
        }
    }
    let mut list: Vec<i64> = cards.into_values().collect();
    list.sort(); list.reverse();
    if list[0] == 5 {7}
    else if list[0] == 4 {6}
    else if list[0] == 3 && list[1] == 2 {5}
    else if list[0] == 3 {4}
    else if list[0] == 2 && list[1] == 2 {3}
    else if list[0] == 2 {2}
    else {1}
}


fn hand_value(hand: String) -> i64 {
    let card_values: HashMap<char, i64> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);

    let mut sum = 0;
    for c in hand.chars() {
        sum = sum*100+card_values.get(&c).unwrap();
    }
    sum
}


fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}