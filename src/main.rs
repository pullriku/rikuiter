use rikuiter::iter::MyIterator;

#[derive(Debug)]
#[allow(unused)]
struct Counter {
    count: usize,
}

#[allow(unused)]
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

#[derive(Debug)]
#[allow(unused)]
struct RangeUsize {
    start: usize,
    end: usize,
}

#[allow(unused)]
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
    let iter = Counter::new();
    println!("{iter:?}");
    // Counter { count: 0 }

    let iter = iter.skip(5);
    println!("{iter:?}");
    // Skip { inner: Counter { count: 0 }, remaining: 5 }

    let iter = iter.map(|x| x * 2);
    println!("{iter:?}");
    // Map { inner: Skip { inner: Counter { count: 0 }, remaining: 5 },
    // f: "|x| ..." }

    let iter = iter.filter(|x| x % 4 == 0);
    println!("{iter:?}");
    // Filter { inner: Map { inner: Skip { inner: Counter { count: 0 },
    //  remaining: 5 }, f: "|x| ..." }, predicate: "|x| ..." }

    let iter = iter.take(3);
    println!("{iter:?}");
    // Take { inner: Filter { inner: Map { inner: Skip { inner: Counter 
    // { count: 0 }, remaining: 5 }, f: "|x| ..." }, predicate: "|x| ..." }, 
    // remaining: 3 }

    let counter_std = Counter::new();
    let vec: Vec<usize> = counter_std
        .map(|x| x * 10)
        .filter(|x| x % 4 == 0)
        .skip(1)
        .take(10)
        .collect_vec();
    println!("{vec:?}");
}
