/// throttle bu eşiğin altındaysa tüm motorlar kapatılır.
/// RC kumanda potansiyometre gürültüsü ve ADC toleransı nedeniyle
/// tam 0.0 yerine küçük bir eşik kullanıyoruz.
const THROTTLE_MIN: f32 = 0.05;

#[derive(Debug, PartialEq)]
pub struct Mixer {
    // motor outputs (0.0 = kapalı, 1.0 = tam güç)
    m1: f32,
    m2: f32,
    m3: f32,
    m4: f32,
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            m1: 0.0,
            m2: 0.0,
            m3: 0.0,
            m4: 0.0,
        }
    }

    pub fn get_motors(&self) -> [f32; 4] {
        [self.m1, self.m2, self.m3, self.m4]
    }
}

pub trait MixerController: Sized {
    fn mix(&mut self, roll: f32, pitch: f32, yaw: f32, throttle: f32);
}

impl MixerController for Mixer {
    // X-Configuration coeff
    // m1: +roll -pitch +yaw +throttle
    // m2: -roll -pitch -yaw +throttle
    // m3: -roll +pitch -yaw +throttle
    // m4: +roll +pitch +yaw +throttle

    fn mix(&mut self, roll: f32, pitch: f32, yaw: f32, throttle: f32) {
        // güvenlik: throttle minimum eşiğin altındaysa tüm motorları kapat.
        // drone yerdeyken IMU titreşimlerinden kaynaklanan PID tepkilerinin
        // motorları seğirtmesini (twitching) önler.
        if throttle < THROTTLE_MIN {
            self.m1 = 0.0;
            self.m2 = 0.0;
            self.m3 = 0.0;
            self.m4 = 0.0;
            return;
        }

        // mix + clamp: ESC'ler 0.0..1.0 aralığında çalışır.
        // negatif motor gücü fiziksel olarak imkansız,
        // 1.0 üstü ise tanımsız davranışa yol açar.
        self.m1 = (roll - pitch + yaw + throttle).clamp(0.0, 1.0);
        self.m2 = (-roll - pitch - yaw + throttle).clamp(0.0, 1.0);
        self.m3 = (-roll + pitch - yaw + throttle).clamp(0.0, 1.0);
        self.m4 = (roll + pitch + yaw + throttle).clamp(0.0, 1.0);
    }
}
