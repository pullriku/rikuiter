use std::{collections::HashSet, fs, num::NonZero, path::Path};

use dot_graph::Edge;
use indicatif::ProgressIterator;
use rikuiter::iter::MyIterator;

struct Collatz {
    n: Option<NonZero<usize>>,
}

impl Collatz {
    fn new(n: NonZero<usize>) -> Self {
        Self { n: Some(n) }
    }
}

impl MyIterator for Collatz {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n?;

        let value = n.get();

        self.n = if value == 1 {
            None
        } else if value % 2 == 0 {
            Some(unsafe { NonZero::new_unchecked(value / 2) })
        } else {
            Some(unsafe { NonZero::new_unchecked(3 * value + 1) })
        };

        Some(value)
    }
}

fn main() {
    let collatz = Collatz::new(NonZero::new(10).unwrap()).collect_vec();
    assert_eq!(collatz, [10, 5, 16, 8, 4, 2, 1]);

    let collatz = Collatz::new(NonZero::new(7).unwrap()).collect_vec();
    assert_eq!(
        collatz,
        [7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1]
    );

    let collatz = Collatz::new(NonZero::new(1000).unwrap()).collect_vec();
    assert_eq!(
        collatz,
        [
            1000, 500, 250, 125, 376, 188, 94, 47, 142, 71, 214, 107, 322, 161, 484, 242, 121, 364,
            182, 91, 274, 137, 412, 206, 103, 310, 155, 466, 233, 700, 350, 175, 526, 263, 790,
            395, 1186, 593, 1780, 890, 445, 1336, 668, 334, 167, 502, 251, 754, 377, 1132, 566,
            283, 850, 425, 1276, 638, 319, 958, 479, 1438, 719, 2158, 1079, 3238, 1619, 4858, 2429,
            7288, 3644, 1822, 911, 2734, 1367, 4102, 2051, 6154, 3077, 9232, 4616, 2308, 1154, 577,
            1732, 866, 433, 1300, 650, 325, 976, 488, 244, 122, 61, 184, 92, 46, 23, 70, 35, 106,
            53, 160, 80, 40, 20, 10, 5, 16, 8, 4, 2, 1
        ]
    );

    eprintln!("テストOK!");

    let mut visited: HashSet<usize> = HashSet::new();
    let mut graph = dot_graph::Graph::new("collatz", dot_graph::Kind::Digraph);

    const COUNT: usize = 1000000;
    for i in (1..COUNT+1).progress() {
        let collatz = Collatz::new(NonZero::new(i).unwrap());

        for slice in collatz.collect_vec().windows(2){
            let [x, next] = slice else { unreachable!() };
            if visited.contains(x) {
                break;
            }

            visited.insert(*x);
            graph.add_edge(Edge::new(&format!("_{}", x), &format!("_{}", next), &format!("{} -> {}", x, next)));
        }
    }

    let dot_string = graph.to_dot_string().unwrap();
    fs::write(Path::new("graph.dot"), dot_string).unwrap();
}
