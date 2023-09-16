use std::io;

fn main() {
    let mut n = read_line();
    if n > 1.0 {
        n = 1.0;
    }
    if n == 0.0 {
        println!("{}", 0.0);
        return;
    }

    println!("{}", max(n, n/2.0, n, 0))
}

fn max(top: f64, bot: f64, m: f64, mut r: i64) -> f64{
    r += 1;
    if r > 1000 {
        return bot * (m-bot);
    }
    let mut v = Vec::new();
    let dif = top-bot;
    v.push(top);
    v.push(top - dif/2.0);
    v.push(top - dif);
    v.push(bot + dif/2.0);
    v.push(bot);

    let mut v2 = Vec::new();
    v2.push((v[0]*(m-v[0]), v[0], v[2]));
    v2.push((v[1]*(m-v[1]), v[0], v[2]));
    v2.push((v[2]*(m-v[2]), v[1], v[3]));
    v2.push((v[3]*(m-v[3]), v[2], v[4]));
    v2.push((v[4]*(m-v[4]), v[2], v[4]));

    v2.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    max(v2[4].0, v2[4].1, m, r)
}

fn read_line() -> f64{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no input");
    input.trim().to_string().parse::<f64>().unwrap()
}