pub fn single<I: Iterator>(mut iter: I) -> Option<I::Item> {
    match iter.next() {
        Some(item) => {
            if iter.next().is_none() {
                Some(item)
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn contains<I: Iterator<Item = V>, V: Eq>(mut iter: I, value: &V) -> bool {
    iter.any(|item| &item == value)
}
