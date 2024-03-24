use stm32g4::stm32g431::{self, Peripherals};

use crate::adc::ADCHandle;

pub struct HAL {
    peripherals: Peripherals,
    adc1: ADCHandle,
}

impl HAL {
    pub fn init() -> Self {
        Self {
            peripherals: stm32g431::Peripherals::take().unwrap(),
            adc1: ADCHandle::new(),
        }
    }
}
