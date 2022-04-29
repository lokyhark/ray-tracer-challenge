use std::{f64::consts::PI, fs::write, path::PathBuf};

use ray_tracer_challenge::{Canvas, Color, Matrix, Point};

fn main() {
    let mut canvas = Canvas::new(400, 400);
    let point = Point::new(0., 1., 0.);
    let radius = 3. * canvas.width() as f64 / 8.;
    for hour in 0..12 {
        let rotation = Matrix::rotation_z(hour as f64 * PI / 6.);
        let point = rotation * point;
        let pixel = (
            (radius * point.x).round() + canvas.width() as f64 / 2.,
            (radius * point.y).round() + canvas.height() as f64 / 2.,
        );
        *canvas.get_mut(pixel.0 as usize, pixel.1 as usize) = Color::white();
    }

    let ppm = canvas.ppm();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("examples/chapter_04/output.ppm");
    if let Err(error) = write(&path, &ppm.as_bytes()) {
        panic!("failed to write to {}: {}", path.display(), error);
    }
}
