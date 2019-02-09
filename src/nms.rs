use crate::Rect;
use std::cmp::PartialOrd;
use std::ops::{Add, Div, Mul, Sub};

pub fn non_maximum_suppression<R, T>(rects: &mut Vec<R>, thresh: T)
where
    R: Rect<T>,
    T: Clone + PartialOrd + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    retain_with_prevs(rects, |r, prevs| {
        prevs
            .iter()
            .all(|p| r.iou(p).map_or(true, |iou| iou < thresh))
    });
}

pub fn non_maximum_suppression_by_key<I, F, R, T>(items: &mut Vec<I>, mut f: F, thresh: T)
where
    F: FnMut(&I) -> R,
    R: Rect<T>,
    T: Clone + PartialOrd + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    retain_with_prevs(items, |i, prevs| {
        prevs
            .iter()
            .all(|p| f(i).iou(&f(p)).map_or(true, |iou| iou < thresh))
    });
}

fn retain_with_prevs<T, F>(v: &mut Vec<T>, mut f: F)
where
    F: FnMut(&T, &[T]) -> bool,
{
    let len = (0..v.len()).fold(0, |l, r| {
        v.swap(l, r);
        l + if f(&v[l], &v[0..l]) { 1 } else { 0 }
    });
    v.truncate(len);
}

#[cfg(test)]
mod tests;
