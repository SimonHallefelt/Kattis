use std::collections::{HashMap, VecDeque};

// Gale Shapley algorithm: stable_matchings/deferred_acceptance 
// g1: HashMap[g1_node, queue[most wanted g2_node first]]
// g2: Vec[HashMap[g1_node, g2_node desire for of g1_node]]
// return/stable_match: HashMap[g2_node, best g1_node match]
fn stable_matchings(mut g1: HashMap<usize, VecDeque<usize>>, g2: Vec<HashMap<usize, usize>>) -> HashMap<usize, usize> {
    let mut stable_match = HashMap::new();
    let mut no_match = VecDeque::from(g1.keys().map(|a| *a).collect::<Vec<usize>>());
    let mut g1_node;
    let mut g2_node;
    while !no_match.is_empty() {
        g1_node = no_match.pop_front().unwrap();
        let t = g1.get_mut(&g1_node).unwrap().pop_front();
        g2_node = match t {
            Some(t) => t,
            None => continue
        };
        if g2.get(g2_node).unwrap().get(&g1_node).is_none() {continue;} 

        if !stable_match.contains_key(&g2_node) {
            stable_match.insert(g2_node, g1_node);
        } else {
            let new_desirability = *g2.get(g2_node).unwrap().get(stable_match.get(&g2_node).unwrap()).unwrap();
            let current_desirability = *g2.get(g2_node).unwrap().get(&g1_node).unwrap();
            if new_desirability > current_desirability {
                no_match.push_back(stable_match.insert(g2_node, g1_node).unwrap());
            } else {
                no_match.push_back(g1_node);
            }
        }
    }
    stable_match
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut g1 = HashMap::new();
        let mut g2 = Vec::new();

        // g1
        g1.insert(0, VecDeque::from([0,1,2,3,4]));
        g1.insert(1, VecDeque::from([0,1,2,3,4]));
        g1.insert(2, VecDeque::from([0,1,2,3,4]));
        g1.insert(3, VecDeque::from([0,1,2,3,4]));
        g1.insert(4, VecDeque::from([1,0,2,3,4]));
        
        // g2
        g2.push(HashMap::from([(0,0),(1,1),(2,2),(3,3),(4,4)]));
        g2.push(HashMap::from([(0,4),(1,1),(2,2),(3,3),(4,0)]));
        g2.push(HashMap::from([(0,0),(1,1),(2,0),(3,3),(4,4)]));
        g2.push(HashMap::from([(0,0),(1,3),(2,2),(3,1),(4,4)]));
        g2.push(HashMap::from([(0,0),(1,4),(2,2),(3,3),(4,1)]));

        let sm = stable_matchings(g1, g2);
        println!("{:?}", sm);
        assert_eq!(5, sm.len());
        assert_eq!(0, *sm.get(&0).unwrap());
        assert_eq!(4, *sm.get(&1).unwrap());
        assert_eq!(2, *sm.get(&2).unwrap());
        assert_eq!(3, *sm.get(&3).unwrap());
        assert_eq!(1, *sm.get(&4).unwrap());
    }

    #[test]
    fn test_2() {
        let mut g1 = HashMap::new();
        let mut g2 = Vec::new();

        // g1
        g1.insert(0, VecDeque::from([0,1,2]));
        g1.insert(1, VecDeque::from([0,1,2]));
        g1.insert(2, VecDeque::from([0,1,2]));
        g1.insert(3, VecDeque::from([3,4]));
        g1.insert(4, VecDeque::from([3,4]));
        
        // g2
        g2.push(HashMap::from([(0,0),(1,1),(2,2)]));
        g2.push(HashMap::from([(0,2),(1,1),(2,0)]));
        g2.push(HashMap::from([(0,0),(1,0),(2,2)]));
        g2.push(HashMap::from([(0,0),(1,1)]));
        g2.push(HashMap::from([(0,0),(1,1),(2,2)]));

        let sm = stable_matchings(g1, g2);
        println!("{:?}", sm);
        assert_eq!(3, sm.len());
        assert_eq!(0, *sm.get(&0).unwrap());
        assert_eq!(2, *sm.get(&1).unwrap());
        assert_eq!(1, *sm.get(&2).unwrap());
        assert!(!sm.contains_key(&3));
        assert!(!sm.contains_key(&4));
    }

    #[test]
    fn test_3() {
        let mut g1 = HashMap::new();
        let mut g2 = Vec::new();

        // g1
        g1.insert(0, VecDeque::from([0]));
        g1.insert(1, VecDeque::from([0,1]));
        
        // g2
        g2.push(HashMap::from([]));
        g2.push(HashMap::from([(1,0)]));

        let sm = stable_matchings(g1, g2);
        println!("{:?}", sm);
        assert_eq!(0, sm.len());
    }

    #[test]
    fn test_4() {
        let mut g1 = HashMap::new();
        let mut g2 = Vec::new();

        // g1
        g1.insert(0, VecDeque::from([0]));
        g1.insert(1, VecDeque::from([0,1]));
        
        // g2
        g2.push(HashMap::from([(0,0),(1,1)]));
        g2.push(HashMap::from([(1,0)]));

        let sm = stable_matchings(g1, g2);
        println!("{:?}", sm);
        assert_eq!(2, sm.len());
        assert_eq!(0, *sm.get(&0).unwrap());
        assert_eq!(1, *sm.get(&1).unwrap());
    }
}