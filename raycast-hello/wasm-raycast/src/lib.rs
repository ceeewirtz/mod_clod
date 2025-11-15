use wasm_bindgen::prelude::*;

const MAP_WIDTH: usize = 8;
const MAP_HEIGHT: usize = 8;
const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 480;

// Simple 8x8 map (1 = wall, 0 = empty, 2 = text marker)
static MAP: [[u8; MAP_WIDTH]; MAP_HEIGHT] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 2, 2, 2, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

#[wasm_bindgen]
pub struct RayCaster {
    pos_x: f64,
    pos_y: f64,
    dir_x: f64,
    dir_y: f64,
    plane_x: f64,
    plane_y: f64,
}

#[wasm_bindgen]
impl RayCaster {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RayCaster {
        RayCaster {
            pos_x: 4.5,
            pos_y: 4.5,
            dir_x: -1.0,
            dir_y: 0.0,
            plane_x: 0.0,
            plane_y: 0.66,
        }
    }

    pub fn rotate(&mut self, angle: f64) {
        let old_dir_x = self.dir_x;
        self.dir_x = self.dir_x * angle.cos() - self.dir_y * angle.sin();
        self.dir_y = old_dir_x * angle.sin() + self.dir_y * angle.cos();

        let old_plane_x = self.plane_x;
        self.plane_x = self.plane_x * angle.cos() - self.plane_y * angle.sin();
        self.plane_y = old_plane_x * angle.sin() + self.plane_y * angle.cos();
    }

    pub fn move_forward(&mut self, speed: f64) {
        let new_x = self.pos_x + self.dir_x * speed;
        let new_y = self.pos_y + self.dir_y * speed;

        if MAP[new_y as usize][new_x as usize] == 0 {
            self.pos_x = new_x;
            self.pos_y = new_y;
        }
    }

    pub fn render(&self) -> Vec<u8> {
        let mut buffer = vec![0u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4];

        for x in 0..SCREEN_WIDTH {
            let camera_x = 2.0 * x as f64 / SCREEN_WIDTH as f64 - 1.0;
            let ray_dir_x = self.dir_x + self.plane_x * camera_x;
            let ray_dir_y = self.dir_y + self.plane_y * camera_x;

            let mut map_x = self.pos_x as i32;
            let mut map_y = self.pos_y as i32;

            let delta_dist_x = if ray_dir_x == 0.0 { f64::INFINITY } else { (1.0 / ray_dir_x).abs() };
            let delta_dist_y = if ray_dir_y == 0.0 { f64::INFINITY } else { (1.0 / ray_dir_y).abs() };

            let step_x: i32;
            let step_y: i32;
            let mut side_dist_x: f64;
            let mut side_dist_y: f64;

            if ray_dir_x < 0.0 {
                step_x = -1;
                side_dist_x = (self.pos_x - map_x as f64) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist_x = (map_x as f64 + 1.0 - self.pos_x) * delta_dist_x;
            }

            if ray_dir_y < 0.0 {
                step_y = -1;
                side_dist_y = (self.pos_y - map_y as f64) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist_y = (map_y as f64 + 1.0 - self.pos_y) * delta_dist_y;
            }

            let mut hit = 0;
            let mut side = 0;

            while hit == 0 {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }

                if map_x >= 0 && map_x < MAP_WIDTH as i32 && map_y >= 0 && map_y < MAP_HEIGHT as i32 {
                    hit = MAP[map_y as usize][map_x as usize];
                } else {
                    hit = 1;
                }
            }

            let perp_wall_dist = if side == 0 {
                (map_x as f64 - self.pos_x + (1.0 - step_x as f64) / 2.0) / ray_dir_x
            } else {
                (map_y as f64 - self.pos_y + (1.0 - step_y as f64) / 2.0) / ray_dir_y
            };

            let line_height = (SCREEN_HEIGHT as f64 / perp_wall_dist) as i32;
            let draw_start = ((-line_height / 2 + SCREEN_HEIGHT as i32 / 2).max(0)) as usize;
            let draw_end = ((line_height / 2 + SCREEN_HEIGHT as i32 / 2).min(SCREEN_HEIGHT as i32 - 1)) as usize;

            // Choose color based on wall type
            let (mut r, mut g, mut b) = match hit {
                1 => (200, 100, 100),  // Regular wall - red
                2 => (100, 200, 255),  // "Hello World" marker - cyan
                _ => (100, 100, 100),  // Default
            };

            // Make horizontal walls darker
            if side == 1 {
                r = r / 2;
                g = g / 2;
                b = b / 2;
            }

            // Draw ceiling (dark blue)
            for y in 0..draw_start {
                let idx = (y * SCREEN_WIDTH + x) * 4;
                buffer[idx] = 20;
                buffer[idx + 1] = 20;
                buffer[idx + 2] = 40;
                buffer[idx + 3] = 255;
            }

            // Draw wall
            for y in draw_start..draw_end {
                let idx = (y * SCREEN_WIDTH + x) * 4;
                buffer[idx] = r;
                buffer[idx + 1] = g;
                buffer[idx + 2] = b;
                buffer[idx + 3] = 255;
            }

            // Draw floor (gray)
            for y in draw_end..SCREEN_HEIGHT {
                let idx = (y * SCREEN_WIDTH + x) * 4;
                buffer[idx] = 60;
                buffer[idx + 1] = 60;
                buffer[idx + 2] = 60;
                buffer[idx + 3] = 255;
            }
        }

        buffer
    }

    pub fn get_width(&self) -> usize {
        SCREEN_WIDTH
    }

    pub fn get_height(&self) -> usize {
        SCREEN_HEIGHT
    }
}
