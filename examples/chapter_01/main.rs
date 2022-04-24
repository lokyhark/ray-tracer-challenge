use ray_tracer_challenge::{Point, Vector};

fn main() {
    let mut projectile = Projectile {
        position: Point::new(0., 1., 0.),
        velocity: Vector::new(1., 1., 0.).normalized(),
    };

    let environment = Environment {
        gravity: Vector::new(0., -0.1, 0.),
        wind: Vector::new(-0.01, 0., 0.),
    };

    while projectile.position.y > 0. {
        tick(&mut projectile, &environment);
    }

    println!("distance: {:.6e}", projectile.position.x);
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
