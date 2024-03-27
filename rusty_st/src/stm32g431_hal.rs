use stm32g4::stm32g431::{self, Peripherals, ADC1, ADC2};

use crate::adc::adc_handle::ADC;

pub struct HAL {
    adc1: ADC<ADC1>,
    adc2: ADC<ADC2>,
}

impl HAL {
    pub fn init() -> Self {
        let peripherals =
            stm32g431::Peripherals::take().expect("The HAL cannot be initialised twice");

        let adc1 = ADC::new(peripherals.ADC1);
        let adc2 = ADC::new(peripherals.ADC2);

        Self { adc1, adc2 }
    }
}
