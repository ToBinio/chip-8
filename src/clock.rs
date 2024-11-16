use std::time::SystemTime;

pub struct Clock {
    delay_timer: u8,
    sound_timer: u8,

    lastTick: u128,
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            delay_timer: 0,
            sound_timer: 0,
            lastTick: Self::get_current_millis(),
        }
    }

    fn get_current_millis() -> u128 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    pub fn tick(&mut self) {
        let current = Self::get_current_millis();
        if ((current - self.lastTick) > 1000 / 60) {
            self.lastTick += 1000 / 60;

            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
                println!("PEEP");
            }
        }
    }

    pub fn delay_timer(&self) -> u8 {
        self.delay_timer
    }

    pub fn sound_timer(&self) -> u8 {
        self.sound_timer
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer = value;
    }

    pub fn set_sound_timer(&mut self, value: u8) {
        self.sound_timer = value;
    }
}
