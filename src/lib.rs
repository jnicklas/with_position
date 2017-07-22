use std::iter::Peekable;
use std::cell::Cell;

// index comparisons
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Position {
    First,
    Middle,
    Last,
    Only,
}

impl Position {
    pub fn is_first(self) -> bool {
        self == Position::First || self == Position::Only
    }

    pub fn is_last(self) -> bool {
        self == Position::Last || self == Position::Only
    }

    pub fn is_only(self) -> bool {
        self == Position::Only
    }
}

pub struct PositionIterator<T> where T: Iterator {
    iter: Peekable<T>,
    did_iter: Cell<bool>,
}

impl<T> Iterator for PositionIterator<T> where T: Iterator {
    type Item = (Position, T::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let is_first = self.did_iter.get();
        self.did_iter.set(true);

        let next = match self.iter.next() {
            Some(item) => item,
            None => return None,
        };

        if is_first {
            match self.iter.peek() {
                Some(_) => Some((Position::Middle, next)),
                None => Some((Position::Last, next)),
            }
        } else {
            match self.iter.peek() {
                Some(_) => Some((Position::First, next)),
                None => Some((Position::Only, next)),
            }
        }
    }
}

pub trait WithPosition where {
    type Iterator: Iterator;

    fn with_position(self) -> PositionIterator<Self::Iterator>;
}

impl<T> WithPosition for T where T: Iterator {
    type Iterator = T;

    fn with_position(self) -> PositionIterator<T> {
        PositionIterator { iter: self.peekable(), did_iter: Cell::new(false) }
    }
}

#[cfg(test)]
mod tests {
    use super::{WithPosition, Position};

    #[test]
    fn it_marks_first_middle_and_last_position() {
        let result: Vec<_> = vec![1,2,3,4].into_iter().with_position().collect();

        assert_eq!(result[0], (Position::First, 1));
        assert_eq!(result[1], (Position::Middle, 2));
        assert_eq!(result[2], (Position::Middle, 3));
        assert_eq!(result[3], (Position::Last, 4));
    }

    #[test]
    fn it_marks_first_and_last_position() {
        let result: Vec<_> = vec![1,4].into_iter().with_position().collect();

        assert_eq!(result[0], (Position::First, 1));
        assert_eq!(result[1], (Position::Last, 4));
    }

    #[test]
    fn it_marks_only_position() {
        let result: Vec<_> = vec![2].into_iter().with_position().collect();

        assert_eq!(result[0], (Position::Only, 2));
    }

    #[test]
    fn it_has_boolean_methods_on_position() {
        assert_eq!(Position::First.is_first(), true);
        assert_eq!(Position::Middle.is_first(), false);
        assert_eq!(Position::Last.is_first(), false);
        assert_eq!(Position::Only.is_first(), true);

        assert_eq!(Position::First.is_last(), false);
        assert_eq!(Position::Middle.is_last(), false);
        assert_eq!(Position::Last.is_last(), true);
        assert_eq!(Position::Only.is_last(), true);

        assert_eq!(Position::First.is_only(), false);
        assert_eq!(Position::Middle.is_only(), false);
        assert_eq!(Position::Last.is_only(), false);
        assert_eq!(Position::Only.is_only(), true);
    }
}
