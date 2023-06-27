use std::{io, collections::HashMap};

fn main(){
    let n = read_line().parse::<i128>().unwrap();
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(read_line().parse::<i128>().unwrap());
    }
    let awnsers: HashMap<i128, String> = get_all_awnsers();

    for i in 0..n {
        if awnsers.contains_key(&v[i as usize]){
            println!("{}", awnsers.get(&v[i as usize]).unwrap());
        } else {
            println!("no solution");
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}



fn get_all_awnsers() -> HashMap<i128, String>{
    let mut awnsers: HashMap<i128, String> = HashMap::new();
    for i in 0..4 {
        let mut temp_value_1: i128 = std::i128::MAX;
        let mut temp_s_1: String = "".to_string();
        if i == 0 {
            temp_value_1 = 4*4;
            temp_s_1 = "4 * 4".to_string();
        } else if i == 1 {
            temp_value_1 = 4/4;
            temp_s_1 = "4 / 4".to_string();
        } else if i == 2 {
            temp_s_1 = "4 + 4".to_string();
            
        } else if i == 3 {
            temp_s_1 = "4 - 4".to_string();
        }

        for j in 0..4 {
            let mut temp_value_2: i128 = std::i128::MAX;
            let mut temp_s_2: String = "".to_string();
            
            if j == 0 {
                if temp_value_1 != std::i128::MAX {
                    temp_value_2 = temp_value_1 * 4;
                } else {
                    temp_value_2 = 4 * 4;
                }
                temp_s_2 = temp_s_1.clone() + " * 4";
            } else if j == 1 {
                if temp_value_1 != std::i128::MAX {
                    temp_value_2 = temp_value_1 / 4;
                } else {
                    temp_value_2 = 4 / 4;
                }
                temp_s_2 = temp_s_1.clone() + " / 4";
            } else if j == 2 {
                temp_s_2 = temp_s_1.clone() + " + 4";
            } else if j == 3 {
                temp_s_2 = temp_s_1.clone() + " - 4";
            }
            for k in 0..4 {
                let mut s = String::new();
                let mut value = 0;
                let mut temp_value_3: i128 = std::i128::MAX;

                if k == 0 {
                    if temp_value_2 != std::i128::MAX {
                        temp_value_3 = temp_value_2 * 4;
                    } else {
                        temp_value_3 = 4 * 4;
                    }
                    s = temp_s_2.clone() + " * 4";
                } else if k == 1 {
                    if temp_value_2 != std::i128::MAX {
                        temp_value_3 = temp_value_2 / 4;
                    } else {
                        temp_value_3 = 4 / 4;
                    }
                    s = temp_s_2.clone() + " / 4";
                } else if k == 2 {
                    s = temp_s_2.clone() + " + 4";
                } else if k == 3 {
                    s = temp_s_2.clone() + " - 4";
                }


                // hantera + och -
                if temp_value_1 == std::i128::MAX && 
                temp_value_2 == std::i128::MAX && 
                temp_value_3 == std::i128::MAX{
                    if i == 2 {
                        value = 4 + 4;
                    } else {
                        value = 4 - 4;
                    }

                    if j == 2 {
                        value = value + 4;
                    } else {
                        value = value - 4;
                    }

                    if k == 2 {
                        value = value + 4;
                    } else {
                        value = value - 4;
                    }
                }

                if temp_value_1 != std::i128::MAX && 
                temp_value_2 == std::i128::MAX && 
                temp_value_3 == std::i128::MAX{
                    if j == 2 {
                        value = temp_value_1 + 4;
                    } else {
                        value = temp_value_1 - 4;
                    }

                    if k == 2 {
                        value = value + 4;
                    } else {
                        value = value - 4;
                    }
                }

                if temp_value_1 == std::i128::MAX && 
                temp_value_2 != std::i128::MAX && 
                temp_value_3 == std::i128::MAX{
                    if i == 2 {
                        value = 4 + temp_value_2;
                    } else {
                        value = 4 - temp_value_2;
                    }

                    if k == 2 {
                        value = value + 4;
                    } else {
                        value = value - 4;
                    }
                }

                if temp_value_1 == std::i128::MAX && 
                temp_value_2 == std::i128::MAX && 
                temp_value_3 != std::i128::MAX{
                    if i == 2 {
                        value = 4 + 4;
                    } else {
                        value = 4 - 4;
                    }

                    if j == 2 {
                        value = value + temp_value_3;
                    } else {
                        value = value - temp_value_3;
                    }
                }

                if temp_value_1 != std::i128::MAX && 
                temp_value_2 != std::i128::MAX && 
                temp_value_3 == std::i128::MAX{
                    if k == 2 {
                        value = temp_value_2 + 4;
                    } else {
                        value = temp_value_2 - 4;
                    }
                }

                if temp_value_1 != std::i128::MAX && 
                temp_value_2 == std::i128::MAX && 
                temp_value_3 != std::i128::MAX{
                    if j == 2 {
                        value = temp_value_1 + temp_value_3;
                    } else {
                        value = temp_value_1 - temp_value_3;
                    }
                }

                if temp_value_1 == std::i128::MAX && 
                temp_value_2 != std::i128::MAX && 
                temp_value_3 != std::i128::MAX{
                    if i == 2 {
                        value = 4 + temp_value_3;
                    } else {
                        value = 4 - temp_value_3;
                    }
                }

                s = s.clone() + " = " + &value.to_string();
                awnsers.insert(value, s);
            }
        }
    }
    return awnsers;
}