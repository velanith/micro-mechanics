#[derive(Debug, PartialEq)]
pub struct PID {
    // coeffs
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,

    // state
    integration: f32,
    prev_error: f32,

    // limits
    output_min: f32,
    output_max: f32,
    max_integration: f32,
}

impl PID {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            integration: 0.0,
            prev_error: 0.0,
            output_min: -1.0,
            output_max: 1.0,
            max_integration: 1.0,
        }
    }
}

pub trait PIDController: Sized {
    fn update(&mut self, setpoint: f32, measured: f32, dt: f32) -> f32;

    fn set_limits(&mut self, output_min: f32, output_max: f32, max_integration: f32);
    fn set_gains(&mut self, kp: f32, ki: f32, kd: f32);

    fn reset(&mut self);
}

impl PIDController for PID {
    fn update(&mut self, setpoint: f32, measured: f32, dt: f32) -> f32 {
        // hedef - bulunduğumuz yer
        let error = setpoint - measured;

        // P: orantısal — hataya doğru orantılı tepki
        let p = error * self.kp;

        // I: integral — biriken hatayı topla (anti-windup ile sınırla)
        self.integration += error * dt;
        self.integration = self
            .integration
            .clamp(-self.max_integration, self.max_integration);
        let i = self.integration * self.ki;

        // D: türev — hatanın değişim hızına göre frenleme
        let d = (error - self.prev_error) / dt * self.kd;
        self.prev_error = error;

        // toplam çıktıyı fiziksel sınırlara kırp
        let output = p + i + d;
        output.clamp(self.output_min, self.output_max)
    }

    fn set_limits(&mut self, output_min: f32, output_max: f32, max_integration: f32) {
        self.output_min = output_min;
        self.output_max = output_max;
        self.max_integration = max_integration;
    }

    fn set_gains(&mut self, kp: f32, ki: f32, kd: f32) {
        self.kp = kp;
        self.ki = ki;
        self.kd = kd;
    }

    fn reset(&mut self) {
        self.integration = 0.0;
        self.prev_error = 0.0;
    }
}
