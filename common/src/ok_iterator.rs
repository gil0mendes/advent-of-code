use anyhow::{format_err, Error, Result};

pub trait OkIterator<T, E>
where
    Self: Sized + Iterator<Item = Result<T, E>>,
    E: Into<Error>,
{
    // Like the (unstable) `try_collect` method but only for results,
    // since it's the only thing I consider here.
    #[inline]
    fn ok_collect<B>(&mut self) -> Result<B>
    where
        B: FromIterator<T>,
    {
        self.collect::<Result<_, _>>().map_err(Into::into)
    }

    /// Let's make some shortcuts to usual types...
    #[inline]
    fn ok_collect_vec(&mut self) -> Result<Vec<T>> {
        self.ok_collect()
    }

    #[inline]
    fn ok_collect_array<const N: usize>(&mut self) -> Result<[T; N]> {
        self.ok_collect_vec()?
            .try_into()
            .map_err(|err: Vec<_>| format_err!("Not {} long but {}", N, err.len()))
    }
}

impl<T, E, It> OkIterator<T, E> for It
where
    It: Sized + Iterator<Item = Result<T, E>>,
    E: Into<Error>,
{
}
