use rikuiter::iter::MyIterator;

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
    print!("[filter+map] even in 0..10, then *10 =>");
    RangeUsize::new(0, 10)
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 10)
        .for_each(|x| print!(" {}", x));
    println!(); // 0 20 40 60 80

    let v = RangeUsize::new(1, 21)
        .filter(|x| *x % 3 == 0)
        .map(|x| x * 100)
        .collect_vec();
    println!("[filter+map+collect_vec] multiples of 3 in 1..21 => {:?}", v);
    // [300, 600, 900, 1200, 1500, 1800]

    let sum = RangeUsize::new(1, 11)
        .filter(|x| *x % 2 == 1) // odd
        .map(|x| x * x)          // square
        .sum();
    println!("[filter+map+sum] sum of odd squares in 1..11 => {}", sum); // 1+9+25+49+81=165

    let mut it = RangeUsize::new(1, 100)
        .filter(|x| *x % 7 == 0)
        .map(|x| x + 1); // 7->8, 14->15, ...
    let found = it.find(|x| *x % 5 == 0); // 15 が最初に当たる
    println!("[filter+map+find] first (multiple of 7)+1 divisible by 5 => {:?}", found); // Some(15)
    let after = it.next();
    println!("[filter+map+find] then next() => {:?}", after); // Some(22)
}
