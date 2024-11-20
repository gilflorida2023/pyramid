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
    
    // Draw first circle
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
    
    // Calculate second circle radius based on original circle's diameter divided into 11 parts
    // and taking 7 parts of that
    let part_size = (model.circle_radius * 2.0) / 11.0;  // divide diameter by 11
    let second_radius = part_size * 7.0;  // take 7 parts
    
    // Draw second circle
    draw.ellipse()
        .xy(center)
        .radius(second_radius)
        .no_fill()
        .stroke(BLACK)
        .stroke_weight(2.0);
    
    // Draw center point
    draw.ellipse()
        .xy(center)
        .radius(3.0)
        .color(BLACK);
    
    draw.to_frame(app, &frame).unwrap();
}