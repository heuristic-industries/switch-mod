use crate::{Persistence, SwitchTimer, ToggleSwitch};
use attiny_hal::port::{
    mode::{Input, Output, PullUp},
    Pin, PB2, PB4,
};

pub struct BypassSwitch {
    switch: ToggleSwitch<Pin<Input<PullUp>, PB2>, Pin<Output, PB4>>,
}

impl BypassSwitch {
    pub fn new(pins: attiny_hal::port::Pins, active: bool) -> Self {
        let input = pins.pb2.into_pull_up_input();
        let output = pins.pb4.into_output();
        let default_state_pin = pins.pb3.into_pull_up_input();
        let default_state = default_state_pin.is_low();
        let mut switch = ToggleSwitch::new(input, output, default_state, active);
        switch.init();

        BypassSwitch { switch }
    }

    pub fn on_change(&mut self, timer: &mut SwitchTimer, persistence: &mut Persistence) {
        let did_change = self.switch.on_change(timer);
        if did_change {
            persistence.set_bypass(self.switch.active);
        }
    }
}
