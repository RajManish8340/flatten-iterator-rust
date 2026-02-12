pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            front_iter: None,
            back_iter: None,
        }
    }
}

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub trait IteratorExt: Iterator + Sized {
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator;
}

impl<T> IteratorExt for T
where
    T: Iterator,
{
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
    {
        flatten(self)
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(front_iter) = &mut self.front_iter {
                if let Some(i) = front_iter.next() {
                    return Some(i);
                }
                self.front_iter = None;
            }

            if let Some(next_outer) = self.outer.next() {
                self.front_iter = Some(next_outer.into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(back_iter) = &mut self.back_iter {
                if let Some(i) = back_iter.next_back() {
                    return Some(i);
                }
            }
            self.back_iter = None;

            if let Some(next_back_outer) = self.outer.next_back() {
                self.back_iter = Some(next_back_outer.into_iter())
            } else {
                return self.front_iter.as_mut()?.next_back();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }

    #[test]
    fn empty_wide() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0)
    }

    #[test]
    fn once() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1)
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        )
    }
    #[test]
    fn reverse_wide() {
        assert_eq!(
            flatten(vec![vec!["a",], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        )
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a", "b", "c"], vec!["d", "e", "f"]]);
        assert_eq!(iter.next(), Some("a"));
        assert_eq!(iter.next(), Some("b"));
        assert_eq!(iter.next_back(), Some("f"));
        assert_eq!(iter.next(), Some("c"));
        assert_eq!(iter.next_back(), Some("e"));
        assert_eq!(iter.next_back(), Some("d"));
    }

    #[test]
    fn ext() {
        assert_eq!(vec![vec!["a", "b"]].into_iter().our_flatten().count(), 2);
    }
}
