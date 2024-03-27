use core::ops::Deref;

extern crate alloc;
use adc_macro::create_set_fn;
use stm32g4::stm32g431::adc1::RegisterBlock;

use super::adc_config_types::*;

macro_rules! gen_set_fns {
    ( $([$reg:ident, $([ $field: ident, $conf_enum:ty, $bits:expr ]),* ]),* ) => {
        $(
            $(
                create_set_fn!($reg, $field, $conf_enum, $bits);
            )*
        )*
    };
}

pub enum ADCReadErr {
    Uninitialised,
    Unimplemented,
}

pub struct ADC<R> {
    register_block: R,
}

impl<R> ADC<R>
where
    R: Deref<Target = RegisterBlock>,
{
    pub fn new(register: R) -> Self {
        Self {
            register_block: register,
        }
    }

    pub fn init(&self, config: AdcConfigData) {
        self.set_dmaen(config.dma_enable);
    }

    gen_set_fns!([
        cfgr,
        [res, AdcResolution, true],
        [dmaen, AdcDmaEnable, false],
        [dmacfg, AdcDmaConfig, false],
        [extsel, AdcExtTrigSel, true],
        [exten, AdcExtTrigEn, true],
        [ovrmod, AdcOvrMod, false],
        [cont, AdcCont, false],
        [autdly, AdcAutDly, false],
        [align, AdcAlign, false],
        [discen, AdcDiscEn, false],
        [discnum, AdcDiscNum, true],
        [jdiscen, AdcJDiscEn, false],
        [jqm, AdcJQM, false],
        [awd1sgl, AdcAWD1SGL, false],
        [awd1en, AdcAWD1En, false],
        [jawd1en, AdcJAWD1En, false],
        [jauto, AdcJAuto, false],
        [awd1ch, AdcAWD1Ch, true],
        [jqdis, AdcJQDis, false]
    ]);

    pub fn enable_dma(&self) {
        self.set_dmaen(AdcDmaEnable::Enabled);
    }

    pub fn disable_dma(&self) {
        self.set_dmaen(AdcDmaEnable::Disabled);
    }
}
