use std::{fs, collections::{HashMap, VecDeque, HashSet, BinaryHeap, btree_set::Intersection}, cmp::Ordering, f32::INFINITY};

#[derive(Clone, Copy, PartialEq, Debug)]
struct Hailstone {
    x:f64,
    y:f64,
    z:f64,
    vx:f64,
    vy:f64,
    vz:f64,
}


// tror man ska ta 3 linjer hitta ytan melan 1 & 2 samt 1 & 3 sen 
// där dessa ytor möts är linjen som borde följas
fn main(){
    let mut total: i64 = 0;

    let file_path = "src\\advent_of_code\\2023\\data\\day_24_test.txt"; // 47
    // let file_path = "src\\advent_of_code\\2023\\data\\day_24.txt"; // 
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let mut hailstones = Vec::new();
    for c in contents.trim().split("\n") {
        let input = fix_input(c);
        hailstones.push(input);
    }

    total += run(&hailstones);

    println!("total = {}", total);
}


fn run(hailstones: &Vec<Hailstone>) -> i64 {
    let mut intersect = 0;
    // let limits: (f64, f64) = (7.0, 27.0); // test
    let limits: (f64, f64) = (200000000000000.0, 400000000000000.0);

    for i in 0..hailstones.len()-1 {
        let h1 = hailstones[i];
        for j in i+1..hailstones.len() {
            let h2 = hailstones[j];
            let intersected = intersection(h1, h2);
            if intersected.0 < limits.0 || intersected.1 < limits.0 {
                println!("hej1, paths will cross outside the test area")
            } else if intersected.0 > limits.1 || intersected.1 > limits.1  {
                println!("hej2, paths will cross outside the test area")
            } else {
                println!("inside test area");
                intersect += 1;
            }
            println!("--------------");
        }
    }

    intersect
}


fn intersection(h1: Hailstone, h2: Hailstone) -> (f64, f64) {
    let mut intersected = (-1.0,-1.0);
    /*
        tideran får vara olika
        h1: y1=19, x1=13, vy1=-2, vx1=1
        h2: y2=18, x2=19, vy2=-1, vx2=-1

        y1 + a * vy1 = y2 + b * vy2
        x1 + a * vx1 = x2 + b * vx2

        y1 - y2 + a * vy1 - b * vy2 = 0
        x1 - x2 + a * vx1 - b * vx2 = 0
        
        y1 - y2 + a * vy1 = b * vy2
        x1 - x2 + a * vx1 = b * vx2

        (x1 - x2 + a * vx1) - (y1 - y2 + a * vy1) * (vx2 / vy2) = 0
        x1 - x2 + a * vx1 = (y1 - y2 + a * vy1) * (vx2 / vy2)
        a * vx1 = (y1 - y2 + a * vy1) * (vx2 / vy2) + (x2 - x1)
        (a * vx1) - (a * vy1) * (vx2 / vy2) = (y1 - y2) * (vx2 / vy2) + (x2 - x1)
        a * (vx1 - vy1 * (vx2 / vy2)) = (y1 - y2) * (vx2 / vy2) + (x2 - x1)
        a = ((y1 - y2) * (vx2 / vy2) + (x2 - x1)) / (vx1 - vy1 * (vx2 / vy2))

        y1 - y2 + a * vy1 - b * vy2 = 0
        (y1 - y2 + a * vy1) / vy2 = b
     */

    // hanteras negativa tal korrekt?
    let a = ((h1.y - h2.y) * (h2.vx / h2.vy) + 
                (h2.x - h1.x)) / (h1.vx - (h1.vy * (h2.vx / h2.vy)));
    let b = (h1.y - h2.y + a * h1.vy) / h2.vy;
    if a >= 0.0 && b >= 0.0 {
        intersected = (h1.y + a * h1.vy, h1.x + a * h1.vx);
        println!("intersected = {:?}, a = {}, b = {}", intersected, a, b);
    } else {
        if a < 0.0 {
            println!("paths crossed in the past for a")
        }
        if b < 0.0 {
            println!("paths crossed in the past for b")
        };
        if intersected.0 == INFINITY as f64 {
            println!("paths are parallel")
        }
        // intersected = (h1.y + a * h1.vy, h1.x + a * h1.vx);
        // println!("intersected = {:?}", intersected)
    }

    // println!("--------------");

    intersected
}


fn fix_input(line: &str) -> Hailstone {
    let i: Vec<f64> = line.trim().replace("@", ",").split(",")
    .map(|x| x.trim().parse::<f64>().unwrap()).collect();
    Hailstone { x: i[0], y: i[1], z: i[2], vx: i[3], vy: i[4], vz: i[5] }
}
