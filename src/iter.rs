use soapy_shared::{RawSoa, Soapy};
use std::marker::PhantomData;

/// Immutable [`Soa`] iterator.
///
/// This struct is created by the [`iter`] method.
///
/// [`Soa`]: crate::Soa
/// [`iter`]: crate::Soa::iter
pub struct Iter<'a, T: 'a>
where
    T: Soapy,
{
    pub(crate) raw: T::RawSoa,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Soapy,
{
    type Item = <<T as Soapy>::RawSoa as RawSoa<T>>::Ref<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            None
        } else {
            let out = unsafe { self.raw.get_ref(self.start) };
            self.start += 1;
            Some(out)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end - self.start;
        (len, Some(len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T>
where
    T: Soapy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            None
        } else {
            self.end -= 1;
            Some(unsafe { self.raw.get_ref(self.end) })
        }
    }
}