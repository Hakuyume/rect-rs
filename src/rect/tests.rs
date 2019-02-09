use super::*;

#[derive(Clone, Copy, Debug)]
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

#[test]
fn test_area() {
    assert_eq!(R(0., 0., 4., 4.).area(), Some(16.));
    assert_eq!(R(4., 0., 0., 4.).area(), None);
    assert_eq!(R(4., 4., 0., 0.).area(), None);
}

#[test]
fn test_iou() {
    assert_eq!(R(0., 0., 4., 4.).iou(&R(1., 1., 5., 5.)), Some(9. / 23.));
    assert_eq!(R(1., 1., 4., 4.).iou(&R(0., 0., 5., 5.)), Some(9. / 25.));
    assert_eq!(R(0., 0., 4., 4.).iou(&R(4., 0., 8., 4.)), None);
}
