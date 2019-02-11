pub struct Expansion<I, T, F> {
    iter: I,
    citer: Option<T>,
    func: F,
}

impl<I, T, F> Expansion<I, T, F> {
    pub fn new(iter: I, citer: Option<T>, func: F) -> Self {
        Expansion { iter, citer, func }
    }
}

impl<I: Iterator, T: Iterator, F> Iterator for Expansion<I, T, F>
where
    F: FnMut(I::Item) -> T,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cit) = &mut self.citer {
            for x in cit {
                return Some(x);
            }
        }
        for tit in &mut self.iter {
            self.citer = Some((self.func)(tit));
            if let Some(cit) = &mut self.citer {
                for x in cit {
                    return Some(x);
                }
            }
        }
        None
    }
}
