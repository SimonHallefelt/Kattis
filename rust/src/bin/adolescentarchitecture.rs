use std::io;

fn main() {
    let n = read_line().parse::<i128>().unwrap();
    let mut blocks: Vec<(i128, i128)> = Vec::new();
    for _ in 0..n {
        let input = read_line();
        let input = input.split(" ").collect::<Vec<&str>>();
        let size = input[1].parse::<i128>().unwrap();
        if input[0] == "cube" {
            blocks.push((size, (size.pow(2))*2));
        } else {
            blocks.push((size*2, (size*2).pow(2)));
        }
    }
    blocks.sort_by(|a, b| a.0.cmp(&b.0));
    sort_blocks(blocks);
}

fn sort_blocks(mut blocks: Vec<(i128, i128)>) {
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..blocks.len()-1 {
            if blocks[i].1 > blocks[i+1].1 {
                if blocks[i].0 < blocks[i+1].0 {
                    println!("impossible");
                    return;
                }else {
                    let temp = blocks[i];
                    blocks[i] = blocks[i+1];
                    blocks[i+1] = temp;
                    changed = true;
                }
            }
        }
    }

    for block in blocks {
        if block.0.pow(2) != block.1 {
            println!("cube {}", block.0);
        }else {
            println!("cylinder {}", block.0/2);
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}