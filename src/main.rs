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

    println!("{} {}", sets.min_set_len(), sets.max_set_len());
}

struct DisjointSets {
    sets: HashMap<u32, usize>,
    set_sizes: HashMap<usize, usize>,
    next_list: usize,
}

impl DisjointSets {
    fn new() -> DisjointSets {
        DisjointSets {
            sets: HashMap::new(),
            set_sizes: HashMap::new(),
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
                self.set_sizes.insert(self.next_list, 2);
                self.next_list += 1;
            },
            (None, Some(set_index)) => {
                self.sets.insert(edge[0], set_index);
                let curr_size = *self.set_sizes.get(&set_index).unwrap();
                self.set_sizes.insert(set_index, curr_size + 1);
            },
            (Some(set_index), None) => {
                self.sets.insert(edge[1], set_index);
                let curr_size = *self.set_sizes.get(&set_index).unwrap();
                self.set_sizes.insert(set_index, curr_size + 1);
            },
            (Some(set1), Some(set2)) => {
                if set1 != set2 {
                    let mut set_increase = 0;
                    for (k, v) in self.sets.iter_mut()
                                           .filter(|&(_, ref s)| *s == &set2) {
                        *v = set1;
                        set_increase += 1;
                    }

                    let curr_size = *self.set_sizes.get(&set1).unwrap();
                    self.set_sizes.insert(set1, curr_size + set_increase);
                    self.set_sizes.remove(&set2);

                    self.next_list += 1;
                }
            },
            _ => (),
        };
    }

    fn max_set_len(&self) -> usize {
        *self.set_sizes.values().max().unwrap_or(&0)
    }

    fn min_set_len(&self) -> usize {
        *self.set_sizes.values().min().unwrap_or(&0)
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
