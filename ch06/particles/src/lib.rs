use std::{
    alloc::{GlobalAlloc, System},
    iter::repeat_n,
    time::Instant,
};

use graphics::math::Vec2d;
use rand::{rngs::ThreadRng, thread_rng, Rng};

#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

struct ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout);
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();
        eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());

        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        System.dealloc(ptr, layout);
    }
}

pub struct World {
    current_turn: u64,
    #[allow(clippy::vec_box)]
    // 벤치마크를 위한 박스
    particles: Vec<Box<Particle>>,
    height: f64,
    width: f64,
    rng: ThreadRng,
}

pub struct Particle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

impl Particle {
    fn new(world: &World) -> Particle {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..=world.width);
        let y = world.height;
        let x_velocity = 0.0;
        let y_velocity = rng.gen_range(-2.0..0.0);
        let x_acceleration = 0.0;
        let y_acceleration = rng.gen_range(0.0..0.15);

        Particle {
            height: 4.0,
            width: 4.0,
            position: [x, y],
            velocity: [x_velocity, y_velocity],
            acceleration: [x_acceleration, y_acceleration],
            color: [1.0, 1.0, 1.0, 0.99],
        }
    }

    fn update(&mut self) {
        self.velocity = graphics::math::add(self.velocity, self.acceleration);
        self.position = graphics::math::add(self.position, self.velocity);
        self.acceleration = graphics::math::mul_scalar(self.acceleration, 0.7);
        self.color[3] *= 0.995;
    }

    pub fn size(&self) -> (f64, f64, f64, f64) {
        (self.position[0], self.position[1], self.width, self.height)
    }

    pub fn color(&self) -> [f32; 4] {
        self.color
    }
}

impl World {
    pub fn new(width: f64, height: f64) -> World {
        World {
            current_turn: 0,
            particles: Vec::new(),
            height,
            width,
            rng: thread_rng(),
        }
    }

    pub fn particles_mut(&mut self) -> &mut [Box<Particle>] {
        &mut self.particles
    }

    pub fn add_shapes(&mut self, n: i32) {
        repeat_n((), n.unsigned_abs() as usize).for_each(|_| {
            let particle = Particle::new(self);
            let boxed_particle = Box::new(particle);
            self.particles.push(boxed_particle);
        })
    }

    fn remove_shapes(&mut self, n: i32) {
        repeat_n((), n.unsigned_abs() as usize).for_each(|_| {
            let to_delete = self.particles.iter().enumerate().find(|(_, particle)| {
                if particle.color[3] < 0.02 {
                    return true;
                }
                false
            });

            if let Some((i, _)) = to_delete {
                self.particles.remove(i);
            } else {
                self.particles.remove(0);
            }
        });
    }

    pub fn update(&mut self) {
        let n = self.rng.gen_range(-3..=3);

        if n > 0 {
            self.add_shapes(n);
        } else {
            self.remove_shapes(n);
        }

        self.particles.shrink_to_fit();
        self.particles.iter_mut().for_each(|shape| shape.update());
        self.current_turn += 1;
    }
}
