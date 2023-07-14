use std::collections::HashMap;
use std::io::{self};
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::{Duration, Instant};


fn main() {
    let mut string_to_int = HashMap::new();
    let mut int_to_string = HashMap::new();
    for _ in 0..2000{
        
        let input = read();

        if input =="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(){
            break;
        }

        let input: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();
        
        if input[0] == "clear" {
            string_to_int.clear();
            int_to_string.clear();
        }else if input[0] == "def" {
            let name = input[1].clone();
            let value = input[2].clone().parse::<i128>().unwrap();
            if string_to_int.contains_key(&name) {
                let old_value = string_to_int.get(&name).unwrap();
                int_to_string.remove(old_value);
            }
            string_to_int.insert(name.clone(), value);
            int_to_string.insert(value, name.clone());            
        }else if input[0] == "calc" {
            let mut total = 0;
            let mut mul = 1;
            let mut error = false;
            for i in 1..input.len()-1 {
                if input[i] == "+" {
                    mul = 1;
                }else if input[i] == "-" {
                    mul = -1;
                }else if string_to_int.contains_key(&input[i]) {
                    total += mul * string_to_int.get(&input[i]).unwrap();
                }else{
                    error = true;
                    break;
                }
            }
            if !int_to_string.contains_key(&total) {
                error = true;
            }
            if !error {
                println!("{} {}", input[1..input.len()].join(" "), int_to_string.get(&total).unwrap());
            }else {
                println!("{} unknown", input[1..input.len()].join(" "));
            }
        }
    }
}

/* fn read_input() -> String {
    let (tx, _rx) = mpsc::channel();
    //let input = tx.clone();
    thread::spawn(move || {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input = input.trim().to_string();
        tx.send(input).unwrap();
        //thread::sleep(Duration::from_millis(10));
    });
    return _rx.recv().unwrap();
} */

fn read_input_with_timeout(timeout: Duration) -> Option<String> {
    let start_time = Instant::now();
    let mut input = None;

    while start_time.elapsed() < timeout {
        // Replace this block with your actual input reading logic
        if let Ok(input_value) = read_input() {
            input = Some(input_value);
            break;
        }
    }

    input
}

fn read_input() -> Result<String, ()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return Ok(input.trim().to_string());
}
    
fn read() -> String {
    let start_time = Instant::now();
    let timeout = Duration::from_millis(50);

    let (tx, rx): (Sender<Option<String>>, Receiver<Option<String>>) = channel();
    thread::spawn(move || {
        let input = read_input_with_timeout(Duration::from_millis(100)).unwrap();
        tx.send(Some(input)).expect("Failed to send data");
    });


    /* let result = rx.recv().expect("Failed to receive data");
    return result.unwrap(); */

    loop {
        let elapsed = start_time.elapsed();
        if elapsed >= timeout {
            return "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(); // Timeout occurred, return None
        }

        let remaining_timeout = timeout - elapsed;
        match rx.recv_timeout(remaining_timeout) {
            Ok(result) => return result.unwrap(), // Input received, return the result
            Err(_) => continue, // Timeout, continue waiting
        }
    }
}