use std::ops;

pub trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    // consume系

    fn count(mut self) -> usize
    where
        Self: Sized,
    {
        let mut count = 0;

        while self.next().is_some() {
            count += 1;
        }

        count
    }

    fn for_each<F>(mut self, mut f: F)
    where
        F: FnMut(Self::Item),
        Self: Sized,
    {
        while let Some(x) = self.next() {
            f(x);
        }
    }

    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let mut last = None;
        while let Some(x) = self.next() {
            last = Some(x);
        }
        last
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let mut i = 0;
        while let Some(x) = self.next() {
            if i == n {
                return Some(x);
            }
            i += 1;
        }
        None
    }

    fn find<P>(&mut self, mut predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        while let Some(x) = self.next() {
            if predicate(&x) {
                return Some(x);
            }
        }
        None
    }

    fn fold<B, F>(mut self, mut acc: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
        Self: Sized,
    {
        while let Some(x) = self.next() {
            acc = f(acc, x);
        }
        acc
    }

    fn sum(self) -> Self::Item
    where
        Self::Item: ops::Add<Output = Self::Item> + Default,
        Self: Sized,
    {
        self.fold(Self::Item::default(), |acc, x| acc + x)
    }

    fn collect_vec(mut self) -> Vec<Self::Item>
    where
        Self: Sized,
    {
        let mut vec = Vec::new();
        while let Some(x) = self.next() {
            vec.push(x);
        }
        vec
    }
}

pub struct StdIter<T>(pub T);

impl<T: MyIterator> Iterator for StdIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
