mod finite_f64 {
    pub struct FiniteF64(f64);

    impl From<FiniteF64> for f64 {
        fn from(value: FiniteF64) -> Self {
            value.0
        }
    }

    impl FiniteF64 {
        pub fn new(v: f64) -> Option<FiniteF64> {
            if v.is_finite() {
                Some(Self(v))
            } else {
                None
            }
        }
    }
}
pub use finite_f64::FiniteF64;

mod position {
    use super::*;
    pub struct Position(FiniteF64);
    impl From<Position> for f64 {
        fn from(value: Position) -> Self {
            value.0.into()
        }
    }
    impl From<Position> for FiniteF64 {
        fn from(value: Position) -> Self {
            value.0
        }
    }
    impl Position {
        pub fn new(v: FiniteF64) -> Self {
            Self(v)
        }
    }
}
pub use position::Position;

mod distance {
    use super::*;
    pub struct Distance(Unsigned<FiniteF64>);
    impl From<Distance> for Unsigned {
        fn from(value: Distance) -> Self {
            value.0
        }
    }
    impl Distance {
        pub fn read(&self) -> f64 {
            self.0.read()
        }
        pub fn new(v: Unsigned<FiniteF64>) -> Self {
            Self(v)
        }
    }
}
pub use distance::Distance;

mod unit_interval {
    use super::*;

    pub struct UnitInterval(FiniteF64);

    impl UnitInterval {
        pub fn new(v: FiniteF64) -> Option<Self> {
            if v.read() >= 0.0 && v.read() <= 1.0 {
                Some(Self(v))
            } else {
                None
            }
        }

        pub fn read(&self) -> f64 {
            self.0.read()
        }
    }
}
pub use unit_interval::UnitInterval;

mod unsigned {
    use super::*;

    pub struct Unsigned<N>(N);

    impl Unsigned<FiniteF64> {
        pub fn new(v: FiniteF64) -> Option<Self> {
            if v.read() >= 0.0 {
                Some(Self(v))
            } else {
                None
            }
        }

        pub fn read(&self) -> f64 {
            self.0.read()
        }
    }
}
pub use unsigned::Unsigned;

mod angle {
    use super::*;

    pub struct Angle(UnitInterval);

    impl Angle {
        pub fn read(&self) -> f64 {
            self.0.read()
        }

        pub fn turn_left(&mut self, amount: Unsigned<FiniteF64>) {
            self.0 =
                UnitInterval::new(FiniteF64::new((self.0.read() + amount.read()).fract()).unwrap())
                    .unwrap();
        }

        pub fn turn_right(&mut self, amount: Unsigned<FiniteF64>) {
            self.0 = UnitInterval::new(
                FiniteF64::new((self.0.read() - amount.read()).fract().abs()).unwrap(),
            )
            .unwrap();
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
        let x = (self.x.read() - other.x.read()).abs();
        let y = (self.y.read() - other.y.read()).abs();
        let z = (self.z.read() - other.z.read()).abs();

        Distance::new(Unsigned::new(FiniteF64::new(x.hypot(y).hypot(z)).unwrap()).unwrap())
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
    fn image(&self, angle_difference: Angle) -> Self::Image;
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

impl RenderPixel {
    pub fn overlay_with(&self, object_pixel: ObjectPixel) -> RenderPixel {
        macro_rules! mix {
            ($color:ident) => {
                UnitInterval::new(
                    FiniteF64::new(
                        (object_pixel.$color.read() * object_pixel.opacity.read()
                            + self.$color.read() * (1.0 - object_pixel.opacity.read()))
                        .clamp(0.0, 1.0),
                    )
                    .unwrap(),
                )
                .unwrap()
            };
        }
        RenderPixel {
            red: mix!(red),
            green: mix!(green),
            blue: mix!(blue),
        }
    }
}

pub struct RenderOutput<const WIDTH: usize, const HEIGHT: usize> {
    pub image: [[RenderPixel; WIDTH]; HEIGHT],
}

fn rotate(x: FiniteF64, y: FiniteF64, angle: Angle) -> (FiniteF64, FiniteF64) {
    let pi_angle = angle.read() * 2.0 * std::f64::consts::PI;
    let new_x = x.read() * pi_angle.cos() - y.read() * pi_angle.sin();
    let new_y = x.read() * pi_angle.sin() + y.read() * pi_angle.cos();
    (
        FiniteF64::new(new_x).unwrap(),
        FiniteF64::new(new_y).unwrap(),
    )
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
                    x: UnitInterval::new(FiniteF64::new(width as f64 / WIDTH as f64).unwrap())
                        .unwrap(),
                    y: UnitInterval::new(FiniteF64::new(height as f64 / HEIGHT as f64).unwrap())
                        .unwrap(),
                }))
            })
        });
        let mut objects = objects
            .map(|obj| (obj.position().distance(&self.position), obj))
            .collect::<Vec<_>>();
        objects.sort_by(|(da, _), (db, _)| da.read().total_cmp(&db.read()));
        for (distance, object) in objects {
            rotate(distance.read())
        }
        image
    }
}
