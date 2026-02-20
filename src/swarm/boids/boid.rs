use crate::math::{Vec3, Vector};
use crate::swarm::boids::FlockParams;

#[derive(Debug, Clone, Copy)]
pub struct Boid {
    pub position: Vec3,
    pub velocity: Vec3,
}

impl Boid {
    pub fn new(position: Vec3, velocity: Vec3) -> Boid {
        Boid { position, velocity }
    }

    pub fn separation(&self, other: &[Boid], params: &FlockParams) -> Vec3 {
        let mut steer = Vec3::zero();
        let mut count: f32 = 0.0;

        for boid in other {
            let mut diff = self.position - boid.position;
            let d_square = diff.magnitude_squared();
            if d_square > 0.0 && d_square < params.perception_radius * params.perception_radius {
                diff *= 1.0 / d_square;
                steer += diff;
                count += 1.0;
            }
        }

        if count > 0.0 {
            steer /= count;
        }

        steer
    }

    pub fn alignment(&self, other: &[Boid], params: &FlockParams) -> Vec3 {
        let mut avg_velocity = Vec3::zero();
        let mut count: f32 = 0.0;
        let mut steer = Vec3::zero();

        for boid in other {
            let diff = self.position - boid.position;
            let d_square = diff.magnitude_squared();
            if d_square > 0.0 && d_square < params.perception_radius * params.perception_radius {
                avg_velocity += boid.velocity;
                count += 1.0;
            }
        }

        if count > 0.0 {
            avg_velocity /= count;
            steer = avg_velocity - self.velocity
        }

        steer
    }

    pub fn cohesion(&self, other: &[Boid], params: &FlockParams) -> Vec3 {
        let mut center = Vec3::zero();
        let mut count: f32 = 0.0;
        let mut steer = Vec3::zero();

        for boid in other {
            let diff = self.position - boid.position;
            let d_square = diff.magnitude_squared();
            if d_square > 0.0 && d_square < params.perception_radius * params.perception_radius {
                center += boid.position;
                count += 1.0;
            }
        }
        if count > 0.0 {
            center /= count;
            steer = center - self.position;
        }
        steer
    }

    pub fn update(&mut self, other: &[Boid], params: &FlockParams, dt: f32) {
        let mut sep = self.separation(other, params);
        let mut ali = self.alignment(other, params);
        let mut coh = self.cohesion(other, params);

        sep *= params.separation_weight;
        ali *= params.alignment_weight;
        coh *= params.cohesion_weight;

        let force = (sep + ali + coh).limit(params.max_force);
        self.velocity = self.velocity + force * dt;
        self.velocity = self.velocity.limit(params.max_speed);
        self.position = self.position + self.velocity * dt;
    }
}
