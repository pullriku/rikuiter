trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

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

struct StdIter<T>(T);

impl<T: MyIterator> Iterator for StdIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

fn main() {
    let mut counter = Counter::new();
    println!("{:?}", counter.next()); // Some(0)
    println!("{:?}", counter.next()); // Some(1)
    println!("{:?}", counter.next()); // Some(2)

    // 無限に増加していく
    for i in StdIter(counter) {
        println!("{}", i);
    }
}
