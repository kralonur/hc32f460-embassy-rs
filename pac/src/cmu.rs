#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    pericksel: Pericksel,
    i2scksel: I2scksel,
    _reserved2: [u8; 0x0c],
    scfgr: Scfgr,
    usbckcfgr: Usbckcfgr,
    _reserved4: [u8; 0x01],
    ckswr: Ckswr,
    _reserved5: [u8; 0x03],
    pllcr: Pllcr,
    _reserved6: [u8; 0x03],
    upllcr: Upllcr,
    _reserved7: [u8; 0x03],
    xtalcr: Xtalcr,
    _reserved8: [u8; 0x03],
    hrccr: Hrccr,
    _reserved9: [u8; 0x01],
    mrccr: Mrccr,
    _reserved10: [u8; 0x03],
    oscstbsr: Oscstbsr,
    mco1cfgr: Mco1cfgr,
    mco2cfgr: Mco2cfgr,
    tpiuckcfgr: Tpiuckcfgr,
    xtalstdcr: Xtalstdcr,
    xtalstdsr: Xtalstdsr,
    _reserved16: [u8; 0x1f],
    mrctrm: Mrctrm,
    hrctrm: Hrctrm,
    _reserved18: [u8; 0x3f],
    xtalstbcr: Xtalstbcr,
    _reserved19: [u8; 0x5d],
    pllcfgr: Pllcfgr,
    upllcfgr: Upllcfgr,
    _reserved21: [u8; 0x0308],
    xtalcfgr: Xtalcfgr,
    _reserved22: [u8; 0x0f],
    xtal32cr: Xtal32cr,
    xtal32cfgr: Xtal32cfgr,
    _reserved24: [u8; 0x03],
    xtal32nfr: Xtal32nfr,
    _reserved25: [u8; 0x01],
    lrccr: Lrccr,
    _reserved26: [u8; 0x01],
    lrctrm: Lrctrm,
}
impl RegisterBlock {
    #[doc = "0x10 - desc PERICKSEL"]
    #[inline(always)]
    pub const fn pericksel(&self) -> &Pericksel {
        &self.pericksel
    }
    #[doc = "0x12 - desc I2SCKSEL"]
    #[inline(always)]
    pub const fn i2scksel(&self) -> &I2scksel {
        &self.i2scksel
    }
    #[doc = "0x20 - desc SCFGR"]
    #[inline(always)]
    pub const fn scfgr(&self) -> &Scfgr {
        &self.scfgr
    }
    #[doc = "0x24 - desc USBCKCFGR"]
    #[inline(always)]
    pub const fn usbckcfgr(&self) -> &Usbckcfgr {
        &self.usbckcfgr
    }
    #[doc = "0x26 - desc CKSWR"]
    #[inline(always)]
    pub const fn ckswr(&self) -> &Ckswr {
        &self.ckswr
    }
    #[doc = "0x2a - desc PLLCR"]
    #[inline(always)]
    pub const fn pllcr(&self) -> &Pllcr {
        &self.pllcr
    }
    #[doc = "0x2e - desc UPLLCR"]
    #[inline(always)]
    pub const fn upllcr(&self) -> &Upllcr {
        &self.upllcr
    }
    #[doc = "0x32 - desc XTALCR"]
    #[inline(always)]
    pub const fn xtalcr(&self) -> &Xtalcr {
        &self.xtalcr
    }
    #[doc = "0x36 - desc HRCCR"]
    #[inline(always)]
    pub const fn hrccr(&self) -> &Hrccr {
        &self.hrccr
    }
    #[doc = "0x38 - desc MRCCR"]
    #[inline(always)]
    pub const fn mrccr(&self) -> &Mrccr {
        &self.mrccr
    }
    #[doc = "0x3c - desc OSCSTBSR"]
    #[inline(always)]
    pub const fn oscstbsr(&self) -> &Oscstbsr {
        &self.oscstbsr
    }
    #[doc = "0x3d - desc MCO1CFGR"]
    #[inline(always)]
    pub const fn mco1cfgr(&self) -> &Mco1cfgr {
        &self.mco1cfgr
    }
    #[doc = "0x3e - desc MCO2CFGR"]
    #[inline(always)]
    pub const fn mco2cfgr(&self) -> &Mco2cfgr {
        &self.mco2cfgr
    }
    #[doc = "0x3f - desc TPIUCKCFGR"]
    #[inline(always)]
    pub const fn tpiuckcfgr(&self) -> &Tpiuckcfgr {
        &self.tpiuckcfgr
    }
    #[doc = "0x40 - desc XTALSTDCR"]
    #[inline(always)]
    pub const fn xtalstdcr(&self) -> &Xtalstdcr {
        &self.xtalstdcr
    }
    #[doc = "0x41 - desc XTALSTDSR"]
    #[inline(always)]
    pub const fn xtalstdsr(&self) -> &Xtalstdsr {
        &self.xtalstdsr
    }
    #[doc = "0x61 - desc MRCTRM"]
    #[inline(always)]
    pub const fn mrctrm(&self) -> &Mrctrm {
        &self.mrctrm
    }
    #[doc = "0x62 - desc HRCTRM"]
    #[inline(always)]
    pub const fn hrctrm(&self) -> &Hrctrm {
        &self.hrctrm
    }
    #[doc = "0xa2 - desc XTALSTBCR"]
    #[inline(always)]
    pub const fn xtalstbcr(&self) -> &Xtalstbcr {
        &self.xtalstbcr
    }
    #[doc = "0x100 - desc PLLCFGR"]
    #[inline(always)]
    pub const fn pllcfgr(&self) -> &Pllcfgr {
        &self.pllcfgr
    }
    #[doc = "0x104 - desc UPLLCFGR"]
    #[inline(always)]
    pub const fn upllcfgr(&self) -> &Upllcfgr {
        &self.upllcfgr
    }
    #[doc = "0x410 - desc XTALCFGR"]
    #[inline(always)]
    pub const fn xtalcfgr(&self) -> &Xtalcfgr {
        &self.xtalcfgr
    }
    #[doc = "0x420 - desc XTAL32CR"]
    #[inline(always)]
    pub const fn xtal32cr(&self) -> &Xtal32cr {
        &self.xtal32cr
    }
    #[doc = "0x421 - desc XTAL32CFGR"]
    #[inline(always)]
    pub const fn xtal32cfgr(&self) -> &Xtal32cfgr {
        &self.xtal32cfgr
    }
    #[doc = "0x425 - desc XTAL32NFR"]
    #[inline(always)]
    pub const fn xtal32nfr(&self) -> &Xtal32nfr {
        &self.xtal32nfr
    }
    #[doc = "0x427 - desc LRCCR"]
    #[inline(always)]
    pub const fn lrccr(&self) -> &Lrccr {
        &self.lrccr
    }
    #[doc = "0x429 - desc LRCTRM"]
    #[inline(always)]
    pub const fn lrctrm(&self) -> &Lrctrm {
        &self.lrctrm
    }
}
#[doc = "PERICKSEL (rw) register accessor: desc PERICKSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`pericksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pericksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pericksel`] module"]
#[doc(alias = "PERICKSEL")]
pub type Pericksel = crate::Reg<pericksel::PerickselSpec>;
#[doc = "desc PERICKSEL"]
pub mod pericksel;
#[doc = "I2SCKSEL (rw) register accessor: desc I2SCKSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`i2scksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2scksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2scksel`] module"]
#[doc(alias = "I2SCKSEL")]
pub type I2scksel = crate::Reg<i2scksel::I2sckselSpec>;
#[doc = "desc I2SCKSEL"]
pub mod i2scksel;
#[doc = "SCFGR (rw) register accessor: desc SCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`scfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfgr`] module"]
#[doc(alias = "SCFGR")]
pub type Scfgr = crate::Reg<scfgr::ScfgrSpec>;
#[doc = "desc SCFGR"]
pub mod scfgr;
#[doc = "USBCKCFGR (rw) register accessor: desc USBCKCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`usbckcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbckcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbckcfgr`] module"]
#[doc(alias = "USBCKCFGR")]
pub type Usbckcfgr = crate::Reg<usbckcfgr::UsbckcfgrSpec>;
#[doc = "desc USBCKCFGR"]
pub mod usbckcfgr;
#[doc = "CKSWR (rw) register accessor: desc CKSWR\n\nYou can [`read`](crate::Reg::read) this register and get [`ckswr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckswr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckswr`] module"]
#[doc(alias = "CKSWR")]
pub type Ckswr = crate::Reg<ckswr::CkswrSpec>;
#[doc = "desc CKSWR"]
pub mod ckswr;
#[doc = "PLLCR (rw) register accessor: desc PLLCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcr`] module"]
#[doc(alias = "PLLCR")]
pub type Pllcr = crate::Reg<pllcr::PllcrSpec>;
#[doc = "desc PLLCR"]
pub mod pllcr;
#[doc = "UPLLCR (rw) register accessor: desc UPLLCR\n\nYou can [`read`](crate::Reg::read) this register and get [`upllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`upllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upllcr`] module"]
#[doc(alias = "UPLLCR")]
pub type Upllcr = crate::Reg<upllcr::UpllcrSpec>;
#[doc = "desc UPLLCR"]
pub mod upllcr;
#[doc = "XTALCR (rw) register accessor: desc XTALCR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalcr`] module"]
#[doc(alias = "XTALCR")]
pub type Xtalcr = crate::Reg<xtalcr::XtalcrSpec>;
#[doc = "desc XTALCR"]
pub mod xtalcr;
#[doc = "HRCCR (rw) register accessor: desc HRCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`hrccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrccr`] module"]
#[doc(alias = "HRCCR")]
pub type Hrccr = crate::Reg<hrccr::HrccrSpec>;
#[doc = "desc HRCCR"]
pub mod hrccr;
#[doc = "MRCCR (rw) register accessor: desc MRCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`mrccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrccr`] module"]
#[doc(alias = "MRCCR")]
pub type Mrccr = crate::Reg<mrccr::MrccrSpec>;
#[doc = "desc MRCCR"]
pub mod mrccr;
#[doc = "OSCSTBSR (rw) register accessor: desc OSCSTBSR\n\nYou can [`read`](crate::Reg::read) this register and get [`oscstbsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscstbsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscstbsr`] module"]
#[doc(alias = "OSCSTBSR")]
pub type Oscstbsr = crate::Reg<oscstbsr::OscstbsrSpec>;
#[doc = "desc OSCSTBSR"]
pub mod oscstbsr;
#[doc = "MCO1CFGR (rw) register accessor: desc MCO1CFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`mco1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mco1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mco1cfgr`] module"]
#[doc(alias = "MCO1CFGR")]
pub type Mco1cfgr = crate::Reg<mco1cfgr::Mco1cfgrSpec>;
#[doc = "desc MCO1CFGR"]
pub mod mco1cfgr;
#[doc = "MCO2CFGR (rw) register accessor: desc MCO2CFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`mco2cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mco2cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mco2cfgr`] module"]
#[doc(alias = "MCO2CFGR")]
pub type Mco2cfgr = crate::Reg<mco2cfgr::Mco2cfgrSpec>;
#[doc = "desc MCO2CFGR"]
pub mod mco2cfgr;
#[doc = "TPIUCKCFGR (rw) register accessor: desc TPIUCKCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`tpiuckcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpiuckcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpiuckcfgr`] module"]
#[doc(alias = "TPIUCKCFGR")]
pub type Tpiuckcfgr = crate::Reg<tpiuckcfgr::TpiuckcfgrSpec>;
#[doc = "desc TPIUCKCFGR"]
pub mod tpiuckcfgr;
#[doc = "XTALSTDCR (rw) register accessor: desc XTALSTDCR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalstdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalstdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalstdcr`] module"]
#[doc(alias = "XTALSTDCR")]
pub type Xtalstdcr = crate::Reg<xtalstdcr::XtalstdcrSpec>;
#[doc = "desc XTALSTDCR"]
pub mod xtalstdcr;
#[doc = "XTALSTDSR (rw) register accessor: desc XTALSTDSR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalstdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalstdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalstdsr`] module"]
#[doc(alias = "XTALSTDSR")]
pub type Xtalstdsr = crate::Reg<xtalstdsr::XtalstdsrSpec>;
#[doc = "desc XTALSTDSR"]
pub mod xtalstdsr;
#[doc = "MRCTRM (rw) register accessor: desc MRCTRM\n\nYou can [`read`](crate::Reg::read) this register and get [`mrctrm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrctrm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrctrm`] module"]
#[doc(alias = "MRCTRM")]
pub type Mrctrm = crate::Reg<mrctrm::MrctrmSpec>;
#[doc = "desc MRCTRM"]
pub mod mrctrm;
#[doc = "HRCTRM (rw) register accessor: desc HRCTRM\n\nYou can [`read`](crate::Reg::read) this register and get [`hrctrm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrctrm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrctrm`] module"]
#[doc(alias = "HRCTRM")]
pub type Hrctrm = crate::Reg<hrctrm::HrctrmSpec>;
#[doc = "desc HRCTRM"]
pub mod hrctrm;
#[doc = "XTALSTBCR (rw) register accessor: desc XTALSTBCR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalstbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalstbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalstbcr`] module"]
#[doc(alias = "XTALSTBCR")]
pub type Xtalstbcr = crate::Reg<xtalstbcr::XtalstbcrSpec>;
#[doc = "desc XTALSTBCR"]
pub mod xtalstbcr;
#[doc = "PLLCFGR (rw) register accessor: desc PLLCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfgr`] module"]
#[doc(alias = "PLLCFGR")]
pub type Pllcfgr = crate::Reg<pllcfgr::PllcfgrSpec>;
#[doc = "desc PLLCFGR"]
pub mod pllcfgr;
#[doc = "UPLLCFGR (rw) register accessor: desc UPLLCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`upllcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`upllcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upllcfgr`] module"]
#[doc(alias = "UPLLCFGR")]
pub type Upllcfgr = crate::Reg<upllcfgr::UpllcfgrSpec>;
#[doc = "desc UPLLCFGR"]
pub mod upllcfgr;
#[doc = "XTALCFGR (rw) register accessor: desc XTALCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalcfgr`] module"]
#[doc(alias = "XTALCFGR")]
pub type Xtalcfgr = crate::Reg<xtalcfgr::XtalcfgrSpec>;
#[doc = "desc XTALCFGR"]
pub mod xtalcfgr;
#[doc = "XTAL32CR (rw) register accessor: desc XTAL32CR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal32cr`] module"]
#[doc(alias = "XTAL32CR")]
pub type Xtal32cr = crate::Reg<xtal32cr::Xtal32crSpec>;
#[doc = "desc XTAL32CR"]
pub mod xtal32cr;
#[doc = "XTAL32CFGR (rw) register accessor: desc XTAL32CFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal32cfgr`] module"]
#[doc(alias = "XTAL32CFGR")]
pub type Xtal32cfgr = crate::Reg<xtal32cfgr::Xtal32cfgrSpec>;
#[doc = "desc XTAL32CFGR"]
pub mod xtal32cfgr;
#[doc = "XTAL32NFR (rw) register accessor: desc XTAL32NFR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32nfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32nfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal32nfr`] module"]
#[doc(alias = "XTAL32NFR")]
pub type Xtal32nfr = crate::Reg<xtal32nfr::Xtal32nfrSpec>;
#[doc = "desc XTAL32NFR"]
pub mod xtal32nfr;
#[doc = "LRCCR (rw) register accessor: desc LRCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`lrccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lrccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lrccr`] module"]
#[doc(alias = "LRCCR")]
pub type Lrccr = crate::Reg<lrccr::LrccrSpec>;
#[doc = "desc LRCCR"]
pub mod lrccr;
#[doc = "LRCTRM (rw) register accessor: desc LRCTRM\n\nYou can [`read`](crate::Reg::read) this register and get [`lrctrm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lrctrm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lrctrm`] module"]
#[doc(alias = "LRCTRM")]
pub type Lrctrm = crate::Reg<lrctrm::LrctrmSpec>;
#[doc = "desc LRCTRM"]
pub mod lrctrm;
