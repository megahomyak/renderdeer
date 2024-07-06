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

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

type UnitInterval = f64;

trait Img {
    fn get_pixel(&self, x: UnitInterval, y: UnitInterval) -> Option<Pixel>;
}

trait Block<'a> {
    type Img: Img + 'a;

    fn btm<'a, I: Img + 'a>(&'a self) -> I;
    fn top<'a, I: Img + 'a>(&'a self) -> I;
    fn left<'a, I: Img + 'a>(&'a self) -> I;
    fn right<'a, I: Img + 'a>(&'a self) -> I;
    fn back<'a, I: Img + 'a>(&'a self) -> I;
    fn front<'a, I: Img + 'a>(&'a self) -> I;
}

struct Grid<'a> {
    blocks: Vec<Vec<Option<&'a dyn Block>>>,
}

fn render() {}
