pub struct UnitInterval(pub f64);
pub struct Position(pub f64);
pub struct UnsignedF64(pub f64);
pub struct Distance(pub f64);
pub struct Radians(pub f64);

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

        Distance(x.hypot(y).hypot(z))
    }
}

pub struct PixelPositionFromTopLeft {
    pub x: UnitInterval,
    pub y: UnitInterval,
}

pub trait Image {
    type Pixel;

    fn pixel(&self, position: PixelPositionFromTopLeft) -> Self::Pixel;
}

pub struct Object<I: Image<Pixel = ObjectPixel>> {
    pub position: ObjectPosition,
    pub height: Distance,
    pub width: Distance,
    pub image: I,
}

pub struct CameraAngle {
    pub x: Radians,
    pub y: Radians,
}

pub struct Camera {
    pub position: ObjectPosition,
    pub x_bias: Position,
    pub y_bias: Position,
    pub width: Distance,
    pub height: Distance,
    pub angle: CameraAngle,
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
                UnitInterval(
                    object_pixel.$color.0 * object_pixel.opacity.0
                        + self.$color.0 * (1.0 - object_pixel.opacity.0),
                )
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

fn rotate(x: Position, y: Position, angle: Radians) -> (Position, Position) {
    let new_x = x.0 * angle.0.cos() - y.0 * angle.0.sin();
    let new_y = x.0 * angle.0.sin() + y.0 * angle.0.cos();
    (Position(new_x), Position(new_y))
}

impl Camera {
    pub fn render<'a, const WIDTH: usize, const HEIGHT: usize, ObjImg, ObjIt, Bg>(
        &self,
        background: Bg,
        objects: ObjIt,
    ) -> [[RenderPixel; WIDTH]; HEIGHT]
    where
        ObjImg: Image<Pixel = ObjectPixel>,
        Bg: Image<Pixel = RenderPixel>,
        ObjIt: Iterator<Item = Object<ObjImg>>,
    {
        use std::array::from_fn as array;
        let mut objects = objects
            .map(|obj| (obj.position.distance(&self.position), obj))
            .collect::<Vec<_>>();
        objects.sort_by(|(da, _), (db, _)| da.0.total_cmp(&db.0));
        array(|height| {
            array(|width| {
                let pixel = background.pixel(PixelPositionFromTopLeft {
                    x: UnitInterval(width as f64 / WIDTH as f64),
                    y: UnitInterval(height as f64 / HEIGHT as f64),
                });
                for (distance, object) in &objects {
                    todo!();
                    pixel = pixel.overlay_with();
                }
                pixel
            })
        })
    }
}
