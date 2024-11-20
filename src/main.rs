use nannou::prelude::*;

struct Model {
    circle_radius: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();
    
    Model {
        circle_radius: 200.0,  // Radius of circle L
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    
    let center = pt2(0.0, 0.0);
    
    // Calculate circle c's radius based on circle L's diameter divided into 11 parts
    // and taking 7 parts of that
    let part_size = (model.circle_radius * 2.0) / 11.0;  // divide diameter by 11
    let second_radius = part_size * 7.0;  // take 7 parts
    
    // Draw circle L (larger circle)
    draw.ellipse()
        .xy(center)
        .radius(model.circle_radius)
        .no_fill()
        .stroke(BLACK)
        .stroke_weight(2.0);
    
    // Calculate square s size to be tangent to circle L
    // For a circle inscribed in a square, square side = diameter = 2 * radius
    let square_size = model.circle_radius * 2.0;
    
    // Draw square s
    draw.rect()
        .xy(center)
        .w_h(square_size, square_size)
        .no_fill()
        .stroke(BLACK)
        .stroke_weight(2.0);
    
    // Draw diameter line D across circle L
    let diameter_start = pt2(-model.circle_radius, 0.0);
    let diameter_end = pt2(model.circle_radius, 0.0);
    draw.line()
        .start(diameter_start)
        .end(diameter_end)
        .color(BLACK)
        .stroke_weight(2.0);

    // Draw circle c (smaller circle)
    draw.ellipse()
        .xy(center)
        .radius(second_radius)
        .no_fill()
        .stroke(BLACK)
        .stroke_weight(2.0);
    
    // Draw point T at top of circle c
    let point_t = pt2(0.0, second_radius);
    draw.ellipse()
        .xy(point_t)
        .radius(3.0)
        .color(BLACK);

    // Calculate radius of circle t (centered at point T)
    // Distance from T to top edge of square s
    let square_top_edge = square_size / 2.0;  // y-coordinate of square's top edge
    let new_circle_radius = square_top_edge - second_radius;  // distance from T to square's top edge

    // Draw circle t
    draw.ellipse()
        .xy(point_t)
        .radius(new_circle_radius)
        .no_fill()
        .stroke(BLACK)
        .stroke_weight(2.0);

    // Draw square tangent to circle t
    let square_t_size = new_circle_radius * 2.0;  // Square side length = diameter of circle t
    draw.rect()
        .xy(point_t)  // Centered at same point as circle t
        .w_h(square_t_size, square_t_size)
        .no_fill()
        .stroke(BLACK)
        .stroke_weight(2.0);

    // Calculate and draw lines connecting top corners of squares s and t
    let s_top_left = pt2(-square_size/2.0, square_size/2.0);
    let s_top_right = pt2(square_size/2.0, square_size/2.0);
    let t_top_left = pt2(-square_t_size/2.0, point_t.y + square_t_size/2.0);
    let t_top_right = pt2(square_t_size/2.0, point_t.y + square_t_size/2.0);

    // Draw connecting lines between squares
    draw.line()
        .start(s_top_left)
        .end(t_top_left)
        .color(BLACK)
        .stroke_weight(2.0);

    draw.line()
        .start(s_top_right)
        .end(t_top_right)
        .color(BLACK)
        .stroke_weight(2.0);

    // Draw lines from T to ends of diameter D
    draw.line()
        .start(point_t)
        .end(diameter_start)
        .color(BLACK)
        .stroke_weight(2.0);
    
    draw.line()
        .start(point_t)
        .end(diameter_end)
        .color(BLACK)
        .stroke_weight(2.0);

    // Draw center point
    draw.ellipse()
        .xy(center)
        .radius(3.0)
        .color(BLACK);
    
    draw.to_frame(app, &frame).unwrap();
}