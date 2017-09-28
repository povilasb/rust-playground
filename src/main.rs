use std::collections::{HashSet, HashMap};
use std::ops::Fn;
use std::cmp::{max, min};

mod stdio;
use stdio::read_numbers;


fn main() {
    let mut sets = DisjointSets::new();

    let edge_count = read_numbers::<usize>()[0];
    for _ in 0..edge_count {
        let edge = read_numbers::<u32>();
        // TODO: why?
        if edge.len() > 0 {
            sets.add(&edge[..]);
        }
    }

    println!("{:?}", sets.sets);
    println!("{} {}", sets.min_set_len(), sets.max_set_len());
}

struct DisjointSets {
    sets: HashMap<u32, usize>,
    next_list: usize,
}

impl DisjointSets {
    fn new() -> DisjointSets {
        DisjointSets {
            sets: HashMap::new(),
            next_list: 0,
        }
    }

    fn add(&mut self, edge: &[u32]) {
       // TODO: don't clone - how?
       let set_indices = (self.sets.get(&edge[0]).cloned(),
                          self.sets.get(&edge[1]).cloned());
       match set_indices {
            (None, None) => {
                self.sets.insert(edge[0], self.next_list);
                self.sets.insert(edge[1], self.next_list);
                self.next_list += 1;
            },
            (None, Some(set_index)) => {
                self.sets.insert(edge[0], set_index);
            },
            (Some(set_index), None) => {
                self.sets.insert(edge[1], set_index);
            },
            (Some(set1), Some(set2)) => {
                for (k, v) in self.sets.iter_mut()
                                       .filter(|&(_, ref s)| *s == &set2) {
                    *v = set1;
                }
                self.next_list += 1;
            },
            _ => (),
        };
    }

    fn max_set_len(&self) -> usize {
        let mut max_len = 0;
        for seti in self.sets.values() {
            max_len = max(max_len,
                          self.sets.values().filter(|&s| s == seti).count());
        }
        max_len
    }

    fn min_set_len(&self) -> usize {
        let mut min_len = 65535;
        for seti in self.sets.values() {
            min_len = min(min_len,
                          self.sets.values().filter(|&s| s == seti).count());
        }
        min_len
    }
}

fn split_vec<T, F>(vec: &mut Vec<T>, when: F) -> usize
        where F: Fn(&T) -> bool {
    let mut lpos = 0;
    let mut rpos = vec.len() - 1;

    'outer: while lpos < rpos {
        if when(&vec[lpos]) {
            while when(&vec[rpos]) {
                if rpos == lpos {
                    break 'outer;
                }
                rpos -= 1;
            }

            vec.swap(lpos, rpos);
        }

        lpos += 1;
    }
    lpos
}

fn contains_any(set: &HashSet<u32>, values: &[u32]) -> bool {
    values.iter().any(|val| set.contains(val))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! set {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_set = HashSet::new();
                $(
                    temp_set.insert($x);
                )*
                temp_set
            }
        };
    }

    mod contains_any {
        use super::*;

        #[test]
        fn it_returns_true_when_at_least_one_value_is_not_present_in_set() {
            let all = contains_any(&set![1, 2, 3], &[1, 4]);

            assert!(all == true);
        }
    }

    mod disjoint_sets {
        use super::*;

        #[test]
        fn add_inserts_disjoint_tuples_as_separate_sets() {
            let mut sets = DisjointSets::new();

            sets.add(&[1, 6]);
            sets.add(&[2, 7]);

            assert!(sets.sets[0] == set![1, 6]);
            assert!(sets.sets[1] == set![2, 7]);
        }

        #[test]
        fn add_inserts_to_existing_set_when_theres_joining_element() {
            let mut sets = DisjointSets::new();

            sets.add(&[1, 6]);
            sets.add(&[2, 6]);

            assert!(sets.sets[0] == set![1, 6, 2]);
        }
    }
}
