pub struct Position(pub f64);
pub struct Distance(pub Unsigned<f64>);

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
    use super::*;

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

impl ObjectPosition {
    pub fn distance(&self, other: &Self) -> Distance {
        let x = (self.x.0 - other.x.0).abs();
        let y = (self.y.0 - other.y.0).abs();
        let z = (self.z.0 - other.z.0).abs();

        Distance(Unsigned::new(x.hypot(y).hypot(z)).unwrap())
    }
}

pub struct FromTopLeft<T>(pub T);
pub struct PixelPosition {
    pub x: UnitInterval,
    pub y: UnitInterval,
}

pub trait Image {
    type Pixel;

    fn pixel(&self, position: FromTopLeft<PixelPosition>) -> Self::Pixel;
}

pub trait Object<'a> {
    type Image: Image<Pixel = ObjectPixel> + 'a;

    fn position(&self) -> ObjectPosition;
    fn height(&self) -> Distance;
    fn width(&self) -> Distance;
    fn tilt(&self) -> Angle;
    fn image(&self) -> Self::Image;
}

pub struct CameraAngle {
    pub x: Angle,
    pub y: Angle,
}

pub struct Camera {
    pub position: ObjectPosition,
    pub x_bias: Position,
    pub y_bias: Position,
    pub width: Distance,
    pub height: Distance,
    pub angle: CameraAngle,
    pub tilt: Angle,
}

pub struct RenderPixel {
    pub red: UnitInterval,
    pub green: UnitInterval,
    pub blue: UnitInterval,
}

pub struct RenderOutput<const WIDTH: usize, const HEIGHT: usize> {
    pub image: [[RenderPixel; WIDTH]; HEIGHT],
}

impl Camera {
    pub fn render<'a, const WIDTH: usize, const HEIGHT: usize, Obj, ObjIt, Bg>(
        &self,
        background: Bg,
        objects: ObjIt,
    ) -> [[RenderPixel; WIDTH]; HEIGHT]
    where
        Obj: Object<'a>,
        Bg: Image<Pixel = RenderPixel>,
        ObjIt: Iterator<Item = Obj>,
    {
        use std::array::from_fn as array;
        let image: [[RenderPixel; WIDTH]; HEIGHT] = array(|height| {
            array(|width| {
                background.pixel(FromTopLeft(PixelPosition {
                    x: UnitInterval::new(width as f64 / WIDTH as f64).unwrap(),
                    y: UnitInterval::new(height as f64 / HEIGHT as f64).unwrap(),
                }))
            })
        });
        let mut objects = objects.collect::<Vec<_>>();
        let dist = |obj: &Obj| *obj.position().distance(&self.position).0.read();
        objects.sort_by(|a, b| dist(a).total_cmp(&dist(b)));
        for object in objects {}
        image
    }
}
