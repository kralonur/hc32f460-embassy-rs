#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    vltsel: Vltsel,
    outmon: Outmon,
    cvsstb: Cvsstb,
    cvsprd: Cvsprd,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CTRL"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x02 - desc VLTSEL"]
    #[inline(always)]
    pub const fn vltsel(&self) -> &Vltsel {
        &self.vltsel
    }
    #[doc = "0x04 - desc OUTMON"]
    #[inline(always)]
    pub const fn outmon(&self) -> &Outmon {
        &self.outmon
    }
    #[doc = "0x06 - desc CVSSTB"]
    #[inline(always)]
    pub const fn cvsstb(&self) -> &Cvsstb {
        &self.cvsstb
    }
    #[doc = "0x08 - desc CVSPRD"]
    #[inline(always)]
    pub const fn cvsprd(&self) -> &Cvsprd {
        &self.cvsprd
    }
}
#[doc = "CTRL (rw) register accessor: desc CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "desc CTRL"]
pub mod ctrl;
#[doc = "VLTSEL (rw) register accessor: desc VLTSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`vltsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vltsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vltsel`] module"]
#[doc(alias = "VLTSEL")]
pub type Vltsel = crate::Reg<vltsel::VltselSpec>;
#[doc = "desc VLTSEL"]
pub mod vltsel;
#[doc = "OUTMON (r) register accessor: desc OUTMON\n\nYou can [`read`](crate::Reg::read) this register and get [`outmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outmon`] module"]
#[doc(alias = "OUTMON")]
pub type Outmon = crate::Reg<outmon::OutmonSpec>;
#[doc = "desc OUTMON"]
pub mod outmon;
#[doc = "CVSSTB (rw) register accessor: desc CVSSTB\n\nYou can [`read`](crate::Reg::read) this register and get [`cvsstb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvsstb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvsstb`] module"]
#[doc(alias = "CVSSTB")]
pub type Cvsstb = crate::Reg<cvsstb::CvsstbSpec>;
#[doc = "desc CVSSTB"]
pub mod cvsstb;
#[doc = "CVSPRD (rw) register accessor: desc CVSPRD\n\nYou can [`read`](crate::Reg::read) this register and get [`cvsprd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvsprd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvsprd`] module"]
#[doc(alias = "CVSPRD")]
pub type Cvsprd = crate::Reg<cvsprd::CvsprdSpec>;
#[doc = "desc CVSPRD"]
pub mod cvsprd;
