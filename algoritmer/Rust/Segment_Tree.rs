//segement tree för att kunna göra snabba queries på intervall
//alla värden placeras längst ner och varje upp är en sammanfatnig av noderna under

fn main(){
    let array = vec![1,2,3,4,5,6,7,8,9,10];
    let mut segment_tree = SegmentTree::new(array.len());
    for i in 0..array.len() {
        segment_tree.update(i, array[i]);
    }
    println!("{}", segment_tree.query(0, 9));// = 55
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
        //summ
        a + b

        //gcd
        /* if a == 0 {
            b
        } else if b == 0 {
            a
        } else {
            gcd(a, b)
        } */
    }
}

//implement gcd
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/* pub fn gcd<T>(a: T, b: T) -> T
where
    T: Integer + Unsigned,
{
    while b != T::zero() {
        let t = b;
        b = a % b;
        a = t;
    }
} */


