use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OSC_HP Status Register"]
    pub oschpstat: OSCHPSTAT,
    #[doc = "0x04 - OSC_HP Control Register"]
    pub oschpctrl: OSCHPCTRL,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - Clock Calibration Constant Register"]
    pub clkcalconst: CLKCALCONST,
}
#[doc = "OSC_HP Status Register"]
pub struct OSCHPSTAT {
    register: VolatileCell<u32>,
}
#[doc = "OSC_HP Status Register"]
pub mod oschpstat;
#[doc = "OSC_HP Control Register"]
pub struct OSCHPCTRL {
    register: VolatileCell<u32>,
}
#[doc = "OSC_HP Control Register"]
pub mod oschpctrl;
#[doc = "Clock Calibration Constant Register"]
pub struct CLKCALCONST {
    register: VolatileCell<u32>,
}
#[doc = "Clock Calibration Constant Register"]
pub mod clkcalconst;
