mod unit_interval {
    #[derive(Clone, Copy)]
    pub struct UnitInterval<F=f64>(F);

    impl<F> UnitInterval<F> {
        pub fn read(&self) -> &F {
            &self.0
        }
    }

    trait ZeroOne {
        fn zero() -> Self;
        fn one() -> Self;
    }

    impl ZeroOne for f64 {
        fn one() -> Self {
            1.0
        }

        fn zero() -> Self {
            0.0
        }
    }

    impl<F: PartialOrd + ZeroOne> UnitInterval<F> {
        pub fn new(v: F) -> Option<Self> {
            if v >= F::zero() && v <= F::one() {
                Some(Self(v))
            } else {
                None
            }
        }
    }
}
pub use unit_interval::UnitInterval;

mod positive {
    pub struct Positive<N>(N);

    impl<N> Positive<N> {
        pub fn read(&self) -> &N {
            &self.0
        }
    }
}

mod angle {
    use super::UnitInterval;

    pub struct Angle(UnitInterval);

    impl Angle {
        pub fn turn_left(&mut self, amount: f64) {
            (self.0.read() + amount).fract()
        }

        pub fn turn_right(&mut self, amount: f64) {

        }
    }
}

#[derive(Clone, Copy)]
pub struct ObjectPixel {
    pub red: UnitInterval,
    pub green: UnitInterval,
    pub blue: UnitInterval,
    pub opacity: UnitInterval,
}

pub struct ObjectPosition {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

pub struct FromTopLeft<T>(pub T);
pub struct PixelPosition {
    pub x: UnitInterval,
    pub y: UnitInterval,
}

pub trait Image {
    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn get_pixel(&self, position: FromTopLeft<PixelPosition>) -> ObjectPixel;
}

pub trait Object {
    type Image: Image;

    fn render(&self, camera_position: ObjectPosition) -> Self::Image;
}

pub struct Camera<const Width: usize, const Height: usize> {
    pub position: ObjectPosition,
    pub x_bias: isize,
    pub y_bias: isize,
}

#[derive(Clone, Copy)]
pub struct RenderPixel {
    pub red: UnitInterval,
    pub green: UnitInterval,
    pub blue: UnitInterval,
}

pub struct RenderOutput<const Width: usize, const Height: usize> {
    pub image: [[RenderPixel; Width]; Height],
}

impl<const Width: usize, const Height: usize> Camera<Width, Height> {
    pub fn render<O: Object>(&self, angle: UnitInterval, object: O) -> RenderOutput<Width, Height> {

    }
}
