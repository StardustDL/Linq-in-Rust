type SelectOneIterator<I, F> = std::iter::Map<I, F>;

pub fn select_one<I: Iterator, B, F>(iter: I, f: F) -> SelectOneIterator<I, F>
where
    F: FnMut(I::Item) -> B,
{
    iter.map(f)
}

#[derive(Clone)]
pub struct SelectTwoIterator<I, F> {
    iter: I,
    f: F,
}

impl<B, I: Iterator<Item = (K, V)>, F, K, V> Iterator for SelectTwoIterator<I, F>
where
    F: FnMut(K, V) -> B,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        match self.iter.next() {
            Some((k, v)) => Some((self.f)(k, v)),
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub fn select_two<B, I: Iterator<Item = (K, V)>, F, K, V>(iter: I, f: F) -> SelectTwoIterator<I, F>
where
    F: FnMut(K, V) -> B,
{
    SelectTwoIterator { iter, f }
}
