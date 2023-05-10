use std::io;

fn main(){
    let n_k = read_line();
    let n_k: Vec<&str> = n_k.trim().split(" ").collect();
    let n_k: Vec<i128> = n_k.iter().map(|x| x.parse::<i128>().unwrap()).collect();

    let mut segment_tree = SegmentTree::new(n_k[0] as usize);
    for i in 0..n_k[0] {
        segment_tree.update(i as usize, 0);
    }

    for _ in 0..n_k[1] {
        let temp = read_line();
        let temp: Vec<&str> = temp.trim().split(" ").collect();
        if temp[0] == "F" {
            let i = temp[1].parse::<usize>().unwrap() - 1;
            segment_tree.flip(i);
        } else {
            let l = temp[1].parse::<usize>().unwrap() - 1;
            let r = temp[2].parse::<usize>().unwrap() - 1;
            println!("{}", segment_tree.query(l, r));
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}

struct SegmentTree {
    v: Vec<usize>,
}

//implement 
impl SegmentTree {
    fn new(n: usize) -> Self {
        SegmentTree {
            v: vec![1; n.next_power_of_two() * 2],
        }
    }

    fn query(&self, ql: usize, qr: usize) -> usize {
        self.query_in(1, 0, self.v.len() / 2 - 1, ql, qr)
    }

    fn update(&mut self, p: usize, value: usize) {
        self.update_in(1, 0, self.v.len() / 2 - 1, p, value);
    }

    fn query_in(&self, n: usize, l: usize, r: usize, ql: usize, qr: usize) -> usize {
        if qr < l || r < ql {
            0
        } else if ql <= l && r <= qr {
            self.v[n]
        } else {
            let m = l + (r - l) / 2;
            let al = self.query_in(2*n, l, m, ql, qr);
            let ar = self.query_in(2*n+1, m+1, r, ql, qr);
            Self::combine(al, ar)
        }
    }
    
    fn update_in(&mut self, n: usize, l: usize, r: usize, p: usize, value: usize) -> usize {
        if l > p || r < p {
            self.v[n]
        } else if l == r {
            self.v[n] = value;
            value
        } else {
            let m = l + (r - l) / 2;
            let al = self.update_in(2*n, l, m, p, value);
            let ar = self.update_in(2*n+1, m+1, r, p, value);
            self.v[n] = Self::combine(al, ar);
            self.v[n]
        }
    }

    //ändre till rätt funktion
    fn combine(a: usize, b: usize) -> usize {
        a + b
    }

    fn flip(&mut self, p: usize) {
        let mut temp = self.query(p, p);
        temp = if temp == 1 { 0 } else { 1 };
        self.update(p, temp);
    }
}