#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    faprt: Faprt,
    fstp: Fstp,
    frmc: Frmc,
    fwmc: Fwmc,
    fsr: Fsr,
    fsclr: Fsclr,
    fite: Fite,
    fswp: Fswp,
    fpmtsw: Fpmtsw,
    fpmtew: Fpmtew,
    _reserved10: [u8; 0x28],
    uqid0: Uqid0,
    uqid1: Uqid1,
    uqid2: Uqid2,
    _reserved13: [u8; 0xa4],
    mmf_remprt: MmfRemprt,
    mmf_remcr0: MmfRemcr0,
    mmf_remcr1: MmfRemcr1,
}
impl RegisterBlock {
    #[doc = "0x00 - desc FAPRT"]
    #[inline(always)]
    pub const fn faprt(&self) -> &Faprt {
        &self.faprt
    }
    #[doc = "0x04 - desc FSTP"]
    #[inline(always)]
    pub const fn fstp(&self) -> &Fstp {
        &self.fstp
    }
    #[doc = "0x08 - desc FRMC"]
    #[inline(always)]
    pub const fn frmc(&self) -> &Frmc {
        &self.frmc
    }
    #[doc = "0x0c - desc FWMC"]
    #[inline(always)]
    pub const fn fwmc(&self) -> &Fwmc {
        &self.fwmc
    }
    #[doc = "0x10 - desc FSR"]
    #[inline(always)]
    pub const fn fsr(&self) -> &Fsr {
        &self.fsr
    }
    #[doc = "0x14 - desc FSCLR"]
    #[inline(always)]
    pub const fn fsclr(&self) -> &Fsclr {
        &self.fsclr
    }
    #[doc = "0x18 - desc FITE"]
    #[inline(always)]
    pub const fn fite(&self) -> &Fite {
        &self.fite
    }
    #[doc = "0x1c - desc FSWP"]
    #[inline(always)]
    pub const fn fswp(&self) -> &Fswp {
        &self.fswp
    }
    #[doc = "0x20 - desc FPMTSW"]
    #[inline(always)]
    pub const fn fpmtsw(&self) -> &Fpmtsw {
        &self.fpmtsw
    }
    #[doc = "0x24 - desc FPMTEW"]
    #[inline(always)]
    pub const fn fpmtew(&self) -> &Fpmtew {
        &self.fpmtew
    }
    #[doc = "0x50 - desc UQID0"]
    #[inline(always)]
    pub const fn uqid0(&self) -> &Uqid0 {
        &self.uqid0
    }
    #[doc = "0x54 - desc UQID1"]
    #[inline(always)]
    pub const fn uqid1(&self) -> &Uqid1 {
        &self.uqid1
    }
    #[doc = "0x58 - desc UQID2"]
    #[inline(always)]
    pub const fn uqid2(&self) -> &Uqid2 {
        &self.uqid2
    }
    #[doc = "0x100 - desc MMF_REMPRT"]
    #[inline(always)]
    pub const fn mmf_remprt(&self) -> &MmfRemprt {
        &self.mmf_remprt
    }
    #[doc = "0x104 - desc MMF_REMCR0"]
    #[inline(always)]
    pub const fn mmf_remcr0(&self) -> &MmfRemcr0 {
        &self.mmf_remcr0
    }
    #[doc = "0x108 - desc MMF_REMCR1"]
    #[inline(always)]
    pub const fn mmf_remcr1(&self) -> &MmfRemcr1 {
        &self.mmf_remcr1
    }
}
#[doc = "FAPRT (rw) register accessor: desc FAPRT\n\nYou can [`read`](crate::Reg::read) this register and get [`faprt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faprt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@faprt`] module"]
#[doc(alias = "FAPRT")]
pub type Faprt = crate::Reg<faprt::FaprtSpec>;
#[doc = "desc FAPRT"]
pub mod faprt;
#[doc = "FSTP (rw) register accessor: desc FSTP\n\nYou can [`read`](crate::Reg::read) this register and get [`fstp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fstp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fstp`] module"]
#[doc(alias = "FSTP")]
pub type Fstp = crate::Reg<fstp::FstpSpec>;
#[doc = "desc FSTP"]
pub mod fstp;
#[doc = "FRMC (rw) register accessor: desc FRMC\n\nYou can [`read`](crate::Reg::read) this register and get [`frmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frmc`] module"]
#[doc(alias = "FRMC")]
pub type Frmc = crate::Reg<frmc::FrmcSpec>;
#[doc = "desc FRMC"]
pub mod frmc;
#[doc = "FWMC (rw) register accessor: desc FWMC\n\nYou can [`read`](crate::Reg::read) this register and get [`fwmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwmc`] module"]
#[doc(alias = "FWMC")]
pub type Fwmc = crate::Reg<fwmc::FwmcSpec>;
#[doc = "desc FWMC"]
pub mod fwmc;
#[doc = "FSR (r) register accessor: desc FSR\n\nYou can [`read`](crate::Reg::read) this register and get [`fsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsr`] module"]
#[doc(alias = "FSR")]
pub type Fsr = crate::Reg<fsr::FsrSpec>;
#[doc = "desc FSR"]
pub mod fsr;
#[doc = "FSCLR (rw) register accessor: desc FSCLR\n\nYou can [`read`](crate::Reg::read) this register and get [`fsclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsclr`] module"]
#[doc(alias = "FSCLR")]
pub type Fsclr = crate::Reg<fsclr::FsclrSpec>;
#[doc = "desc FSCLR"]
pub mod fsclr;
#[doc = "FITE (rw) register accessor: desc FITE\n\nYou can [`read`](crate::Reg::read) this register and get [`fite::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fite::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fite`] module"]
#[doc(alias = "FITE")]
pub type Fite = crate::Reg<fite::FiteSpec>;
#[doc = "desc FITE"]
pub mod fite;
#[doc = "FSWP (r) register accessor: desc FSWP\n\nYou can [`read`](crate::Reg::read) this register and get [`fswp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fswp`] module"]
#[doc(alias = "FSWP")]
pub type Fswp = crate::Reg<fswp::FswpSpec>;
#[doc = "desc FSWP"]
pub mod fswp;
#[doc = "FPMTSW (rw) register accessor: desc FPMTSW\n\nYou can [`read`](crate::Reg::read) this register and get [`fpmtsw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpmtsw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpmtsw`] module"]
#[doc(alias = "FPMTSW")]
pub type Fpmtsw = crate::Reg<fpmtsw::FpmtswSpec>;
#[doc = "desc FPMTSW"]
pub mod fpmtsw;
#[doc = "FPMTEW (rw) register accessor: desc FPMTEW\n\nYou can [`read`](crate::Reg::read) this register and get [`fpmtew::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpmtew::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpmtew`] module"]
#[doc(alias = "FPMTEW")]
pub type Fpmtew = crate::Reg<fpmtew::FpmtewSpec>;
#[doc = "desc FPMTEW"]
pub mod fpmtew;
#[doc = "UQID0 (r) register accessor: desc UQID0\n\nYou can [`read`](crate::Reg::read) this register and get [`uqid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uqid0`] module"]
#[doc(alias = "UQID0")]
pub type Uqid0 = crate::Reg<uqid0::Uqid0Spec>;
#[doc = "desc UQID0"]
pub mod uqid0;
#[doc = "UQID1 (r) register accessor: desc UQID1\n\nYou can [`read`](crate::Reg::read) this register and get [`uqid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uqid1`] module"]
#[doc(alias = "UQID1")]
pub type Uqid1 = crate::Reg<uqid1::Uqid1Spec>;
#[doc = "desc UQID1"]
pub mod uqid1;
#[doc = "UQID2 (r) register accessor: desc UQID2\n\nYou can [`read`](crate::Reg::read) this register and get [`uqid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uqid2`] module"]
#[doc(alias = "UQID2")]
pub type Uqid2 = crate::Reg<uqid2::Uqid2Spec>;
#[doc = "desc UQID2"]
pub mod uqid2;
#[doc = "MMF_REMPRT (rw) register accessor: desc MMF_REMPRT\n\nYou can [`read`](crate::Reg::read) this register and get [`mmf_remprt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmf_remprt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmf_remprt`] module"]
#[doc(alias = "MMF_REMPRT")]
pub type MmfRemprt = crate::Reg<mmf_remprt::MmfRemprtSpec>;
#[doc = "desc MMF_REMPRT"]
pub mod mmf_remprt;
#[doc = "MMF_REMCR0 (rw) register accessor: desc MMF_REMCR0\n\nYou can [`read`](crate::Reg::read) this register and get [`mmf_remcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmf_remcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmf_remcr0`] module"]
#[doc(alias = "MMF_REMCR0")]
pub type MmfRemcr0 = crate::Reg<mmf_remcr0::MmfRemcr0Spec>;
#[doc = "desc MMF_REMCR0"]
pub mod mmf_remcr0;
#[doc = "MMF_REMCR1 (rw) register accessor: desc MMF_REMCR1\n\nYou can [`read`](crate::Reg::read) this register and get [`mmf_remcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmf_remcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmf_remcr1`] module"]
#[doc(alias = "MMF_REMCR1")]
pub type MmfRemcr1 = crate::Reg<mmf_remcr1::MmfRemcr1Spec>;
#[doc = "desc MMF_REMCR1"]
pub mod mmf_remcr1;
