use crate::math::Vec3;
use crate::math::rem_euclid_f32;
use crate::swarm::boids::flock_params::BoundaryType;

use crate::swarm::boids::Boid;
use crate::swarm::boids::FlockParams;

pub struct Flock<const N: usize> {
    pub buffer_a: [Boid; N],
    pub buffer_b: [Boid; N],
    pub active_buffer: bool,
    pub params: FlockParams,
}

impl<const N: usize> Flock<N> {
    pub fn new(initial_boids: [Boid; N], params: FlockParams) -> Flock<N> {
        Flock {
            buffer_a: initial_boids,
            buffer_b: initial_boids,
            active_buffer: true,
            params,
        }
    }

    pub fn wrap_around(mut pos: Vec3, min: Vec3, max: Vec3) -> Vec3 {
        let width = max.x - min.x;
        let height = max.y - min.y;
        let depth = max.z - min.z;

        pos.x = rem_euclid_f32(pos.x - min.x, width) + min.x;
        pos.y = rem_euclid_f32(pos.y - min.y, height) + min.y;
        pos.z = rem_euclid_f32(pos.z - min.z, depth) + min.z;

        pos
    }

    pub fn soft_boundry(mut pos: Vec3, min: Vec3, max: Vec3) -> Vec3 {
        if pos.x < min.x {
            pos.x = min.x;
        }
        if pos.x > max.x {
            pos.x = max.x;
        }
        if pos.y < min.y {
            pos.y = min.y;
        }
        if pos.y > max.y {
            pos.y = max.y;
        }
        if pos.z < min.z {
            pos.z = min.z;
        }
        if pos.z > max.z {
            pos.z = max.z;
        }

        pos
    }

    pub fn current_boids(&self) -> &[Boid; N] {
        if self.active_buffer {
            &self.buffer_a
        } else {
            &self.buffer_b
        }
    }

    pub fn next_boids(&mut self) -> &mut [Boid; N] {
        if self.active_buffer {
            &mut self.buffer_b
        } else {
            &mut self.buffer_a
        }
    }

    pub fn tick(&mut self, dt: f32, bounds_min: Vec3, bounds_max: Vec3) {
        let snapshot: [Boid; N] = if self.active_buffer {
            self.buffer_a
        } else {
            self.buffer_b
        };

        for i in 0..N {
            let mut curr = snapshot[i];

            curr.update(&snapshot, &self.params, dt);

            curr.position = match self.params.boundary_type {
                BoundaryType::WrapAround => {
                    Self::wrap_around(curr.position, bounds_min, bounds_max)
                }
                BoundaryType::SoftBoundry => {
                    Self::soft_boundry(curr.position, bounds_min, bounds_max)
                }
            };

            // inactive buffer'a yaz
            if self.active_buffer {
                self.buffer_b[i] = curr;
            } else {
                self.buffer_a[i] = curr;
            }
        }
        self.active_buffer = !self.active_buffer;
    }
}
