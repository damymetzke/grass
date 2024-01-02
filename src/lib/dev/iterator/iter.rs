use std::iter::repeat_with;

use tracing::error;

pub struct MarkEndIterator<'a, T: Iterator> {
    source: &'a mut T,
    result: Vec<Option<T::Item>>,
}

pub enum TakeUntilIteratorItem<T> {
    Start(T),
    End(T),
}

impl<'a, T: Iterator<Item = U>, U: Clone> Iterator for MarkEndIterator<'a, T> {
    type Item = TakeUntilIteratorItem<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = if let Some(next) = self.source.next() {
                next
            } else {
                for value in 0..self.result.len() {
                    let result = match &self.result[value] {
                        Some(value) => Some(TakeUntilIteratorItem::End(value.clone())),
                        None => continue,
                    };

                    self.result[value] = None;
                    return result;
                }
                break;
            };

            let value = self.result.first().map(|value| (*value).clone());
            match value {
                Some(Some(value)) => {
                    for i in 0..self.result.len() - 1 {
                        self.result[i] = self.result[i + 1].clone();
                    }
                    let length = self.result.len();
                    self.result[length - 1] = Some(next);
                    let result = value.clone();
                    return Some(TakeUntilIteratorItem::Start(result));
                }
                Some(None) => {
                    for i in 0..self.result.len() - 1 {
                        self.result[i] = self.result[i + 1].clone();
                    }
                    let length = self.result.len();
                    self.result[length - 1] = Some(next);
                }
                None => {
                    error!("TakeUntilIterator called with size 0");
                    break;
                }
            };
        }
        None
    }
}

pub trait IterExtensions: Iterator + Sized {
    fn mark_end(&mut self, n: usize) -> MarkEndIterator<Self>;
}

impl<T: Iterator<Item = U> + Sized, U: Default> IterExtensions for T {
    fn mark_end(&mut self, n: usize) -> MarkEndIterator<T> {
        MarkEndIterator {
            source: self,
            result: Vec::from_iter(repeat_with(Default::default).take(n)),
        }
    }
}
