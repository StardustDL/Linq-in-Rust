#[derive(Clone)]
pub struct SelectManyIterator<I: Iterator, T, FC, FR> {
    iter: I,
    ci: Option<I::Item>,
    citer: Option<T>,
    fc: FC,
    fr: FR,
}

impl<I: Iterator, T: Iterator, FC, FR, R> Iterator for SelectManyIterator<I, T, FC, FR>
where
    I::Item: Clone,
    FC: FnMut(I::Item) -> T,
    FR: FnMut(I::Item, T::Item) -> R,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cit) = &mut self.citer {
            for x in cit {
                return Some((self.fr)(self.ci.as_ref().unwrap().clone(), x));
            }
        }
        loop {
            let ci = self.iter.next();
            if ci.is_none() {
                break;
            }
            self.ci = Some(ci.as_ref().unwrap().clone());
            self.citer = Some((self.fc)(ci.unwrap()));
            if let Some(cit) = &mut self.citer {
                for x in cit {
                    return Some((self.fr)(self.ci.as_ref().unwrap().clone(), x));
                }
            }
        }
        None
    }
}

pub fn select_many<I: Iterator, T: Iterator, FC, FR, R>(
    iter: I,
    fc: FC,
    fr: FR,
) -> SelectManyIterator<I, T, FC, FR>
where
    I::Item: Clone,
    FC: FnMut(I::Item) -> T,
    FR: FnMut(I::Item, T::Item) -> R,
{
    SelectManyIterator {
        iter,
        ci: None,
        citer: None,
        fc,
        fr,
    }
}

pub struct SelectManySingleIterator<I, T, F> {
    iter: I,
    citer: Option<T>,
    func: F,
}

impl<I: Iterator, T: Iterator, F> Iterator for SelectManySingleIterator<I, T, F>
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

pub fn select_many_single<I: Iterator, T: Iterator, F>(
    iter: I,
    func: F,
) -> SelectManySingleIterator<I, T, F>
where
    F: FnMut(I::Item) -> T,
{
    SelectManySingleIterator {
        iter,
        citer: None,
        func,
    }
}
