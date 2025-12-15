use crate::iter::MyIterator;

pub struct Take<I> {
    inner: I,
    remaining: usize,
}

impl<I> Take<I> {
    pub fn new(iter: I, n: usize) -> Self {
        Self {
            inner: iter,
            remaining: n,
        }
    }
}

impl<I> MyIterator for Take<I>
where
    I: MyIterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        match self.inner.next() {
            Some(x) => {
                self.remaining -= 1;
                Some(x)
            }
            None => None,
        }
    }
}
