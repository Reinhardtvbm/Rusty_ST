use stm32g4::{stm32g431::adc1::cfgr::CFGR_SPEC, Reg};

pub trait HalAdc {
    fn cfgr(&self) -> &Reg<CFGR_SPEC>;
}

pub struct AdcConfigData {
    pub resolution: AdcResolution,
    pub dma_enable: AdcDmaEnable,
}

#[repr(u8)]
pub enum AdcResolution {
    Twelve,
    Ten,
    Eight,
    Six,
}

#[repr(u8)]
pub enum AdcDmaEnable {
    Enabled,
    Disabled,
}

#[repr(u8)]
pub enum AdcDmaConfig {
    OneShot,
    Circular,
}

#[repr(u8)]
pub enum AdcExtTrigSel {
    Event0,
    Event1,
    Event2,
    Event3,
    Event4,
    Event5,
    Event6,
    Event7,
    Event8,
    Event9,
    Event10,
    Event11,
    Event12,
    Event13,
    Event14,
    Event15,
    Event16,
    Event17,
    Event18,
    Event19,
    Event20,
    Event21,
    Event22,
    Event23,
    Event24,
    Event25,
    Event26,
    Event27,
    Event28,
    Event29,
    Event30,
    Event31,
}

#[repr(u8)]
pub enum AdcExtTrigEn {
    Disabled,
    RisingEdge,
    FallingEdge,
    RisingAndFallingEdge,
}

#[repr(u8)]
pub enum AdcOvrMod {
    Preserved,
    Overwritten,
}

#[repr(u8)]
pub enum AdcCont {
    Single,
    Continuous,
}

#[repr(u8)]
pub enum AdcAutDly {
    Off,
    On,
}

#[repr(u8)]
pub enum AdcAlign {
    Right,
    Left,
}

#[repr(u8)]
pub enum AdcDiscEn {
    Disabled,
    Enabled,
}

#[repr(u8)]
pub enum AdcDiscNum {
    ChannelsCnt1,
    ChannelsCnt2,
    ChannelsCnt3,
    ChannelsCnt4,
    ChannelsCnt5,
    ChannelsCnt6,
    ChannelsCnt7,
    ChannelsCnt8,
}

#[repr(u8)]
pub enum AdcJDiscEn {
    Disabled,
    Enabled,
}

#[repr(u8)]
pub enum AdcJQM {
    Mode0,
    Mode1,
}

#[repr(u8)]
pub enum AdcAWD1SGL {
    EnAll,
    EnSingle,
}

#[repr(u8)]
pub enum AdcAWD1En {
    Disabled,
    Enabled,
}

#[repr(u8)]
pub enum AdcJAWD1En {
    Disabled,
    Enabled,
}

#[repr(u8)]
pub enum AdcJAuto {
    Disabled,
    Enabled,
}

#[repr(u8)]
pub enum AdcAWD1Ch {
    Channel0,
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
    Channel8,
    Channel9,
    Channel10,
    Channel11,
    Channel12,
    Channel13,
    Channel14,
    Channel15,
    Channel16,
    Channel17,
    Channel18,
}

#[repr(u8)]
pub enum AdcJQDis {
    Enabled,
    Disabled,
}
