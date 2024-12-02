use std::{fs, collections::{HashMap, VecDeque, HashSet, BinaryHeap}, cmp::Ordering};

#[derive(Clone, Eq, PartialEq, Debug)]
struct Brick {
    id:usize,
    x:(usize, usize),
    y:(usize, usize),
    z:(usize, usize),
    over:HashSet<usize>,
    under:usize,
    will_fall:usize,
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

    //let file_path = "src\\advent_of_code\\2023\\data\\day_22_test.txt"; // 7
    //let file_path = "src\\advent_of_code\\2023\\data\\day_22_test2.txt"; // 3
    //let file_path = "src\\advent_of_code\\2023\\data\\day_22_test3.txt"; // 9
    //let file_path = "src\\advent_of_code\\2023\\data\\day_22_test4.txt"; // 6
    //let file_path = "src\\advent_of_code\\2023\\data\\day_22_test5.txt"; // 13
    let file_path = "src\\advent_of_code\\2023\\data\\day_22.txt"; // 79144
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let mut stack = BinaryHeap::new();
    let mut x_max = 0;
    let mut y_max = 0;
    for c in (contents.trim().split("\n")).enumerate() {
        let b = fix_input(c.1, c.0);
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
    total += run(&mut map);

    println!("total = {}", total);
}


fn run(map: &mut HashMap<usize, Vec<Brick>>) -> i64 {
    let mut can_remove = 0;

    let mut hights: Vec<usize> = map.keys().map(|&x| x).collect();
    hights.sort();

    for i in (0..hights.len()).rev() {
        let m = map.clone();
        for b in map.get_mut(&hights[i]).unwrap() {
            if b.over.len() == 0 {
                b.will_fall = 0;
                continue;
            }
            assert!(hights.contains(&(b.z.1+1)));

            let mut hm = HashMap::new();
            for other in m.get(&(b.z.1+1)).unwrap() {
                if b.x.1 < other.x.0 || b.x.0 > other.x.1 {continue;}
                if b.y.1 < other.y.0 || b.y.0 > other.y.1 {continue;}
                assert_eq!(b.z.1+1, other.z.0);
                assert_ne!(0, other.under);
                if other.under == 1 {
                    b.will_fall += 1;
                    for other2 in other.over.clone() {
                        if !hm.contains_key(&other2) {
                            hm.insert(other2, (1, other.z.1+1));
                        } else {
                            hm.get_mut(&other2).unwrap().0 += 1;
                        }
                    }
                }
            }
            b.will_fall += domino_fall(&mut hm, m.clone());
            can_remove += b.will_fall;
        }
    }

    can_remove as i64
}


fn domino_fall(hm: &mut HashMap<usize, (usize, usize)>, map: HashMap<usize, Vec<Brick>>) -> usize{
    let mut domino = 0;

    while !hm.is_empty() {
        let mut values: Vec<(usize, usize, usize)> = hm.iter()
            .map(|x| (*x.0, x.1.0, x.1.1)).collect();
        values.sort_by(|a, b| a.2.cmp(&b.2));
        let value = values[0];
        hm.remove(&value.0);

        for b in map.get(&value.2).unwrap() {
            if b.id != value.0 {continue;}
            assert!(b.under >= value.1);
            if b.under != value.1 {break;}

            domino += 1;
            for other in b.over.clone() {
                if !hm.contains_key(&other) {
                    hm.insert(other, (1, b.z.1+1));
                } else {
                    hm.get_mut(&other).unwrap().0 += 1;
                }
            }
        }
    }

    domino
}


fn find_over_under(map: &mut HashMap<usize, Vec<Brick>>) {
    let mut hights: Vec<usize> = map.keys().map(|&x| x).collect();
    hights.sort();
    println!("hights = {:?}", hights);

    for i in 0..hights.len() {
        let m = map.clone();
        if i == hights.len()-1 {break;}
        //under
        for b in m.get(&hights[i]).unwrap() {
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
        //over
        for b in map.get_mut(&hights[i]).unwrap() {
            if !hights.contains(&(b.z.1+1)){
                continue;
            }
            for other in m.get(&(b.z.1+1)).unwrap() {
                if b.x.1 < other.x.0 || b.x.0 > other.x.1 {continue;}
                if b.y.1 < other.y.0 || b.y.0 > other.y.1 {continue;}
                assert_eq!(b.z.1+1, other.z.0);
                b.over.insert(other.id);
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


fn fix_input(line: &str, id: usize) -> Brick {
    let t = line.trim().replace("~", ",").split(",")
        .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    Brick{id: id, x: (t[0],t[3]), y: (t[1],t[4]), z: (t[2],t[5]), over: HashSet::new(), under: 0, will_fall:0}
}
