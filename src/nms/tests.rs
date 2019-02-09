use super::*;

#[test]
fn test_non_maximum_suppression() {
    let rects = vec![
        R(0., 0., 4., 4.),
        R(1., 1., 5., 5.), // 9/23
        R(2., 1., 6., 5.), // 6/26, 12/20
        R(4., 0., 8., 4.), // N/A, 3/29, 6/26
    ];

    for &(thresh, indices) in &[
        (1., &[0, 1, 2, 3][..]),
        (0.5, &[0, 1, 3]),
        (0.3, &[0, 2, 3]),
        (0.2, &[0, 3]),
        (0., &[0, 3]),
    ] {
        let expected: Vec<_> = indices.iter().map(|&i| rects[i]).collect();
        let mut rects = rects.clone();
        non_maximum_suppression(&mut rects, thresh);
        assert_eq!(rects, expected);
    }
}

#[test]
fn test_non_maximum_suppression_by_key() {
    let rects = vec![
        R(0., 0., 4., 4.),
        R(1., 1., 5., 5.), // 9/23
        R(2., 1., 6., 5.), // 6/26, 12/20
        R(4., 0., 8., 4.), // N/A, 3/29, 6/26
    ];

    for &(thresh, expected) in &[
        (1., &[0, 1, 2, 3][..]),
        (0.5, &[0, 1, 3]),
        (0.3, &[0, 2, 3]),
        (0.2, &[0, 3]),
        (0., &[0, 3]),
    ] {
        let mut indices = (0..rects.len()).collect();
        non_maximum_suppression_by_key(&mut indices, |&i| &rects[i], thresh);
        assert_eq!(indices, expected);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
