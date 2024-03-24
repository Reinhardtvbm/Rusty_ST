use stm32g4::stm32g431::{Peripherals, ADC1, ADC2};

pub enum ADCReadErr {
    Uninitialised,
    Unimplemented,
}

enum ADC {
    ADC1(*const ADC1),
    ADC2(*const ADC2),
}

pub enum ADCResolution {
    Twelve,
    Ten,
    Eight,
    Six,
}

pub struct ADCHandle {
    adc: ADC,
}

impl ADCHandle {
    pub fn adc1(p: &Peripherals) -> Self {
        Self {
            adc: ADC::ADC1(&p.ADC1 as *const ADC1),
        }
    }

    pub fn adc2(p: &Peripherals) -> Self {
        Self {
            adc: ADC::ADC2(&p.ADC2 as *const ADC2),
        }
    }

    pub fn set_resolution(&self, res: ADCResolution) {
        match self.adc {
            ADC::ADC1(adc) => unsafe { adc.read().cfgr.write(|w| w.res().bits(res.into())) },
            ADC::ADC2(adc) => unsafe { adc.read().cfgr.write(|w| w.res().bits(res.into())) },
        }
    }
}

impl Into<u8> for ADCResolution {
    fn into(self) -> u8 {
        match self {
            ADCResolution::Twelve => 0b00,
            ADCResolution::Ten => 0b01,
            ADCResolution::Eight => 0b10,
            ADCResolution::Six => 0b11,
        }
    }
}
