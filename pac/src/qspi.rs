#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    cscr: Cscr,
    fcr: Fcr,
    sr: Sr,
    dcom: Dcom,
    ccmd: Ccmd,
    xcmd: Xcmd,
    _reserved7: [u8; 0x08],
    sr2: Sr2,
    _reserved8: [u8; 0x07dc],
    exar: Exar,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - desc CSCR"]
    #[inline(always)]
    pub const fn cscr(&self) -> &Cscr {
        &self.cscr
    }
    #[doc = "0x08 - desc FCR"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        &self.fcr
    }
    #[doc = "0x0c - desc SR"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - desc DCOM"]
    #[inline(always)]
    pub const fn dcom(&self) -> &Dcom {
        &self.dcom
    }
    #[doc = "0x14 - desc CCMD"]
    #[inline(always)]
    pub const fn ccmd(&self) -> &Ccmd {
        &self.ccmd
    }
    #[doc = "0x18 - desc XCMD"]
    #[inline(always)]
    pub const fn xcmd(&self) -> &Xcmd {
        &self.xcmd
    }
    #[doc = "0x24 - desc SR2"]
    #[inline(always)]
    pub const fn sr2(&self) -> &Sr2 {
        &self.sr2
    }
    #[doc = "0x804 - desc EXAR"]
    #[inline(always)]
    pub const fn exar(&self) -> &Exar {
        &self.exar
    }
}
#[doc = "CR (rw) register accessor: desc CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "CSCR (rw) register accessor: desc CSCR\n\nYou can [`read`](crate::Reg::read) this register and get [`cscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cscr`] module"]
#[doc(alias = "CSCR")]
pub type Cscr = crate::Reg<cscr::CscrSpec>;
#[doc = "desc CSCR"]
pub mod cscr;
#[doc = "FCR (rw) register accessor: desc FCR\n\nYou can [`read`](crate::Reg::read) this register and get [`fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`] module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "desc FCR"]
pub mod fcr;
#[doc = "SR (rw) register accessor: desc SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "desc SR"]
pub mod sr;
#[doc = "DCOM (rw) register accessor: desc DCOM\n\nYou can [`read`](crate::Reg::read) this register and get [`dcom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcom`] module"]
#[doc(alias = "DCOM")]
pub type Dcom = crate::Reg<dcom::DcomSpec>;
#[doc = "desc DCOM"]
pub mod dcom;
#[doc = "CCMD (rw) register accessor: desc CCMD\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmd`] module"]
#[doc(alias = "CCMD")]
pub type Ccmd = crate::Reg<ccmd::CcmdSpec>;
#[doc = "desc CCMD"]
pub mod ccmd;
#[doc = "XCMD (rw) register accessor: desc XCMD\n\nYou can [`read`](crate::Reg::read) this register and get [`xcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcmd`] module"]
#[doc(alias = "XCMD")]
pub type Xcmd = crate::Reg<xcmd::XcmdSpec>;
#[doc = "desc XCMD"]
pub mod xcmd;
#[doc = "SR2 (w) register accessor: desc SR2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr2`] module"]
#[doc(alias = "SR2")]
pub type Sr2 = crate::Reg<sr2::Sr2Spec>;
#[doc = "desc SR2"]
pub mod sr2;
#[doc = "EXAR (rw) register accessor: desc EXAR\n\nYou can [`read`](crate::Reg::read) this register and get [`exar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exar`] module"]
#[doc(alias = "EXAR")]
pub type Exar = crate::Reg<exar::ExarSpec>;
#[doc = "desc EXAR"]
pub mod exar;
