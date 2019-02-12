pub struct ZipFromIterator<I: Iterator, T, F> {
    iter: I,
    ci: Option<I::Item>,
    citer: Option<T>,
    func: F,
}

impl<I: Iterator, T: Iterator, F> Iterator for ZipFromIterator<I, T, F>
where
    I::Item: Clone,
    F: FnMut(&I::Item) -> T,
{
    type Item = (I::Item, T::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cit) = &mut self.citer {
            for x in cit {
                return Some((self.ci.as_ref().unwrap().clone(), x));
            }
        }
        loop {
            self.ci = self.iter.next();
            if self.ci.is_none() {
                break;
            }
            self.citer = Some((self.func)(self.ci.as_ref().unwrap()));
            if let Some(cit) = &mut self.citer {
                for x in cit {
                    return Some((self.ci.as_ref().unwrap().clone(), x));
                }
            }
        }
        None
    }
}

pub fn zip_from<I: Iterator, T: Iterator, F>(iter: I, func: F) -> ZipFromIterator<I, T, F>
where
    F: FnMut(&I::Item) -> T,
{
    ZipFromIterator {
        iter,
        ci: None,
        citer: None,
        func,
    }
}
