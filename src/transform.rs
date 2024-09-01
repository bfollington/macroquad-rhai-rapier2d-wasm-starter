use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Transform {
    position: Vec2,
    rotation: f32,
    scale: Vec2,
}

impl Transform {
    fn new(position: Vec2, rotation: f32, scale: Vec2) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    fn to_matrix(&self) -> Mat3 {
        Mat3::from_scale_angle_translation(self.scale, self.rotation, self.position)
    }
}

struct TransformStack {
    stack: Vec<Mat3>,
}

impl TransformStack {
    fn new() -> Self {
        Self {
            stack: vec![Mat3::IDENTITY],
        }
    }

    fn push(&mut self, transform: Transform) {
        let current = self.stack.last().unwrap_or(&Mat3::IDENTITY);
        let new = *current * transform.to_matrix();
        self.stack.push(new);
    }

    fn pop(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }

    fn current(&self) -> Mat3 {
        *self.stack.last().unwrap_or(&Mat3::IDENTITY)
    }
}

fn draw_rectangle_transformed(x: f32, y: f32, w: f32, h: f32, color: Color, transform: Mat3) {
    let points = [
        transform.transform_point2(vec2(x, y)),
        transform.transform_point2(vec2(x + w, y)),
        transform.transform_point2(vec2(x + w, y + h)),
        transform.transform_point2(vec2(x, y + h)),
    ];

    draw_triangle(points[0], points[1], points[2], color);
    draw_triangle(points[0], points[2], points[3], color);
}

#[macroquad::main("Nested Transform")]
async fn main() {
    let mut transform_stack = TransformStack::new();

    loop {
        clear_background(WHITE);

        let time = get_time() as f32;

        // Root transform
        let root_transform = Transform::new(
            vec2(screen_width() * 0.5, screen_height() * 0.5),
            time * 0.5,
            Vec2::ONE,
        );

        transform_stack.push(root_transform);

        // Draw the root rectangle
        draw_rectangle_transformed(-50.0, -50.0, 100.0, 100.0, RED, transform_stack.current());

        // Child transform
        let child_transform = Transform::new(vec2(100.0, 0.0), time * 2.0, Vec2::ONE * 0.5);

        transform_stack.push(child_transform);

        // Draw the child rectangle
        draw_rectangle_transformed(-50.0, -50.0, 100.0, 100.0, BLUE, transform_stack.current());

        // Grandchild transform
        let grandchild_transform = Transform::new(vec2(100.0, 0.0), time * -3.0, Vec2::ONE * 0.5);

        transform_stack.push(grandchild_transform);

        // Draw the grandchild rectangle
        draw_rectangle_transformed(-50.0, -50.0, 100.0, 100.0, GREEN, transform_stack.current());

        // Pop all transforms
        transform_stack.pop();
        transform_stack.pop();
        transform_stack.pop();

        next_frame().await
    }
}
