use std::{fs, collections::{HashMap, VecDeque, HashSet, BinaryHeap}, cmp::Ordering};

#[derive(Clone, Eq, PartialEq, Debug)]
struct Brick {
    x:(usize, usize),
    y:(usize, usize),
    z:(usize, usize),
    over:usize,
    under:usize,
}

impl Ord for Brick{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.z.0.cmp(&self.z.0)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main(){
    let mut total: i64 = 0;

    //let file_path = "src\\advent_of_code\\2023\\data\\day_22_test.txt"; // 5
    let file_path = "src\\advent_of_code\\2023\\data\\day_22.txt"; // 475
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let mut stack = BinaryHeap::new();
    let mut x_max = 0;
    let mut y_max = 0;
    for c in contents.trim().split("\n") {
        let b = fix_input(c);
        if b.x.1 > x_max {x_max = b.x.1}
        if b.y.1 > y_max {y_max = b.y.1}
        stack.push(b.clone());
    }

    let test1 = stack.len();
    let mut map = find_fall_pos(stack, x_max, y_max);
    let mut test2 = 0;
    for m in map.clone() {
        test2 += m.1.len();
    }
    println!("test1 = {}, test2 = {}", test1, test2);
    find_over_under(&mut map);
    total += run(&map);

    println!("total = {}", total);
}


fn run(map: &HashMap<usize, Vec<Brick>>) -> i64 {
    let mut can_remove = 0;

    let mut hights: Vec<usize> = map.keys().map(|&x| x).collect();
    hights.sort();
    //println!("hights = {:?}", hights);

    for i in 0..hights.len() {
        if i == hights.len()-1 {
            can_remove += map.get(&hights[i]).unwrap().len();
            //println!("top, b = {:?}", map.get(&hights[i]).unwrap().clone());
            break;
        }
        for b in map.get(&hights[i]).unwrap() {
            //println!("b = {:?}", b.clone());
            let mut removeble = true;
            if !hights.contains(&(b.z.1+1)){
                can_remove += 1;
                continue;
            }
            for other in map.get(&(b.z.1+1)).unwrap() {
                if b.x.1 < other.x.0 || b.x.0 > other.x.1 {continue;}
                if b.y.1 < other.y.0 || b.y.0 > other.y.1 {continue;}
                assert_eq!(b.z.1+1, other.z.0);
                assert_ne!(0, other.under);
                if other.under == 1 {
                    removeble = false;
                    break;
                }
            }
            if removeble {
                can_remove += 1;
            }
        }
    }

    can_remove as i64
}


fn find_over_under(map: &mut HashMap<usize, Vec<Brick>>) {
    let mut hights: Vec<usize> = map.keys().map(|&x| x).collect();
    hights.sort();
    println!("hights = {:?}", hights);

    for i in 0..hights.len() {
        if i == hights.len()-1 {break;}
        for b in map.clone().get(&hights[i]).unwrap() {
            if !map.contains_key(&(b.z.1+1)){
                continue;
            }
            for other in map.get_mut(&(b.z.1+1)).unwrap() {
                if b.x.1 < other.x.0 || b.x.0 > other.x.1 {continue;}
                if b.y.1 < other.y.0 || b.y.0 > other.y.1 {continue;}
                assert_eq!(b.z.1+1, other.z.0);
                other.under += 1;
            }
        }
    }
}


fn find_fall_pos(mut stack: BinaryHeap<Brick>, x_max: usize, y_max: usize) -> HashMap<usize, Vec<Brick>> {
    let mut map = HashMap::new();
    let mut hight = Vec::new();
    for i in 0..y_max+1 as usize{
        hight.push(Vec::new());
        for _ in 0..x_max+1 {
            hight[i].push(0);
        }
    }

    while !stack.is_empty() {
        let mut b = stack.pop().unwrap();
        //println!("b = {:?}", b.clone());
        let mut higest = 0;
        if b.x.0 != b.x.1 {
            assert_eq!(b.y.0, b.y.1);
            assert_eq!(b.z.0, b.z.1);
            for i in b.x.0..b.x.1+1 {
                if hight[b.y.0][i] > higest {
                    higest = hight[b.y.0][i];
                }
            }
            assert!(higest < b.z.0);
            b.z = (higest+1, higest+1);
            for i in b.x.0..b.x.1+1 {
                hight[b.y.0][i] = higest+1;
            }
        } else if b.y.0 != b.y.1 {
            assert_eq!(b.x.0, b.x.1);
            assert_eq!(b.z.0, b.z.1);
            for i in b.y.0..b.y.1+1 {
                if hight[i][b.x.0] > higest {
                    higest = hight[i][b.x.0];
                }
            }
            assert!(higest < b.z.0);
            b.z = (higest+1, higest+1);
            for i in b.y.0..b.y.1+1 {
                hight[i][b.x.0] = higest+1;
            }
        } else {
            assert_eq!(b.x.0, b.x.1);
            assert_eq!(b.y.0, b.y.1);
            higest = hight[b.y.0][b.x.0];
            let dif = b.z.0-(higest+1);
            assert!(higest < b.z.0);
            b.z = (b.z.0-dif, b.z.1-dif);
            hight[b.y.0][b.x.0] = b.z.1;
        }
        assert_eq!(higest+1, b.z.0);
        //println!("b = {:?}", b.clone());
        if !map.contains_key(&(b.z.0)) {
            map.insert(b.z.0, Vec::from([b]));
        } else {
            map.get_mut(&b.z.0).unwrap().push(b);
        }
    }
    println!("stack.len() = {}", stack.len());
    println!("map.len() = {}", map.len());
    map
}


fn fix_input(line: &str) -> Brick {
    let t = line.trim().replace("~", ",").split(",")
        .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    Brick{x: (t[0],t[3]), y: (t[1],t[4]), z: (t[2],t[5]), over: 0, under: 0}
}
