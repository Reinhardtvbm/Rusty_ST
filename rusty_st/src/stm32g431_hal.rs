use stm32g4::stm32g431::{self, Peripherals};

use crate::adc::ADCHandle;

pub struct HAL {
    peripherals: Peripherals,
    adc1: ADCHandle,
    adc2: ADCHandle,
}

impl HAL {
    pub fn init() -> Self {
        let peripherals =
            stm32g431::Peripherals::take().expect("The HAL cannot be initialised twice");

        let adc1 = ADCHandle::adc1(&peripherals);
        let adc2 = ADCHandle::adc2(&peripherals);

        Self {
            peripherals,
            adc1,
            adc2,
        }
    }
}
