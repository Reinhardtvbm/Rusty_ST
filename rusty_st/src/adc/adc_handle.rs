use stm32g4::stm32g431::{Peripherals, ADC1, ADC2};

use super::adc_config_types::*;

pub enum ADCReadErr {
    Uninitialised,
    Unimplemented,
}

enum ADC {
    ADC1(*const ADC1),
    ADC2(*const ADC2),
}

macro_rules! adc_config_set_func_bits {
    ($func_name:ident, $field:ident, $enum_type:ty) => {
        pub fn $func_name(&self, conf: $enum_type) {
            match self.adc {
                ADC::ADC1(adc) => unsafe { adc.read().cfgr.write(|w| w.$field().bits(conf as u8)) },
                ADC::ADC2(adc) => unsafe { adc.read().cfgr.write(|w| w.$field().bits(conf as u8)) },
            }
        }
    };
}

macro_rules! adc_config_set_func_bit {
    ($func_name:ident, $field:ident, $enum_type:ty) => {
        pub fn $func_name(&self, conf: $enum_type) {
            match self.adc {
                ADC::ADC1(adc) => unsafe {
                    adc.read().cfgr.write(|w| {
                        w.$field().bit(match (conf as u8) {
                            0 => false,
                            1 => true,
                            _ => panic!(),
                        })
                    })
                },
                ADC::ADC2(adc) => unsafe {
                    adc.read().cfgr.write(|w| {
                        w.$field().bit(match (conf as u8) {
                            0 => false,
                            1 => true,
                            _ => panic!(),
                        })
                    })
                },
            }
        }
    };
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

    pub fn init(&self, config: AdcConfigData) {
        self.set_dma_enable(config.dma_enable);
    }

    adc_config_set_func_bits!(set_resolution, res, AdcResolution);

    adc_config_set_func_bit!(set_dma_enable, dmaen, AdcDmaEnable);

    adc_config_set_func_bit!(set_dma_config, dmacfg, AdcDmaConfig);

    adc_config_set_func_bits!(set_ext_trig_sel, extsel, AdcExtTrigSel);

    adc_config_set_func_bits!(set_ext_trig_en, exten, AdcExtTrigEn);

    adc_config_set_func_bit!(set_overrun_mode, ovrmod, AdcOvrMod);

    adc_config_set_func_bit!(set_continuous_conv_mode, cont, AdcCont);

    adc_config_set_func_bit!(set_auto_dly_conv_mode, autdly, AdcAutDly);

    adc_config_set_func_bit!(set_align, align, AdcAlign);

    adc_config_set_func_bit!(set_disc_enable, discen, AdcDiscEn);

    adc_config_set_func_bits!(set_disc_num, discnum, AdcDiscNum);

    adc_config_set_func_bit!(set_inj_disc_enable, jdiscen, AdcJDiscEn);

    adc_config_set_func_bit!(set_inj_disc_mode, jqm, AdcJQM);

    adc_config_set_func_bit!(set_awd1_sgl, awd1sgl, AdcAWD1SGL);

    adc_config_set_func_bit!(set_awd1_enable, awd1en, AdcAWD1En);

    adc_config_set_func_bit!(set_inj_awd1_enable, jawd1en, AdcJAWD1En);

    adc_config_set_func_bit!(set_inj_auto, jauto, AdcJAuto);

    adc_config_set_func_bits!(set_awd1_channel, awd1ch, AdcAWD1Ch);

    adc_config_set_func_bit!(set_injq_disable, jqdis, AdcJQDis);

    pub fn enable_dma(&self) {
        self.set_dma_enable(AdcDmaEnable::Enabled);
    }

    pub fn disable_dma(&self) {
        self.set_dma_enable(AdcDmaEnable::Disabled);
    }
}
