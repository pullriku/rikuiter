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

    let iter = iter.skip(5);
    println!("{iter:?}");

    let iter = iter.map(|x| x * 2);
    println!("{iter:?}");
}
