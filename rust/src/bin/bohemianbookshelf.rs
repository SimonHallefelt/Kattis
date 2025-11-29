use std::{collections::HashSet, io};

fn main() {
    let mut input = read_line_stdin();
    let (n,h,w) = (input[0], input[1], input[2]);
    let mut books = Vec::new();
    for _ in 0..n {
        input = read_line_stdin();
        books.push((input[0], input[1]));
    }
    let mut must_upright = Vec::new();
    let mut must_stacked = Vec::new();
    let mut books_left = Vec::new();
    for (i, book) in books.iter().enumerate() {
        if (book.0 > h && book.0 > w) || (book.1 > h && book.1 > w) {
            println!("impossible");
            return;
        }
        if book.0 > h || book.1 > w {
            must_stacked.push(i);
        } 
        else if book.0 > w || book.1 > h {
            must_upright.push(i);
        } 
        else {
            books_left.push(i);
        }
    }
    books_left.sort_by(|a,b| books[*a].0.cmp(&books[*b].0));
    let s_h = must_stacked.iter().map(|a| books[*a].1).sum();
    let s_w = must_stacked.iter().map(|a| books[*a].0).max().unwrap_or(0);
    let u_w = must_upright.iter().map(|a| books[*a].1).sum::<i32>() 
                    + books_left.iter().map(|a| books[*a].1).sum::<i32>();

    let res = run(0, h, w, s_h, s_w, u_w, &mut HashSet::new(), &books, &books_left, &mut must_stacked);
    if !res {
        println!("impossible");
    }
}

fn run(i:usize, h:i32, w:i32, s_h:i32, s_w:i32, u_w:i32, dp:&mut HashSet<(usize,i32,i32)>, books:&Vec<(i32,i32)>, books_left:&Vec<usize>, mut stacked_books:&mut Vec<usize>) -> bool {
    if !dp.insert((i, s_h, s_w)) {
        return false;
    }
    if s_h > h {
        return false;
    }
    if i == books_left.len() {
        return check_if_passed(h, w, s_h, s_w, u_w, books, &stacked_books)
    }

    if run(i+1, h, w, s_h, s_w, u_w, dp, books, books_left, stacked_books) {
        return true;
    }
    stacked_books.push(books_left[i]);
    let book = books[books_left[i]];
    if run(i+1, h, w, s_h+book.1, s_w.max(book.0), u_w-book.1, dp, books, books_left, stacked_books) {
        return true;
    }
    stacked_books.pop();
    false
}

fn check_if_passed(h:i32, w:i32, s_h:i32, s_w:i32, u_w:i32, books:&Vec<(i32,i32)>, stacked_books:&Vec<usize>) -> bool{
    if s_w == 0 || u_w == 0 || u_w + s_w > w {
        return false;
    }

    let mut stacked = Vec::new();
    print!("upright");
    for i in 0..books.len() {
        if stacked_books.contains(&i) {
            stacked.push(i);
        }
        else {
            print!(" {}", i+1);
        }
    }
    println!();
    stacked.sort_by(|a,b| books[*a].cmp(&books[*b]));
    print!("stacked");
    for i in stacked.iter().rev() {
        print!(" {}", i+1);
    }

    true
}

fn read_line_stdin() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input = input.trim().to_string();
    input.split_whitespace().map(|f| f.parse().unwrap()).collect()
}