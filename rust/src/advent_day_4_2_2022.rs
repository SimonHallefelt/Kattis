use std::io;


fn main(){
    let mut total = 0;

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        let span1: Vec<&str> = input.split(",").collect();
        let mut span2: Vec<&str> = span1[0].split("-").collect();
        let mut span3: Vec<&str> = span1[1].split("-").collect();

        span2.append(&mut span3);
        let mut span4: Vec<i32> = Vec::new();
        for i in span2{
            span4.push(i.parse::<i32>().unwrap());
        }

        if span4[0] == span4[2] || span4[1] == span4[3] || span4[0] == span4[3] || span4[1] == span4[2]{
            total += 1;
        }else if span4[0] < span4[2] && span4[1] > span4[3]{
            total += 1;
        }else if span4[0] > span4[2] && span4[1] < span4[3]{
            total += 1;
        }else if span4[0] > span4[2] && span4[0] < span4[3]{
            total += 1;
        }else if span4[1] > span4[2] && span4[1] < span4[3]{
            total += 1;
        }
        println!("{}", total);
    }
}

