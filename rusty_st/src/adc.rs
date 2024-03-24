use stm32g4::stm32g431::{ADC1, ADC2};

pub enum ADCReadErr {
    Uninitialised,
}

enum ADC {
    ADC1(ADC1),
    ADC2(ADC2),
}

pub struct ADCHandle {
    adc: Option<ADC>,
}

impl ADCHandle {
    pub fn new() -> Self {
        Self { adc: None }
    }

    pub fn read(&self) -> Result<u16, ADCReadErr> {
        if let Some(adc) = &self.adc {
            return match adc {
                ADC::ADC1(adc1) => {
                    todo!()
                }
                ADC::ADC2(adc2) => {
                    todo!()
                }
            };
        }

        Err(ADCReadErr::Uninitialised)
    }
}
