use macroquad::prelude::*;

const RECT_SIZE: f32 = 20.0;
const RECT_SPEED: f32 = 100.0;
const COLLISION_PUSH: f32 = 5.0;

#[derive(Clone, Copy)]
struct Rectangle {
    pos: Vec2,
    color: Color,
    selected: bool,
    target: Option<Vec2>,
}

impl Rectangle {
    fn new(x: f32, y: f32, color: Color) -> Self {
        Rectangle {
            pos: Vec2::new(x, y),
            color,
            selected: false,
            target: None,
        }
    }

    fn draw(&self) {
        let color = if self.selected {
            Color::new(self.color.r, self.color.g, self.color.b, 0.5)
        } else {
            self.color
        };
        draw_rectangle(self.pos.x, self.pos.y, RECT_SIZE, RECT_SIZE, color);
        if let Some(target) = self.target {
            draw_line(
                self.pos.x + RECT_SIZE / 2.0,
                self.pos.y + RECT_SIZE / 2.0,
                target.x,
                target.y,
                1.0,
                GRAY,
            );
        }
    }

    fn update(&mut self, dt: f32) {
        if let Some(target) = self.target {
            let direction = (target - self.pos).normalize();
            self.pos += direction * RECT_SPEED * dt;
            if self.pos.distance(target) < 1.0 {
                self.target = None;
            }
        }
    }

    fn collides_with(&self, other: &Rectangle) -> bool {
        self.pos.x < other.pos.x + RECT_SIZE
            && self.pos.x + RECT_SIZE > other.pos.x
            && self.pos.y < other.pos.y + RECT_SIZE
            && self.pos.y + RECT_SIZE > other.pos.y
    }
}

struct SelectionBox {
    start: Vec2,
    end: Vec2,
}

impl SelectionBox {
    fn new(start: Vec2) -> Self {
        SelectionBox { start, end: start }
    }

    fn draw(&self) {
        let top_left = Vec2::new(self.start.x.min(self.end.x), self.start.y.min(self.end.y));
        let size = (self.end - self.start).abs();
        draw_rectangle_lines(top_left.x, top_left.y, size.x, size.y, 2.0, BLUE);
    }

    fn contains(&self, point: Vec2) -> bool {
        let top_left = Vec2::new(self.start.x.min(self.end.x), self.start.y.min(self.end.y));
        let bottom_right = Vec2::new(self.start.x.max(self.end.x), self.start.y.max(self.end.y));
        point.x >= top_left.x
            && point.x <= bottom_right.x
            && point.y >= top_left.y
            && point.y <= bottom_right.y
    }
}

fn avoid_collisions(rectangles: &mut Vec<Rectangle>) {
    for i in 0..rectangles.len() {
        for j in (i + 1)..rectangles.len() {
            if rectangles[i].collides_with(&rectangles[j]) {
                let dir = (rectangles[i].pos - rectangles[j].pos).normalize();
                rectangles[i].pos += dir * COLLISION_PUSH;
                rectangles[j].pos -= dir * COLLISION_PUSH;
            }
        }
    }
}

#[macroquad::main("Advanced Interactive Rectangles")]
async fn main() {
    let mut rectangles: Vec<Rectangle> = (0..50)
        .map(|_| {
            Rectangle::new(
                rand::gen_range(0.0, screen_width() - RECT_SIZE),
                rand::gen_range(0.0, screen_height() - RECT_SIZE),
                Color::new(
                    rand::gen_range(0.0, 1.0),
                    rand::gen_range(0.0, 1.0),
                    rand::gen_range(0.0, 1.0),
                    1.0,
                ),
            )
        })
        .collect();

    avoid_collisions(&mut rectangles); // Initial collision avoidance

    let mut selection_box: Option<SelectionBox> = None;

    loop {
        clear_background(WHITE);

        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);

        // Handle selection box
        if is_mouse_button_pressed(MouseButton::Left) {
            selection_box = Some(SelectionBox::new(mouse_pos));
        } else if is_mouse_button_released(MouseButton::Left) {
            selection_box = None;
        }

        if let Some(box_) = &mut selection_box {
            box_.end = mouse_pos;
            box_.draw();

            // Update selection status in real-time
            for rect in &mut rectangles {
                rect.selected = box_.contains(rect.pos);
            }
        }

        // Handle right-click for setting target
        if is_mouse_button_pressed(MouseButton::Right) {
            for rect in &mut rectangles {
                if rect.selected {
                    rect.target = Some(mouse_pos);
                }
            }
        }

        // Update and draw rectangles
        for rect in &mut rectangles {
            rect.update(get_frame_time());
            rect.draw();
        }

        // Avoid collisions
        avoid_collisions(&mut rectangles);

        // Draw instructions
        draw_text(
            "Left-click and drag to select. Right-click to set target for selected rectangles.",
            10.0,
            20.0,
            20.0,
            BLACK,
        );

        next_frame().await
    }
}
