pub struct Position(pub isize);
pub struct Size(pub usize);

mod unit_interval {
    #[derive(Clone, Copy)]
    pub struct UnitInterval(f64);

    impl UnitInterval {
        pub fn new(v: f64) -> Option<Self> {
            if v >= 0.0 && v <= 1.0 {
                Some(Self(v))
            } else {
                None
            }
        }

        pub fn read(&self) -> f64 {
            self.0
        }
    }
}
pub use unit_interval::UnitInterval;

mod positive {
    pub struct Unsigned<N>(N);

    impl<N> Unsigned<N> {
        pub fn read(&self) -> &N {
            &self.0
        }
    }

    impl Unsigned<f64> {
        pub fn new(v: f64) -> Option<Self> {
            if v >= 0.0 {
                Some(Self(v))
            } else {
                None
            }
        }
    }
}
pub use positive::Unsigned;

mod angle {
    use super::{UnitInterval, Unsigned};

    pub struct Angle(UnitInterval);

    impl Angle {
        pub fn read(&self) -> f64 {
            self.0.read()
        }

        pub fn turn_left(&mut self, amount: Unsigned<f64>) {
            self.0 = UnitInterval::new((self.0.read() + amount.read()).fract()).unwrap();
        }

        pub fn turn_right(&mut self, amount: Unsigned<f64>) {
            self.0 = UnitInterval::new((self.0.read() - amount.read()).fract()).unwrap();
        }
    }
}
pub use angle::Angle;

#[derive(Clone, Copy)]
pub struct ObjectPixel {
    pub red: UnitInterval,
    pub green: UnitInterval,
    pub blue: UnitInterval,
    pub opacity: UnitInterval,
}

pub struct ObjectPosition {
    pub x: Position,
    pub y: Position,
    pub z: Position,
}

pub struct FromTopLeft<T>(pub T);
pub struct PixelPosition {
    pub x: UnitInterval,
    pub y: UnitInterval,
}

pub trait Image {
    fn height(&self) -> Size;
    fn width(&self) -> Size;
    fn get_pixel(&self, position: FromTopLeft<PixelPosition>) -> ObjectPixel;
}

pub trait Object {
    type Image: Image;
    type Positions: Iterator<Item = ObjectPosition>;

    fn positions(&self) -> Self::Positions;
    fn image(&self, camera_position: ObjectPosition) -> Self::Image;
}

pub struct Camera {
    pub position: ObjectPosition,
    pub x_bias: Position,
    pub y_bias: Position,
    pub width: Size,
    pub height: Size,
    pub angle: UnitInterval,
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
    pub fn render<O: Object>(&self, object: &O) -> RenderOutput<Width, Height> {
        let position = object.position();
        let image = object.image(self.position);

    }
}
