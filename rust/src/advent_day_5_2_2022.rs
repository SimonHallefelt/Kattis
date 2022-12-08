use std::io;

fn main(){
    /* let test_input1: Vec<&str> = vec!["Z", "N"];
    let test_input2: Vec<&str> = vec!["M", "C", "D"];
    let test_input3: Vec<&str> = vec!["P"];
    let mut cargo = vec![test_input1, test_input2, test_input3]; */

    let input1: Vec<&str> = vec!["B", "L", "D", "T", "W", "C", "F", "M"];
    let input2: Vec<&str> = vec!["N", "B", "L"];
    let input3: Vec<&str> = vec!["J", "C", "H", "T", "L", "V"];
    let input4: Vec<&str> = vec!["S", "P", "J", "W"];
    let input5: Vec<&str> = vec!["Z", "S", "C", "F", "T", "L", "R"];
    let input6: Vec<&str> = vec!["W", "D", "G", "B", "H", "N", "Z"];
    let input7: Vec<&str> = vec!["F", "M", "S", "P", "V", "G", "C", "N"];
    let input8: Vec<&str> = vec!["W", "Q", "R", "J", "F", "V", "C", "Z"];
    let input9: Vec<&str> = vec!["R", "P", "M", "L", "H"]; 
    let mut cargo = vec![input1, input2, input3, input4, input5, input6, input7, input8, input9];
    
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        let input: Vec<&str> = input.split(" ").collect();
        let box_to_move = input[1].parse::<i32>().unwrap();
        let cargo_from = input[3].parse::<i32>().unwrap() -1;
        let cargo_to = input[5].parse::<i32>().unwrap() -1;

        let mut temp: Vec<&str> = Vec::new();

        for i in 0..box_to_move{
            let j = i as usize;
            let btm = box_to_move as usize;
            let cf = cargo_from as usize;
            temp.push(cargo[cf][cargo[cf].len() - (btm - j)]);
        }

        for _ in 0..box_to_move{
            let cf = cargo_from as usize;
            cargo[cf].pop();
        }

        cargo[cargo_to as usize].append(&mut temp);

        for i in 0..cargo.len(){
            let j = i as usize;
            if cargo[j].len() != 0{
                println!("{:?}", cargo[j][cargo[j].len() - 1]);
            }else{
                println!("Empty");
            }
        }
    }
}

