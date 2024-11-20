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
        circle_radius: 200.0,  // Radius of the first circle
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    
    let center = pt2(0.0, 0.0);
    
    // Calculate second circle radius based on original circle's diameter divided into 11 parts
    // and taking 7 parts of that
    let part_size = (model.circle_radius * 2.0) / 11.0;  // divide diameter by 11
    let second_radius = part_size * 7.0;  // take 7 parts
    
    // Draw first circle (larger)
    draw.ellipse()
        .xy(center)
        .radius(model.circle_radius)
        .no_fill()
        .stroke(BLACK)
        .stroke_weight(2.0);
    
    // Calculate square size for circle to be inscribed
    // For a circle inscribed in a square, square side = diameter = 2 * radius
    let square_size = model.circle_radius * 2.0;
    
    // Draw square with circle inscribed
    draw.rect()
        .xy(center)
        .w_h(square_size, square_size)
        .no_fill()
        .stroke(BLACK)
        .stroke_weight(2.0);
    
    // Draw horizontal diameter D across first (larger) circle
    let diameter_start = pt2(-model.circle_radius, 0.0);  // Using model.circle_radius (200.0) for the larger circle
    let diameter_end = pt2(model.circle_radius, 0.0);
    draw.line()
        .start(diameter_start)
        .end(diameter_end)
        .color(BLACK)
        .stroke_weight(2.0);

    // Draw second circle (smaller)
    draw.ellipse()
        .xy(center)
        .radius(second_radius)
        .no_fill()
        .stroke(BLACK)
        .stroke_weight(2.0);
    
    // Draw point T at top of second (smaller) circle
    let point_t = pt2(0.0, second_radius);  // Keeping T at top of smaller circle
    draw.ellipse()
        .xy(point_t)
        .radius(3.0)
        .color(BLACK);

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