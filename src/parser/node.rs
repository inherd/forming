use std::{cmp, fmt};

#[derive(Copy, Clone)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

impl Location {
    pub fn new(row: usize, column: usize) -> Location {
        Location {
            row,
            column
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn none() -> Location {
        Location {
            row: 0,
            column: 0,
        }
    }

    pub fn is_none(&self) -> bool {
        self.row == 0 && self.column == 0
    }
}

impl cmp::PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        (self.row == other.row && self.column == other.column) || self.is_none() || other.is_none()
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {} column {}", self.row, self.column)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if !self.is_none() {
            write!(fmt, "{}:{}", self.row, self.column)
        } else {
            write!(fmt, "…")
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node<T> {
    pub node: T,
    pub location: Location
}

impl<T> Node<T> {
    pub fn new(node: T, location: Location) -> Node<T> {
        Node {
            node,
            location
        }
    }
}
