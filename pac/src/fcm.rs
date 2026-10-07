#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lvr: Lvr,
    uvr: Uvr,
    cntr: Cntr,
    str: Str,
    mccr: Mccr,
    rccr: Rccr,
    rier: Rier,
    sr: Sr,
    clr: Clr,
}
impl RegisterBlock {
    #[doc = "0x00 - desc LVR"]
    #[inline(always)]
    pub const fn lvr(&self) -> &Lvr {
        &self.lvr
    }
    #[doc = "0x04 - desc UVR"]
    #[inline(always)]
    pub const fn uvr(&self) -> &Uvr {
        &self.uvr
    }
    #[doc = "0x08 - desc CNTR"]
    #[inline(always)]
    pub const fn cntr(&self) -> &Cntr {
        &self.cntr
    }
    #[doc = "0x0c - desc STR"]
    #[inline(always)]
    pub const fn str(&self) -> &Str {
        &self.str
    }
    #[doc = "0x10 - desc MCCR"]
    #[inline(always)]
    pub const fn mccr(&self) -> &Mccr {
        &self.mccr
    }
    #[doc = "0x14 - desc RCCR"]
    #[inline(always)]
    pub const fn rccr(&self) -> &Rccr {
        &self.rccr
    }
    #[doc = "0x18 - desc RIER"]
    #[inline(always)]
    pub const fn rier(&self) -> &Rier {
        &self.rier
    }
    #[doc = "0x1c - desc SR"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x20 - desc CLR"]
    #[inline(always)]
    pub const fn clr(&self) -> &Clr {
        &self.clr
    }
}
#[doc = "LVR (rw) register accessor: desc LVR\n\nYou can [`read`](crate::Reg::read) this register and get [`lvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvr`] module"]
#[doc(alias = "LVR")]
pub type Lvr = crate::Reg<lvr::LvrSpec>;
#[doc = "desc LVR"]
pub mod lvr;
#[doc = "UVR (rw) register accessor: desc UVR\n\nYou can [`read`](crate::Reg::read) this register and get [`uvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uvr`] module"]
#[doc(alias = "UVR")]
pub type Uvr = crate::Reg<uvr::UvrSpec>;
#[doc = "desc UVR"]
pub mod uvr;
#[doc = "CNTR (r) register accessor: desc CNTR\n\nYou can [`read`](crate::Reg::read) this register and get [`cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`] module"]
#[doc(alias = "CNTR")]
pub type Cntr = crate::Reg<cntr::CntrSpec>;
#[doc = "desc CNTR"]
pub mod cntr;
#[doc = "STR (rw) register accessor: desc STR\n\nYou can [`read`](crate::Reg::read) this register and get [`str::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`str::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@str`] module"]
#[doc(alias = "STR")]
pub type Str = crate::Reg<str::StrSpec>;
#[doc = "desc STR"]
pub mod str;
#[doc = "MCCR (rw) register accessor: desc MCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`mccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mccr`] module"]
#[doc(alias = "MCCR")]
pub type Mccr = crate::Reg<mccr::MccrSpec>;
#[doc = "desc MCCR"]
pub mod mccr;
#[doc = "RCCR (rw) register accessor: desc RCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`rccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rccr`] module"]
#[doc(alias = "RCCR")]
pub type Rccr = crate::Reg<rccr::RccrSpec>;
#[doc = "desc RCCR"]
pub mod rccr;
#[doc = "RIER (rw) register accessor: desc RIER\n\nYou can [`read`](crate::Reg::read) this register and get [`rier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rier`] module"]
#[doc(alias = "RIER")]
pub type Rier = crate::Reg<rier::RierSpec>;
#[doc = "desc RIER"]
pub mod rier;
#[doc = "SR (r) register accessor: desc SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "desc SR"]
pub mod sr;
#[doc = "CLR (w) register accessor: desc CLR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`] module"]
#[doc(alias = "CLR")]
pub type Clr = crate::Reg<clr::ClrSpec>;
#[doc = "desc CLR"]
pub mod clr;
