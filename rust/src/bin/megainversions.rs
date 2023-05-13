use std::io;

fn main(){
    let n: usize = input().parse().unwrap();
    let vec: Vec<i32> = input().split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    let mut vec2: Vec<(i32, i32)> = vec![(0, 0); n];
    for i in 0..n {
        vec2[i] = (vec[i], i as i32);
    }
    vec2.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let mut total = 0;
    let mut s1 = SegmentTree::new(n);
    let mut s2 = SegmentTree::new(n);
    for (_, index) in vec2 {
        let temp = s2.query(index as usize, n-1);
        total += temp as i32;

        s2.update(index as usize, s1.query(index as usize, n-1));
        s1.update(index as usize, 1);
    }

    println!("{}", total);
}

fn input() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    return input;
}


struct SegmentTree {
    v: Vec<usize>,
}

//implement 
impl SegmentTree {
    fn new(n: usize) -> Self {
        SegmentTree {
            v: vec![0; n.next_power_of_two() * 2],
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
        //summ
        a + b
    }
}
