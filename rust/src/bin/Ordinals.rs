use std::io;

fn main(){
    let n = read();

    let i_0 = "{}";
    let i_1 = "{{}}";
    let i_2 = "{{},{{}}}";
    let i_3 = "{{},{{}},{{},{{}}}}";
    let i_4 = "{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}}";
    let i_5 = "{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}}}";
    let i_6 = "{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}}}}";
    let i_7 = "{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}}}}}";
    let i_8 = "{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}},{{},{{}},{{},{{}}},{{},{{}},{{},{{}}}}}}}}}";

    if n == 0 {println!("{}", i_0);}
    if n == 1 {println!("{}", i_1);}
    if n == 2 {println!("{}", i_2);}
    if n == 3 {println!("{}", i_3);}
    if n == 4 {println!("{}", i_4);}
    if n == 5 {println!("{}", i_5);}
    if n == 6 {println!("{}", i_6);}
    if n == 7 {println!("{}", i_7);}
    if n == 8 {println!("{}", i_8);}
}



fn read() -> i64{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    input.parse::<i64>().unwrap()
}