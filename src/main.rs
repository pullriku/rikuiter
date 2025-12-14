use rikuiter::iter::{MyIterator, StdIter};

struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl MyIterator for Counter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.count;
        self.count += 1;
        Some(result)
    }
}

struct RangeUsize {
    start: usize,
    end: usize,
}

impl RangeUsize {
    fn new(start: usize, end: usize) -> RangeUsize {
        RangeUsize { start, end }
    }
}

impl MyIterator for RangeUsize {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let result = self.start;
            self.start += 1;
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    println!("== MyIteratorを手で回すデモ ==");

    let mut range = RangeUsize::new(1, 4);
    while let Some(x) = range.next() {
        println!("Raw Range: {}", x);
    }

    println!("\n== StdIterでラップしてforループを使う ==");

    let range_std = StdIter(RangeUsize::new(1, 4));
    for x in range_std {
        println!("StdIter: {}", x);
    }

    println!("\n== 標準メソッドを使いまくる ==");

    let counter_std = StdIter(Counter::new());
    let vec: Vec<usize> = counter_std
        .map(|x| x * 10)
        .filter(|x| x % 4 == 0)
        .skip(1)
        .take(10)
        .collect();

    println!("Result: {vec:?}"); // Result: [20, 40, 60, 80, 100, 120, 140, 160, 180, 200]
}
