use particles::World;
use piston_window::{clear, rectangle, PistonWindow, WindowSettings};

fn main() {
    run();
}

fn run() {
    let (witdth, height) = (1280.0, 960.0);
    let mut window = WindowSettings::new("particles", [witdth, height])
        .exit_on_esc(true)
        .build::<PistonWindow>()
        .expect("Could not create a window.");

    let mut world = World::new(witdth, height);
    world.add_shapes(1000);

    while let Some(event) = window.next() {
        world.update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);
            world.particles_mut().iter_mut().for_each(|s| {
                rectangle(s.color(), s.size(), ctx.transform, renderer);
            });
        });
    }
}
