#![feature(test)]
extern crate test;

use rand::{distributions::Uniform, thread_rng, Rng};
use rect::{non_maximum_suppression, non_maximum_suppression_by_key, Rect};
use test::Bencher;

#[bench]
fn bench_non_maximum_suppression(b: &mut Bencher) {
    let rects = setup();
    b.iter(|| {
        let mut rects = rects.clone();
        non_maximum_suppression(&mut rects, 0.5);
        rects
    });
}

#[bench]
fn bench_non_maximum_suppression_by_key(b: &mut Bencher) {
    let rects = setup();
    let indices: Vec<_> = (0..rects.len()).collect();
    b.iter(|| {
        let mut indices = indices.clone();
        non_maximum_suppression_by_key(&mut indices, |&i| &rects[i], 0.5);
        indices
    });
}

fn setup() -> Vec<R> {
    let mut rng = thread_rng();
    (0..10000)
        .map(|_| {
            let canvas = (480., 640.);
            let edge = Uniform::new(10., 400.);

            let height = rng.sample(edge);
            let width = rng.sample(edge);
            let top = rng.sample(Uniform::new(0., canvas.0 - height));
            let left = rng.sample(Uniform::new(0., canvas.1 - width));
            R(top, left, top + height, left + width)
        })
        .collect()
}

#[derive(Clone)]
struct R(f32, f32, f32, f32);
impl Rect<f32> for R {
    fn top(&self) -> f32 {
        self.0
    }

    fn left(&self) -> f32 {
        self.1
    }

    fn bottom(&self) -> f32 {
        self.2
    }

    fn right(&self) -> f32 {
        self.3
    }
}
impl Rect<f32> for &R {
    fn top(&self) -> f32 {
        self.0
    }

    fn left(&self) -> f32 {
        self.1
    }

    fn bottom(&self) -> f32 {
        self.2
    }

    fn right(&self) -> f32 {
        self.3
    }
}
