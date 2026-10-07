#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wtcr: Wtcr,
    wtpr: Wtpr,
    ckcr: Ckcr,
    ckpr: Ckpr,
    cksr: Cksr,
}
impl RegisterBlock {
    #[doc = "0x00 - desc WTCR"]
    #[inline(always)]
    pub const fn wtcr(&self) -> &Wtcr {
        &self.wtcr
    }
    #[doc = "0x04 - desc WTPR"]
    #[inline(always)]
    pub const fn wtpr(&self) -> &Wtpr {
        &self.wtpr
    }
    #[doc = "0x08 - desc CKCR"]
    #[inline(always)]
    pub const fn ckcr(&self) -> &Ckcr {
        &self.ckcr
    }
    #[doc = "0x0c - desc CKPR"]
    #[inline(always)]
    pub const fn ckpr(&self) -> &Ckpr {
        &self.ckpr
    }
    #[doc = "0x10 - desc CKSR"]
    #[inline(always)]
    pub const fn cksr(&self) -> &Cksr {
        &self.cksr
    }
}
#[doc = "WTCR (rw) register accessor: desc WTCR\n\nYou can [`read`](crate::Reg::read) this register and get [`wtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtcr`] module"]
#[doc(alias = "WTCR")]
pub type Wtcr = crate::Reg<wtcr::WtcrSpec>;
#[doc = "desc WTCR"]
pub mod wtcr;
#[doc = "WTPR (rw) register accessor: desc WTPR\n\nYou can [`read`](crate::Reg::read) this register and get [`wtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtpr`] module"]
#[doc(alias = "WTPR")]
pub type Wtpr = crate::Reg<wtpr::WtprSpec>;
#[doc = "desc WTPR"]
pub mod wtpr;
#[doc = "CKCR (rw) register accessor: desc CKCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckcr`] module"]
#[doc(alias = "CKCR")]
pub type Ckcr = crate::Reg<ckcr::CkcrSpec>;
#[doc = "desc CKCR"]
pub mod ckcr;
#[doc = "CKPR (rw) register accessor: desc CKPR\n\nYou can [`read`](crate::Reg::read) this register and get [`ckpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckpr`] module"]
#[doc(alias = "CKPR")]
pub type Ckpr = crate::Reg<ckpr::CkprSpec>;
#[doc = "desc CKPR"]
pub mod ckpr;
#[doc = "CKSR (rw) register accessor: desc CKSR\n\nYou can [`read`](crate::Reg::read) this register and get [`cksr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cksr`] module"]
#[doc(alias = "CKSR")]
pub type Cksr = crate::Reg<cksr::CksrSpec>;
#[doc = "desc CKSR"]
pub mod cksr;
