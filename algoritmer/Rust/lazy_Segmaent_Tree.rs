

fn main(){
    let array = vec![1,2,3,4,5,6,7,8,9,10];
    let mut lezy_segment_tree = LazySegmentTree::new(array.len());
    for i in 0..array.len() {
        lezy_segment_tree.update(i, array[i]);
    }
    println!("{}", lezy_segment_tree.query(0, 9));// = 55
}

struct LazySegmentTree {
    v: Vec<usize>,
    lazy: Vec<usize>,
}

//implement