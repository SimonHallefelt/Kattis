use std::{collections::HashSet, io::{self, BufRead}};

fn main() {
    let (x, y, mut screen) = read_input();

    let mut pos = (0,0);
    screen[0][0].is_appendage = true;
    let mut hs = HashSet::new();
    hs.insert(pos);
    let dirs = vec![(1,0),(0,1),(-1,0),(0,-1)];
    'loop1: loop {
        let mut new_pos;
        for dir in &dirs {
            new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if !hs.contains(&new_pos) && (0..y).contains(&new_pos.0) && (0..x).contains(&new_pos.1) {
                if screen[pos.0 as usize][pos.1 as usize].appendage_neighbor(&screen[new_pos.0 as usize][new_pos.1 as usize]) {
                    screen[new_pos.0 as usize][new_pos.1 as usize].is_appendage = true;
                    hs.insert(new_pos);
                    pos = new_pos;
                    continue 'loop1;
                }
            }
        }
        break;
    }

    for yy in 0..y as usize {
        for xx in 0..x as usize {
            print!("{}", screen[yy][xx].get_pixel());
        }
        println!()
    }
}

fn read_input() -> (i32, i32, Vec<Vec<Pixel>>){
    let mut row = String::new();
    io::stdin().read_line(&mut row).expect("Failed to read line");
    let temp: Vec<i32> = row.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (y,x) = (temp[0], temp[1]);

    let mut screen= Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines().take(y as usize) {
        let row = line.expect("Failed to read line");
        let pixels = row.trim().chars().map(Pixel::new).collect();
        screen.push(pixels);
    }

    (x, y, screen)
}

struct Pixel {
    pc: PixelColor,
    is_appendage: bool,
}

impl Pixel {
    fn new(c: char) -> Self {
        let pc = match c {
            '#' => PixelColor::Red,
            'O' => PixelColor::White,
            _ => PixelColor::Rest,
        };
        let is_appendage = false;
        Self {pc, is_appendage}
    }

    fn appendage_neighbor(&self, p: &Pixel) -> bool {
        self.pc != p.pc && self.pc != PixelColor::Rest && p.pc != PixelColor::Rest
    }

    fn get_pixel(&self) -> char {
        if self.is_appendage {
            return '.';
        }
        match self.pc {
            PixelColor::Red => '#',
            PixelColor::White => 'O',
            PixelColor::Rest => '.'
        }
    }
}

#[derive(PartialEq)]
enum PixelColor {
    Red, White, Rest
}