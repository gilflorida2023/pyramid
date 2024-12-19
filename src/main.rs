use nannou::prelude::*;

struct Model {
    length: f32,
    a: Point2,
    b: Point2,
    center: Point2,
    phi: f32,
    phi_sqrt: f32,
    phi_point: Point2,
    thickness: f32,
    smallcircle_radius: f32,
}


fn main() {
    nannou::app(model).view(view).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();
    let length = 200.0;
    let phi = (1.0 + f32::sqrt(5.0)) / 2.0;
    let phi_sqrt = f32::sqrt(phi);
    
    Model {
        length,
        a: pt2(-length, 0.0),
        b: pt2(length, 0.0),
        center: pt2(0.0, 0.0),
        phi,
        phi_sqrt,
        phi_point: pt2(0.0, phi_sqrt * length),
        thickness: 3.8,
        smallcircle_radius: length * phi_sqrt - length,
    }
}

/*fn update(_app: &App, _model: &mut Model, _update: Update) {
}*/

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.line()
        .start(model.a)
        .end(model.b)
        .color(RED)
        .stroke_weight(model.thickness);

    draw.line()
        .start(model.a)
        .end(model.phi_point)
        .color(RED)
        .stroke_weight(model.thickness);

    draw.arrow()
    .weight(3.0) // Set the line width
    .color(BLUE) 
    .points(model.a, model.phi_point); 
    draw.arrow()
    .weight(3.0) // Set the line width
    .color(BLUE) 
    .points( model.phi_point,model.a); 

    // Draw the label with mathematical symbols
    let label_position = Point2::new(
        (model.a.x + model.phi_point.x) / 2.0, 
        (model.a.y + model.phi_point.y) / 2.0 
    );

    draw.text(&format!("φ")) // Directly draw the text
        .xy(label_position)
        .color(BLUE)
        .font_size(72);

    draw.line()
        .start(model.phi_point)
        .end(model.b)
        .color(RED)
        .stroke_weight(model.thickness);

    draw.arrow()
    .weight(3.0) // Set the line width
    .color(BLUE) 
    .points(model.center, model.phi_point); 
    draw.arrow()
    .weight(3.0) // Set the line width
    .color(BLUE) 
    .points( model.phi_point,model.center); 

    // Draw the label with mathematical symbols
    let label_position = Point2::new(
        (model.center.x + model.phi_point.x) / 2.0, 
        (model.center.y + model.phi_point.y) / 2.0 
    );
    draw.text(&format!("√φ")) // Directly draw the text
        .xy(label_position)
        .color(BLUE)
        .font_size(72);
    
    draw.arrow()
    .weight(3.0) // Set the line width
    .color(BLUE) 
    .points(model.center, model.a); 
    draw.arrow()
    .weight(3.0) // Set the line width
    .color(BLUE) 
    .points( model.a,model.center); 

    // Draw the label with mathematical symbols
    let label_position = Point2::new(
        (model.center.x + model.a.x) / 2.0, 
        (model.center.y + model.a.y) / 2.0 
    );
    draw.text(&format!("1")) // Directly draw the text
        .xy(label_position)
        .color(BLUE)
        .font_size(72);

    draw.ellipse()
        .xy(model.center)
        .radius(model.length)
        .no_fill()
        .stroke(RED)
        .stroke_weight(model.thickness);

    draw.ellipse()
        .xy(model.center)
        .radius(model.length*model.phi_sqrt)
        .no_fill()
        .stroke(RED)
        .stroke_weight(model.thickness);

    draw.rect()
        .xy(model.center)
        .w_h(model.length*2.0, model.length*2.0)
        .no_fill()
        .stroke(BLUE)
        .stroke_weight(model.thickness);

    draw.ellipse()
        .xy(model.phi_point)
        .radius(model.smallcircle_radius)
        .no_fill()
        .stroke(ORANGE)
        .stroke_weight(model.thickness);
    
    draw.rect()
        .xy(model.phi_point)
        .w_h((model.smallcircle_radius)*2.0,(model.smallcircle_radius)*2.0)
        .no_fill()
        .stroke(ORANGE)
        .stroke_weight(model.thickness);

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
