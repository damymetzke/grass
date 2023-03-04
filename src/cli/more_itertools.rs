pub struct SandwichMapIterator<I, R, T, U, V>
where
    T: FnMut(&I) -> R,
    U: FnMut(&I) -> R,
    V: FnMut(&I) -> R,
{
    items: Vec<I>,
    offset: usize,

    start: T,
    middle: U,
    end: V,
}

impl<I, R, T, U, V> SandwichMapIterator<I, R, T, U, V>
where
    T: FnMut(&I) -> R,
    U: FnMut(&I) -> R,
    V: FnMut(&I) -> R,
{
    pub fn new<W>(items: W, start: T, middle: U, end: V) -> SandwichMapIterator<I, R, T, U, V>
    where
        W: IntoIterator<Item = I>,
    {
        SandwichMapIterator {
            items: items.into_iter().collect(),
            offset: 0,

            start,
            middle,
            end,
        }
    }
}

impl<I, R, T, U, V> Iterator for SandwichMapIterator<I, R, T, U, V>
where
    T: FnMut(&I) -> R,
    U: FnMut(&I) -> R,
    V: FnMut(&I) -> R,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.items.len() {
            return None;
        };

        let result = match self.offset {
            0 => (self.start)(&self.items[self.offset]),
            _ if self.offset == self.items.len() - 1 => (self.end)(&self.items[self.offset]),
            _ => (self.middle)(&self.items[self.offset]),
        };

        self.offset += 1;

        Some(result)
    }
}

pub trait MoreItertools: Iterator {
    fn sandwich_map<R, T, U, V>(
        self,
        start: T,
        middle: U,
        end: V,
    ) -> SandwichMapIterator<Self::Item, R, T, U, V>
    where
        Self: Sized,
        T: FnMut(&Self::Item) -> R,
        U: FnMut(&Self::Item) -> R,
        V: FnMut(&Self::Item) -> R,
    {
        SandwichMapIterator::new(self, start, middle, end)
    }
}

impl<T: Iterator> MoreItertools for T {}
