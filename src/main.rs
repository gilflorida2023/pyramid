use nannou::prelude::*;
struct Model {
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
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let length = 200.0;
    let a = pt2(-length,0.0);
    let b = pt2(length,0.0);
    let center = pt2(0.0,0.0);
    let phi = (1.0 + f32::sqrt(5.0)) / 2.0; 
    let phi_sqrt = f32::sqrt(phi);
    let phi_point = pt2(0.0,phi_sqrt*length);
    draw.line()
        .start(a)
        .end(center)
        .color(BLUE)
        .stroke_weight(2.0);

    draw.line()
        .start(center)
        .end(b)
        .color(RED)
        .stroke_weight(2.0);


    draw.line()
        .start(a)
        .end(phi_point)
        .color(BLUE)
        .stroke_weight(2.0);

    draw.line()
        .start(phi_point)
        .end(b)
        .color(RED)
        .stroke_weight(2.0);
        
    draw.ellipse()
        .xy(center)
        .radius(length)
        .no_fill()
        .stroke(RED)
        .stroke_weight(2.0);

    draw.ellipse()
        .xy(center)
        .radius(length*phi_sqrt)
        .no_fill()
        .stroke(CYAN)
        .stroke_weight(2.0);

    draw.rect()
        .xy(center)
        .w_h(length*2.0, length*2.0)
        .no_fill()
        .stroke(BLUE)
        .stroke_weight(2.0);

    draw.ellipse()
        .xy(phi_point)
        .radius(length*phi_sqrt-length)
        .no_fill()
        .stroke(CYAN)
        .stroke_weight(2.0);
    
    draw.rect()
        .xy(phi_point)
        .w_h((length*phi_sqrt-length)*2.0,(length*phi_sqrt-length)*2.0)
        .no_fill()
        .stroke(CYAN)
        .stroke_weight(2.0);

    draw.to_frame(app, &frame).unwrap();
     // Save the frame when 'S' key is pressed
     if app.keys.down.contains(&Key::S) {
        let file_path = app.project_path()
            .expect("failed to locate project path")
            .join("output.png");
        app.main_window().capture_frame(file_path);
        println!("Frame saved!");
    }
}