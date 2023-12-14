use std::io;

fn main(){
    let mut total: i32 = 0;
    loop{
        let input = read_line();
        let mut min:i32 = -1;
        let mut max: i32 = -1; 
        for c in input.chars() {
            if c.is_digit(10) {
                if min < 0 {
                    min = c.to_digit(10).unwrap() as i32;
                }
                max = c.to_digit(10).unwrap() as i32;
            }
        }
        total += min*10+max;
        println!("{}", total);
    }

}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}