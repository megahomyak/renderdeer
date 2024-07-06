use minifb::{Key, Window, WindowOptions};

#[derive(Clone, Copy)]
enum Block {
    Air,
    Solid,
}

impl Block {
    fn get_color(&self, _x: f32, _y: f32) -> (u8, u8, u8) {
        match self {
            Block::Air => unreachable!(),
            Block::Solid => (255, 255, 255),
        }
    }
}

#[derive(Clone, Copy)]
struct Vector3(f32, f32, f32);

impl Vector3 {
    fn normalize(&self) -> Vector3 {
        let length = (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt();
        Vector3(self.0 / length, self.1 / length, self.2 / length)
    }
}

struct Camera {
    position: Vector3,
    direction: Vector3,
}

impl Camera {
    fn new(position: Vector3, direction: Vector3) -> Self {
        Camera {
            position,
            direction,
        }
    }
}

type Grid = Vec<Vec<Vec<Block>>>;

fn render<const W: usize, const H: usize>(
    grid: Grid,
    camera: &Camera,
    fov_v: f32,
    fov_h: f32,
) -> [[(u8, u8, u8); W]; H] {
    let mut image = [[(0, 0, 0); W]; H];

    let Vector3(dx, dy, dz) = camera.direction;

    let fov_v_rad = fov_v.to_radians();
    let fov_h_rad = fov_h.to_radians();

    for y in 0..H {
        for x in 0..W {
            let px = (2.0 * (x as f32 + 0.5) / W as f32 - 1.0) * (fov_h_rad / 2.0).tan();
            let py = (2.0 * (y as f32 + 0.5) / H as f32 - 1.0) * (fov_v_rad / 2.0).tan();
            let ray_dir = Vector3(dx + px, dy + py, dz).normalize();

            if let Some((block, ix, iy)) = march_ray(&grid, &camera.position, ray_dir) {
                image[y][x] = block.get_color(ix, iy);
            }
        }
    }

    image
}

fn march_ray(grid: &Grid, origin: &Vector3, direction: Vector3) -> Option<(Block, f32, f32)> {
    let (x, y, z) = (origin.0, origin.1, origin.2);
    let (dx, dy, dz) = (direction.0, direction.1, direction.2);

    let mut gx = x.floor() as isize;
    let mut gy = y.floor() as isize;
    let mut gz = z.floor() as isize;

    let step_x = if dx > 0.0 { 1 } else { -1 };
    let step_y = if dy > 0.0 { 1 } else { -1 };
    let step_z = if dz > 0.0 { 1 } else { -1 };

    let mut t_max_x = ((gx + (if step_x > 0 { 1 } else { 0 })) as f32 - x) / dx;
    let mut t_max_y = ((gy + (if step_y > 0 { 1 } else { 0 })) as f32 - y) / dy;
    let mut t_max_z = ((gz + (if step_z > 0 { 1 } else { 0 })) as f32 - z) / dz;

    let t_delta_x = (1.0 / dx).abs();
    let t_delta_y = (1.0 / dy).abs();
    let t_delta_z = (1.0 / dz).abs();

    while in_bounds(gx, gy, gz, grid) {
        if let Block::Solid = grid[gx as usize][gy as usize][gz as usize] {
            let ix = x.fract();
            let iy = y.fract();
            return Some((grid[gx as usize][gy as usize][gz as usize], ix, iy));
        }

        if t_max_x < t_max_y {
            if t_max_x < t_max_z {
                gx += step_x;
                t_max_x += t_delta_x;
            } else {
                gz += step_z;
                t_max_z += t_delta_z;
            }
        } else {
            if t_max_y < t_max_z {
                gy += step_y;
                t_max_y += t_delta_y;
            } else {
                gz += step_z;
                t_max_z += t_delta_z;
            }
        }
    }

    None
}

fn in_bounds(gx: isize, gy: isize, gz: isize, grid: &Grid) -> bool {
    gx >= 0
        && gx < grid.len() as isize
        && gy >= 0
        && gy < grid[0].len() as isize
        && gz >= 0
        && gz < grid[0][0].len() as isize
}

const MOVE_SPEED: f32 = 0.1;
const TURN_SPEED: f32 = 0.1;

fn main() {
    const W: usize = 640;
    const H: usize = 480;

    let grid = vec![
        vec![
            vec![Block::Air, Block::Solid, Block::Air],
            vec![Block::Solid, Block::Solid, Block::Solid],
            vec![Block::Air, Block::Solid, Block::Air],
        ],
        vec![
            vec![Block::Air, Block::Solid, Block::Air],
            vec![Block::Air, Block::Solid, Block::Air],
            vec![Block::Air, Block::Solid, Block::Air],
        ],
        vec![
            vec![Block::Solid, Block::Air, Block::Solid],
            vec![Block::Air, Block::Air, Block::Air],
            vec![Block::Solid, Block::Air, Block::Solid],
        ],
    ];

    let mut camera_position = Vector3(1.5, 1.5, 1.5);
    let mut camera_direction = Vector3(0.0, 0.0, 0.0);
    let camera = Camera::new(camera_position, camera_direction);

    let fov_v = 90.0;
    let fov_h = 120.0;

    let mut window = Window::new(
        "3D Ray Marching",
        W,
        H,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_input(&mut camera_position, &mut camera_direction, &window);

        let image = render::<W, H>(grid.clone(), &camera, fov_v, fov_h);

        let buffer: Vec<u32> = image.iter()
            .flat_map(|row| row.iter().map(|&(r, g, b)| {
                ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
            }))
            .collect();

        window.update_with_buffer(&buffer, W, H).unwrap();
    }
}

fn handle_input(position: &mut Vector3, direction: &mut Vector3, window: &Window) {
    let Vector3(px, py, pz) = *position;
    let Vector3(dx, dy, dz) = *direction;

    if window.is_key_down(Key::W) {
        *position = Vector3(px + dx * MOVE_SPEED, py + dy * MOVE_SPEED, pz + dz * MOVE_SPEED);
    }
    if window.is_key_down(Key::S) {
        *position = Vector3(px - dx * MOVE_SPEED, py - dy * MOVE_SPEED, pz - dz * MOVE_SPEED);
    }
    if window.is_key_down(Key::A) {
        let left = Vector3(-dz, 0.0, dx).normalize();
        *position = Vector3(px + left.0 * MOVE_SPEED, py + left.1 * MOVE_SPEED, pz + left.2 * MOVE_SPEED);
    }
    if window.is_key_down(Key::D) {
        let right = Vector3(dz, 0.0, -dx).normalize();
        *position = Vector3(px + right.0 * MOVE_SPEED, py + right.1 * MOVE_SPEED, pz + right.2 * MOVE_SPEED);
    }

    if window.is_key_down(Key::Left) {
        let rotation = rotate_y(direction, -TURN_SPEED);
        *direction = rotation.normalize();
    }
    if window.is_key_down(Key::Right) {
        let rotation = rotate_y(direction, TURN_SPEED);
        *direction = rotation.normalize();
    }
    if window.is_key_down(Key::Up) {
        let rotation = rotate_x(direction, -TURN_SPEED);
        *direction = rotation.normalize();
    }
    if window.is_key_down(Key::Down) {
        let rotation = rotate_x(direction, TURN_SPEED);
        *direction = rotation.normalize();
    }
}

fn rotate_y(vector: &Vector3, angle: f32) -> Vector3 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();
    Vector3(
        vector.0 * cos_angle - vector.2 * sin_angle,
        vector.1,
        vector.0 * sin_angle + vector.2 * cos_angle,
    )
}

fn rotate_x(vector: &Vector3, angle: f32) -> Vector3 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();
    Vector3(
        vector.0,
        vector.1 * cos_angle - vector.2 * sin_angle,
        vector.1 * sin_angle + vector.2 * cos_angle,
    )
}
