use std::{
    fs::write,
    path::{Path, PathBuf},
};

use ray_tracer_challenge::{Canvas, Color, Point, Vector};

const RED: Color = Color::red();

fn main() {
    let mut projectile = Projectile {
        position: Point::new(0., 1., 0.),
        velocity: Vector::new(1., 1.8, 0.).normalized() * 11.25,
    };

    let environment = Environment {
        gravity: Vector::new(0., -0.1, 0.),
        wind: Vector::new(-0.01, 0., 0.),
    };

    let mut canvas = Canvas::with_color(900, 500, Color::white());

    while projectile.position.y > 0. {
        tick(&mut projectile, &environment);
        if projectile.position.y < 0. {
            break;
        }
        draw(&mut canvas, &projectile.position);
    }

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("examples/chapter_02/output.ppm");
    save(&canvas, &path);
}

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(projectile: &mut Projectile, environment: &Environment) {
    projectile.position += projectile.velocity;
    projectile.velocity += environment.gravity + environment.wind;
}

fn draw(canvas: &mut Canvas, position: &Point) {
    let x = position.x.round() as usize;
    let y = canvas.height() - position.y.round() as usize;
    *canvas.get_mut(x, y) = RED;
}

fn save(canvas: &Canvas, path: &Path) {
    let ppm = canvas.ppm();
    if let Err(error) = write(path, &ppm.as_bytes()) {
        panic!("failed to write to {}: {}", path.display(), error);
    }
}
