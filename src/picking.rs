use macroquad::prelude::*;

struct Rectangle {
    rect: Rect,
    color: Color,
    is_dragging: bool,
    drag_offset: Vec2,
}

impl Rectangle {
    fn new(x: f32, y: f32, w: f32, h: f32, color: Color) -> Self {
        Rectangle {
            rect: Rect::new(x, y, w, h),
            color,
            is_dragging: false,
            drag_offset: Vec2::ZERO,
        }
    }

    fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.color,
        );
    }

    fn contains(&self, point: Vec2) -> bool {
        self.rect.contains(point)
    }

    fn start_drag(&mut self, mouse_pos: Vec2) {
        self.is_dragging = true;
        self.drag_offset = mouse_pos - Vec2::new(self.rect.x, self.rect.y);
    }

    fn update_position(&mut self, mouse_pos: Vec2) {
        if self.is_dragging {
            self.rect.x = mouse_pos.x - self.drag_offset.x;
            self.rect.y = mouse_pos.y - self.drag_offset.y;
        }
    }

    fn stop_drag(&mut self) {
        self.is_dragging = false;
    }
}

#[macroquad::main("Interactive Rectangles")]
async fn main() {
    let mut rectangles = vec![
        Rectangle::new(100.0, 100.0, 100.0, 100.0, BLUE),
        Rectangle::new(250.0, 100.0, 100.0, 100.0, RED),
        Rectangle::new(400.0, 100.0, 100.0, 100.0, GREEN),
    ];

    loop {
        clear_background(WHITE);

        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
        let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        let mouse_down = is_mouse_button_down(MouseButton::Left);

        for rect in rectangles.iter_mut() {
            if rect.contains(mouse_pos) {
                if mouse_pressed {
                    rect.start_drag(mouse_pos);
                }
                rect.color.a = 0.7; // Change transparency on hover
            } else {
                rect.color.a = 1.0; // Reset transparency when not hovering
            }

            if mouse_down {
                rect.update_position(mouse_pos);
            } else {
                rect.stop_drag();
            }

            rect.draw();
        }

        draw_text(
            "Hover over rectangles and drag them!",
            20.0,
            20.0,
            30.0,
            BLACK,
        );

        next_frame().await
    }
}
