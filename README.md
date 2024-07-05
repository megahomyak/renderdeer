# What the fuck will this be?

This is going to be a renderer that's suited for 2D-3D cartoons. Everything is a dot with a texture that looks at the camera, the camera is a plane that does immediately from the observing dot. Just writing these things for myself, if you understand nothing, wait

## Notes

Every rendered object gets the position and the rotation of the camera object AND THAT'S IT. BUUUUT! Since the camera object looks at everything and sees everything as images that are pointed directly at the camera object (everything is flat to the camera object), no rotation is necessary. And also, since the camera object kinda does not have perspective and there's just a 2D window slice of the world that it can see, it's probably gonna be wise to just pass the distance from the camera to the object. But if I want to make this thing 2D too, it's probably going to be better to put things like camera distance into the objects themselves. However: if we're moving the camera to some other position, it's gonna be cool to have the objects rotate according to this new position, which is why the 3D coords are probably needed

```
pub type Size = usize;

mod unit_interval {
    pub struct UnitInterval(f64);

    impl UnitInterval {
        pub fn new(v: f64) -> Option<Self> {
            if v >= 0 && v <= 1 {
                Some(Self(v))
            } else {
                None
            }
        }

        pub fn read(&self) -> &f64 {
            &self.0
        }
    }
}
pub use unit_interval::UnitInterval;

pub struct Pixel {
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
    fn height(&self) -> Size;
    fn width(&self) -> Size;
    fn get_pixel(&self, position: FromTopLeft<PixelPosition>) -> Pixel;
}

pub trait Object {
    type Image;

    fn render(&self, camera_position: ObjectPosition) -> Self::Image;
}
```

Daym ive just wrote a bunch of code to put my thoughts in

Programmer's curse
