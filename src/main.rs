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
    print!("[filter] even in 0..10 =>");
    RangeUsize::new(0, 10)
        .filter(|x| *x % 2 == 0)
        .for_each(|x| print!(" {}", x));
    println!(); // 0 2 4 6 8

    let n = RangeUsize::new(0, 100).filter(|x| *x % 3 == 0).count();
    println!("[filter+count] multiples of 3 in 0..100 => {}", n); // 34

    let last_even = RangeUsize::new(0, 10).filter(|x| *x % 2 == 0).last();
    println!("[filter+last] last even in 0..10 => {:?}", last_even); // Some(8)

    let sum_even = RangeUsize::new(1, 11).filter(|x| *x % 2 == 0).sum();
    println!("[filter+fold] sum of evens in 1..11 => {}", sum_even); // 2+4+6+8+10 = 30

    let v = RangeUsize::new(0, 20).filter(|x| *x % 7 == 0).collect_vec();
    println!("[filter+collect_vec] multiples of 7 in 0..20 => {:?}", v); // [0, 7, 14]

    let mut it = RangeUsize::new(0, 20).filter(|x| *x % 5 == 0);
    //  nth は「filter後の列」に対して数える
    let third = it.nth(3); // 0,5,10,[15]...
    println!("[filter+nth] (0..20).filter(%5==0).nth(3) => {:?}", third); // Some(15)
    let next = it.next(); // もう次は無い (0,5,10,15 で終わり)
    println!("[filter+nth] then next() => {:?}", next); // None
}
