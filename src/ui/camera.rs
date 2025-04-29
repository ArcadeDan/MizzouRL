// camera.rs
use bracket_lib::prelude::Point;

pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub display_width: i32,
    pub display_height: i32,
}

impl Camera {
    pub fn new(x: i32, y: i32, width: i32, height: i32, display_width: i32, display_height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            display_width,
            display_height,
        }
    }

    pub fn center_on_point(&mut self, point: Point) {
        self.x = point.x - self.width / 2;
        self.y = point.y - self.height / 2;

        // Make sure the camera doesn't go out of bounds
        if self.x < 0 {
            self.x = 0;
        }
        if self.y < 0 {
            self.y = 0;
        }
        if self.x + self.width > self.display_width {
            self.x = self.display_width - self.width;
        }
        if self.y + self.height > self.display_height {
            self.y = self.display_height - self.height;
        }
    }

    // Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, point: Point) -> Option<(i32, i32)> {
        let screen_x = point.x - self.x;
        let screen_y = point.y - self.y;

        // Check if the point is within the viewport
        if screen_x >= 0 && screen_x < self.width && screen_y >= 0 && screen_y < self.height {
            Some((screen_x, screen_y))
        } else {
            None
        }
    }

    // Convert screen coordinates to world coordinates
    pub fn screen_to_world(&self, screen_x: i32, screen_y: i32) -> Point {
        Point::new(screen_x + self.x, screen_y + self.y)
    }

    // Check if a world position is in the camera's view
    pub fn in_viewport(&self, point: Point) -> bool {
        point.x >= self.x && point.x < self.x + self.width &&
            point.y >= self.y && point.y < self.y + self.height
    }
}