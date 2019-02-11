pub struct ExpansionIterator<I, T, F> {
    iter: I,
    citer: Option<T>,
    func: F,
}

impl<I: Iterator, T: Iterator, F> Iterator for ExpansionIterator<I, T, F>
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

pub fn expansion<I: Iterator, T: Iterator, F>(iter: I, func: F) -> ExpansionIterator<I, T, F>
where
    F: FnMut(I::Item) -> T,
{
    ExpansionIterator {
        iter,
        citer: None,
        func,
    }
}
