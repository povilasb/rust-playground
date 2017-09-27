use std::collections::HashSet;
use std::ops::Fn;
use std::cmp::{max, min};

mod stdio;
use stdio::read_numbers;


fn main() {
    let mut sets = DisjointSets::new();

    let edge_count = read_numbers::<usize>()[0];
    for _ in 0..edge_count {
        let edge = read_numbers::<u32>();
        sets.add(&edge[..]);
    }

    println!("{} {}", sets.min_set_len(), sets.max_set_len());
}

struct DisjointSets {
    sets: Vec<HashSet<u32>>,
}

impl DisjointSets {
    fn new() -> DisjointSets {
        DisjointSets {
            sets: Vec::new(),
        }
    }

    fn add(&mut self, edge: &[u32]) {
        let set: HashSet<u32> = edge.iter().cloned().collect();
        self.sets.push(set);

        let rm_pos = split_vec(&mut self.sets, |ref s| contains_any(&s, edge));
        let new_set = {
            let join_sets = self.sets.drain(rm_pos..);
            join_sets.fold(HashSet::new(), |s1, s2| &s1 | &s2)
        };
        self.sets.push(new_set);
    }

    fn max_set_len(&self) -> usize {
        self.sets.iter().fold(0, |max_len, set| max(max_len, set.len()))
    }

    fn min_set_len(&self) -> usize {
        self.sets.iter().skip(1)
            .fold(self.sets[0].len(), |min_len, set| min(min_len, set.len()))
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
