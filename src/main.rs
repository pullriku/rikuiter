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
    let v = RangeUsize::new(0, 10).skip(3).collect_vec();
    println!("[skip] 0..10 skip(3) => {:?}", v); // [3,4,5,6,7,8,9]

    let v = RangeUsize::new(0, 10).take(4).collect_vec();
    println!("[take] 0..10 take(4) => {:?}", v); // [0,1,2,3]

    // skipしてからtake
    let v = RangeUsize::new(0, 10).skip(3).take(4).collect_vec();
    println!("[skip+take] 0..10 skip(3).take(4) => {:?}", v); // [3,4,5,6]

    // takeしてからskip
    let v = RangeUsize::new(0, 10)
        .take(7) // [0,1,2,3,4,5,6]
        .skip(3) // [3,4,5,6]
        .collect_vec();
    println!("[take+skip] 0..10 take(7).skip(3) => {:?}", v); // [3,4,5,6]

    // skipが大きすぎると空
    let v = RangeUsize::new(0, 5).skip(100).collect_vec();
    println!("[skip too much] 0..5 skip(100) => {:?}", v); // []

    let v = RangeUsize::new(0, 5).take(0).collect_vec();
    println!("[take 0] 0..5 take(0) => {:?}", v); // []

    let s = RangeUsize::new(0, 10).skip(3).take(4).sum();
    println!("[sum] 0..10 skip(3).take(4) sum => {}", s); // 3+4+5+6 = 18

    let n = RangeUsize::new(0, 10).skip(3).take(4).count();
    println!("[count] 0..10 skip(3).take(4) count => {}", n); // 4

    let last = RangeUsize::new(0, 10).skip(3).take(4).last();
    println!("[last] 0..10 skip(3).take(4) last => {:?}", last); // Some(6)
}
