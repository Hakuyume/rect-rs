use std::cmp::{Ordering, PartialOrd};
use std::ops::{Add, Div, Mul, Sub};

pub trait Rect<T> {
    fn top(&self) -> T;
    fn left(&self) -> T;
    fn bottom(&self) -> T;
    fn right(&self) -> T;

    fn height(&self) -> Option<T>
    where
        T: PartialOrd + Sub<Output = T>,
    {
        if self.top() < self.bottom() {
            Some(self.bottom() - self.top())
        } else {
            None
        }
    }

    fn width(&self) -> Option<T>
    where
        T: PartialOrd + Sub<Output = T>,
    {
        if self.left() < self.right() {
            Some(self.right() - self.left())
        } else {
            None
        }
    }

    fn area(&self) -> Option<T>
    where
        T: PartialOrd + Sub<Output = T> + Mul<Output = T>,
    {
        Some(self.height()? * self.width()?)
    }

    fn iou<Rhs>(&self, rhs: &Rhs) -> Option<T>
    where
        T: Clone
            + PartialOrd
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>,
        Rhs: Rect<T>,
    {
        let i = R(
            max(self.top(), rhs.top())?,
            max(self.left(), rhs.left())?,
            min(self.bottom(), rhs.bottom())?,
            min(self.right(), rhs.right())?,
        )
        .area()?;
        Some(i.clone() / (self.area()? + rhs.area()? - i))
    }
}

struct R<T>(T, T, T, T);
impl<T> Rect<T> for R<T>
where
    T: Clone,
{
    fn top(&self) -> T {
        self.0.clone()
    }
    fn left(&self) -> T {
        self.1.clone()
    }
    fn bottom(&self) -> T {
        self.2.clone()
    }
    fn right(&self) -> T {
        self.3.clone()
    }
}

fn min<T>(v1: T, v2: T) -> Option<T>
where
    T: PartialOrd,
{
    match v1.partial_cmp(&v2)? {
        Ordering::Greater => Some(v2),
        _ => Some(v1),
    }
}

fn max<T>(v1: T, v2: T) -> Option<T>
where
    T: PartialOrd,
{
    match v1.partial_cmp(&v2)? {
        Ordering::Less => Some(v2),
        _ => Some(v1),
    }
}

#[cfg(test)]
mod tests;
