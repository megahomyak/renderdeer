use std::rc::Rc;

struct Pos {
    x: f64,
    y: f64,
    z: f64,
}

struct GridPos {
    x: isize,
    y: isize,
    z: isize,
}

struct Camera {
    pos: Pos,
    direction: Pos,
}

struct RGBA(u8, u8, u8, u8);
struct RGB(u8, u8, u8);

type UnitInterval = f64;

trait Img {
    fn get_pixel(&self, x: UnitInterval, y: UnitInterval) -> RGBA;
}

trait Block {
    fn btm(&self) -> Rc<dyn Img>;
    fn top(&self) -> Rc<dyn Img>;
    fn left(&self) -> Rc<dyn Img>;
    fn right(&self) -> Rc<dyn Img>;
    fn back(&self) -> Rc<dyn Img>;
    fn front(&self) -> Rc<dyn Img>;
}

struct Grid {
    blocks: Vec<Vec<Option<Rc<dyn Block>>>>,
}

fn render<const W: usize, const H: usize>(vert_fov: f64, hor_fov: f64) -> [[RGB; W]; H] {
    use std::array::from_fn as array;
    array(|height| array(|width| {
        let pixel = RGB(0, 0, 0);
    }))
}
