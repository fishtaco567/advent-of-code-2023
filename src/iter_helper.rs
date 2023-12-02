use std::{fmt::Debug, marker::PhantomData, str::FromStr};

pub trait IterHelper: Iterator {
    fn parse_all<'a, T>(self) -> Parsed<Self, T>
    where
        Self: Sized,
        Self: Iterator<Item = &'a str>,
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        Parsed {
            iter: self,
            _marker: PhantomData,
        }
    }
}

impl<T> IterHelper for T where T: Iterator {}

pub struct Parsed<I, T> {
    iter: I,
    _marker: PhantomData<T>,
}

impl<'a, I, T> Iterator for Parsed<I, T>
where
    I: Iterator<Item = &'a str>,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            Some(str::parse::<T>(item).unwrap())
        } else {
            None
        }
    }
}
