struct Pos {
    x: f64,
    y: f64,
    z: f64,
}

struct RGB(u8, u8, u8);

type UnitInterval = f64;

enum Block {
    Air,
    Solid,
}

struct Grid {
    blocks: Vec<Vec<Block>>,
}

struct Ray {
    origin: Pos,
    direction: Pos,
}

fn render<const W: usize, const H: usize>(camera: Ray, vert_fov: f64, hor_fov: f64) -> [[RGB; W]; H] {
    use std::array::from_fn as array;
    array(|height| array(|width| {
        let pixel = RGB(0, 0, 0);
    }))
}
