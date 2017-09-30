use std::collections::HashMap;

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
                    for (_, v) in self.sets.iter_mut()
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
        };
    }

    fn max_set_len(&self) -> usize {
        *self.set_sizes.values().max().unwrap_or(&0)
    }

    fn min_set_len(&self) -> usize {
        *self.set_sizes.values().min().unwrap_or(&0)
    }
}
