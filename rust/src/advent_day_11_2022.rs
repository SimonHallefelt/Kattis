use std::io;

fn main() {
    let mut items: Vec<Vec<i64>> = Vec::new();
    let mut operation: Vec<Vec<String>> = Vec::new();
    let mut test: Vec<Vec<i64>> = Vec::new();
    let mut inspected_items: Vec<i64> = Vec::new();
    let monkys = 8;
    let rounds = 20;

    for _ in 0..monkys {
        inspected_items.push(0);
    }

    for i in 0..monkys {
        let mut input0 = String::new();
        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();
        let mut input4 = String::new();
        let mut input5 = String::new();
        io::stdin().read_line(&mut input0).expect("Failed to read line");
        io::stdin().read_line(&mut input1).expect("Failed to read line");
        io::stdin().read_line(&mut input2).expect("Failed to read line");
        io::stdin().read_line(&mut input3).expect("Failed to read line");
        io::stdin().read_line(&mut input4).expect("Failed to read line");
        io::stdin().read_line(&mut input5).expect("Failed to read line");
        input1 = input1.replace(",", "").trim().to_string();
        input2 = input2.trim().to_string();
        input3 = input3.trim().to_string();
        input4 = input4.trim().to_string();
        input5 = input5.trim().to_string();
        let input1: Vec<&str> = input1.split(" ").collect();
        let input2: Vec<&str> = input2.split(" ").collect();
        let input3: Vec<&str> = input3.split(" ").collect();
        let input4: Vec<&str> = input4.split(" ").collect();
        let input5: Vec<&str> = input5.split(" ").collect();

        let mut items_temp = Vec::new();
        for j in 2..input1.len() {
            items_temp.push(input1[j].to_string().parse::<i64>().unwrap());
        }
        items.push(items_temp);

        let operation_temp = vec![input2[4].to_string(), input2[5].to_string()];
        operation.push(operation_temp);

        let test_temp = vec![input3[3].to_string().parse::<i64>().unwrap(), input4[5].to_string().parse::<i64>().unwrap(), input5[5].to_string().parse::<i64>().unwrap()];
        test.push(test_temp);

        if i < monkys-1{
            io::stdin().read_line(&mut input0).expect("Failed to read line");
        }
    }
    
    for r in 0..rounds {
        for i in 0..monkys {
            inspected_items[i] += items[i].len() as i64;
            for _ in 0..items[i].len() {
                let mut item: i64 = items[i].remove(0);
                if operation[i][0] == "+" {
                    if operation[i][1] == "old" {
                        item += item;
                    }else{
                        item += operation[i][1].to_string().parse::<i64>().unwrap();
                    }
                }else{
                    if operation[i][1] == "old" {
                        item *= item;
                    }else{
                        item *= operation[i][1].to_string().parse::<i64>().unwrap();
                    }
                }
                item /= 3;

                if 0 == item % test[i][0] {
                    items[test[i][1] as usize].push(item);
                }else{
                    items[test[i][2] as usize].push(item);
                }
            }
        }
    }

    inspected_items.sort();
    println!("monkey business = {:?}", inspected_items[monkys-1] * inspected_items[monkys-2]);
}
