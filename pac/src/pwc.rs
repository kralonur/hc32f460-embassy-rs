#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fcg0: Fcg0,
    fcg1: Fcg1,
    fcg2: Fcg2,
    fcg3: Fcg3,
    fcg0pc: Fcg0pc,
    _reserved5: [u8; 0x43ec],
    wktcr: Wktcr,
    _reserved6: [u8; 0x7c0a],
    stpmcr: Stpmcr,
    _reserved7: [u8; 0x06],
    rampc0: Rampc0,
    ramopm: Ramopm,
    _reserved9: [u8; 0xc6],
    pvdicr: Pvdicr,
    pvddsr: Pvddsr,
    _reserved11: [u8; 0x031c],
    fprc: Fprc,
    pwrc0: Pwrc0,
    pwrc1: Pwrc1,
    pwrc2: Pwrc2,
    pwrc3: Pwrc3,
    pdwke0: Pdwke0,
    pdwke1: Pdwke1,
    pdwke2: Pdwke2,
    pdwkes: Pdwkes,
    pdwkf0: Pdwkf0,
    pdwkf1: Pdwkf1,
    pwcmr: Pwcmr,
    _reserved23: [u8; 0x04],
    mdswcr: Mdswcr,
    _reserved24: [u8; 0x02],
    pvdcr0: Pvdcr0,
    pvdcr1: Pvdcr1,
    pvdfcr: Pvdfcr,
    pvdlcr: Pvdlcr,
    _reserved28: [u8; 0x15],
    xtal32cs: Xtal32cs,
}
impl RegisterBlock {
    #[doc = "0x00 - desc FCG0"]
    #[inline(always)]
    pub const fn fcg0(&self) -> &Fcg0 {
        &self.fcg0
    }
    #[doc = "0x04 - desc FCG1"]
    #[inline(always)]
    pub const fn fcg1(&self) -> &Fcg1 {
        &self.fcg1
    }
    #[doc = "0x08 - desc FCG2"]
    #[inline(always)]
    pub const fn fcg2(&self) -> &Fcg2 {
        &self.fcg2
    }
    #[doc = "0x0c - desc FCG3"]
    #[inline(always)]
    pub const fn fcg3(&self) -> &Fcg3 {
        &self.fcg3
    }
    #[doc = "0x10 - desc FCG0PC"]
    #[inline(always)]
    pub const fn fcg0pc(&self) -> &Fcg0pc {
        &self.fcg0pc
    }
    #[doc = "0x4400 - desc WKTCR"]
    #[inline(always)]
    pub const fn wktcr(&self) -> &Wktcr {
        &self.wktcr
    }
    #[doc = "0xc00c - desc STPMCR"]
    #[inline(always)]
    pub const fn stpmcr(&self) -> &Stpmcr {
        &self.stpmcr
    }
    #[doc = "0xc014 - desc RAMPC0"]
    #[inline(always)]
    pub const fn rampc0(&self) -> &Rampc0 {
        &self.rampc0
    }
    #[doc = "0xc018 - desc RAMOPM"]
    #[inline(always)]
    pub const fn ramopm(&self) -> &Ramopm {
        &self.ramopm
    }
    #[doc = "0xc0e0 - desc PVDICR"]
    #[inline(always)]
    pub const fn pvdicr(&self) -> &Pvdicr {
        &self.pvdicr
    }
    #[doc = "0xc0e1 - desc PVDDSR"]
    #[inline(always)]
    pub const fn pvddsr(&self) -> &Pvddsr {
        &self.pvddsr
    }
    #[doc = "0xc3fe - desc FPRC"]
    #[inline(always)]
    pub const fn fprc(&self) -> &Fprc {
        &self.fprc
    }
    #[doc = "0xc400 - desc PWRC0"]
    #[inline(always)]
    pub const fn pwrc0(&self) -> &Pwrc0 {
        &self.pwrc0
    }
    #[doc = "0xc401 - desc PWRC1"]
    #[inline(always)]
    pub const fn pwrc1(&self) -> &Pwrc1 {
        &self.pwrc1
    }
    #[doc = "0xc402 - desc PWRC2"]
    #[inline(always)]
    pub const fn pwrc2(&self) -> &Pwrc2 {
        &self.pwrc2
    }
    #[doc = "0xc403 - desc PWRC3"]
    #[inline(always)]
    pub const fn pwrc3(&self) -> &Pwrc3 {
        &self.pwrc3
    }
    #[doc = "0xc404 - desc PDWKE0"]
    #[inline(always)]
    pub const fn pdwke0(&self) -> &Pdwke0 {
        &self.pdwke0
    }
    #[doc = "0xc405 - desc PDWKE1"]
    #[inline(always)]
    pub const fn pdwke1(&self) -> &Pdwke1 {
        &self.pdwke1
    }
    #[doc = "0xc406 - desc PDWKE2"]
    #[inline(always)]
    pub const fn pdwke2(&self) -> &Pdwke2 {
        &self.pdwke2
    }
    #[doc = "0xc407 - desc PDWKES"]
    #[inline(always)]
    pub const fn pdwkes(&self) -> &Pdwkes {
        &self.pdwkes
    }
    #[doc = "0xc408 - desc PDWKF0"]
    #[inline(always)]
    pub const fn pdwkf0(&self) -> &Pdwkf0 {
        &self.pdwkf0
    }
    #[doc = "0xc409 - desc PDWKF1"]
    #[inline(always)]
    pub const fn pdwkf1(&self) -> &Pdwkf1 {
        &self.pdwkf1
    }
    #[doc = "0xc40a - desc PWCMR"]
    #[inline(always)]
    pub const fn pwcmr(&self) -> &Pwcmr {
        &self.pwcmr
    }
    #[doc = "0xc40f - desc MDSWCR"]
    #[inline(always)]
    pub const fn mdswcr(&self) -> &Mdswcr {
        &self.mdswcr
    }
    #[doc = "0xc412 - desc PVDCR0"]
    #[inline(always)]
    pub const fn pvdcr0(&self) -> &Pvdcr0 {
        &self.pvdcr0
    }
    #[doc = "0xc413 - desc PVDCR1"]
    #[inline(always)]
    pub const fn pvdcr1(&self) -> &Pvdcr1 {
        &self.pvdcr1
    }
    #[doc = "0xc414 - desc PVDFCR"]
    #[inline(always)]
    pub const fn pvdfcr(&self) -> &Pvdfcr {
        &self.pvdfcr
    }
    #[doc = "0xc415 - desc PVDLCR"]
    #[inline(always)]
    pub const fn pvdlcr(&self) -> &Pvdlcr {
        &self.pvdlcr
    }
    #[doc = "0xc42b - desc XTAL32CS"]
    #[inline(always)]
    pub const fn xtal32cs(&self) -> &Xtal32cs {
        &self.xtal32cs
    }
}
#[doc = "FCG0 (rw) register accessor: desc FCG0\n\nYou can [`read`](crate::Reg::read) this register and get [`fcg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcg0`] module"]
#[doc(alias = "FCG0")]
pub type Fcg0 = crate::Reg<fcg0::Fcg0Spec>;
#[doc = "desc FCG0"]
pub mod fcg0;
#[doc = "FCG1 (rw) register accessor: desc FCG1\n\nYou can [`read`](crate::Reg::read) this register and get [`fcg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcg1`] module"]
#[doc(alias = "FCG1")]
pub type Fcg1 = crate::Reg<fcg1::Fcg1Spec>;
#[doc = "desc FCG1"]
pub mod fcg1;
#[doc = "FCG2 (rw) register accessor: desc FCG2\n\nYou can [`read`](crate::Reg::read) this register and get [`fcg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcg2`] module"]
#[doc(alias = "FCG2")]
pub type Fcg2 = crate::Reg<fcg2::Fcg2Spec>;
#[doc = "desc FCG2"]
pub mod fcg2;
#[doc = "FCG3 (rw) register accessor: desc FCG3\n\nYou can [`read`](crate::Reg::read) this register and get [`fcg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcg3`] module"]
#[doc(alias = "FCG3")]
pub type Fcg3 = crate::Reg<fcg3::Fcg3Spec>;
#[doc = "desc FCG3"]
pub mod fcg3;
#[doc = "FCG0PC (rw) register accessor: desc FCG0PC\n\nYou can [`read`](crate::Reg::read) this register and get [`fcg0pc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcg0pc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcg0pc`] module"]
#[doc(alias = "FCG0PC")]
pub type Fcg0pc = crate::Reg<fcg0pc::Fcg0pcSpec>;
#[doc = "desc FCG0PC"]
pub mod fcg0pc;
#[doc = "WKTCR (rw) register accessor: desc WKTCR\n\nYou can [`read`](crate::Reg::read) this register and get [`wktcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wktcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wktcr`] module"]
#[doc(alias = "WKTCR")]
pub type Wktcr = crate::Reg<wktcr::WktcrSpec>;
#[doc = "desc WKTCR"]
pub mod wktcr;
#[doc = "STPMCR (rw) register accessor: desc STPMCR\n\nYou can [`read`](crate::Reg::read) this register and get [`stpmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stpmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stpmcr`] module"]
#[doc(alias = "STPMCR")]
pub type Stpmcr = crate::Reg<stpmcr::StpmcrSpec>;
#[doc = "desc STPMCR"]
pub mod stpmcr;
#[doc = "RAMPC0 (rw) register accessor: desc RAMPC0\n\nYou can [`read`](crate::Reg::read) this register and get [`rampc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rampc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rampc0`] module"]
#[doc(alias = "RAMPC0")]
pub type Rampc0 = crate::Reg<rampc0::Rampc0Spec>;
#[doc = "desc RAMPC0"]
pub mod rampc0;
#[doc = "RAMOPM (rw) register accessor: desc RAMOPM\n\nYou can [`read`](crate::Reg::read) this register and get [`ramopm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramopm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ramopm`] module"]
#[doc(alias = "RAMOPM")]
pub type Ramopm = crate::Reg<ramopm::RamopmSpec>;
#[doc = "desc RAMOPM"]
pub mod ramopm;
#[doc = "PVDICR (rw) register accessor: desc PVDICR\n\nYou can [`read`](crate::Reg::read) this register and get [`pvdicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvdicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvdicr`] module"]
#[doc(alias = "PVDICR")]
pub type Pvdicr = crate::Reg<pvdicr::PvdicrSpec>;
#[doc = "desc PVDICR"]
pub mod pvdicr;
#[doc = "PVDDSR (rw) register accessor: desc PVDDSR\n\nYou can [`read`](crate::Reg::read) this register and get [`pvddsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvddsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvddsr`] module"]
#[doc(alias = "PVDDSR")]
pub type Pvddsr = crate::Reg<pvddsr::PvddsrSpec>;
#[doc = "desc PVDDSR"]
pub mod pvddsr;
#[doc = "FPRC (rw) register accessor: desc FPRC\n\nYou can [`read`](crate::Reg::read) this register and get [`fprc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fprc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fprc`] module"]
#[doc(alias = "FPRC")]
pub type Fprc = crate::Reg<fprc::FprcSpec>;
#[doc = "desc FPRC"]
pub mod fprc;
#[doc = "PWRC0 (rw) register accessor: desc PWRC0\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrc0`] module"]
#[doc(alias = "PWRC0")]
pub type Pwrc0 = crate::Reg<pwrc0::Pwrc0Spec>;
#[doc = "desc PWRC0"]
pub mod pwrc0;
#[doc = "PWRC1 (rw) register accessor: desc PWRC1\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrc1`] module"]
#[doc(alias = "PWRC1")]
pub type Pwrc1 = crate::Reg<pwrc1::Pwrc1Spec>;
#[doc = "desc PWRC1"]
pub mod pwrc1;
#[doc = "PWRC2 (rw) register accessor: desc PWRC2\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrc2`] module"]
#[doc(alias = "PWRC2")]
pub type Pwrc2 = crate::Reg<pwrc2::Pwrc2Spec>;
#[doc = "desc PWRC2"]
pub mod pwrc2;
#[doc = "PWRC3 (rw) register accessor: desc PWRC3\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrc3`] module"]
#[doc(alias = "PWRC3")]
pub type Pwrc3 = crate::Reg<pwrc3::Pwrc3Spec>;
#[doc = "desc PWRC3"]
pub mod pwrc3;
#[doc = "PDWKE0 (rw) register accessor: desc PDWKE0\n\nYou can [`read`](crate::Reg::read) this register and get [`pdwke0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdwke0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdwke0`] module"]
#[doc(alias = "PDWKE0")]
pub type Pdwke0 = crate::Reg<pdwke0::Pdwke0Spec>;
#[doc = "desc PDWKE0"]
pub mod pdwke0;
#[doc = "PDWKE1 (rw) register accessor: desc PDWKE1\n\nYou can [`read`](crate::Reg::read) this register and get [`pdwke1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdwke1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdwke1`] module"]
#[doc(alias = "PDWKE1")]
pub type Pdwke1 = crate::Reg<pdwke1::Pdwke1Spec>;
#[doc = "desc PDWKE1"]
pub mod pdwke1;
#[doc = "PDWKE2 (rw) register accessor: desc PDWKE2\n\nYou can [`read`](crate::Reg::read) this register and get [`pdwke2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdwke2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdwke2`] module"]
#[doc(alias = "PDWKE2")]
pub type Pdwke2 = crate::Reg<pdwke2::Pdwke2Spec>;
#[doc = "desc PDWKE2"]
pub mod pdwke2;
#[doc = "PDWKES (rw) register accessor: desc PDWKES\n\nYou can [`read`](crate::Reg::read) this register and get [`pdwkes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdwkes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdwkes`] module"]
#[doc(alias = "PDWKES")]
pub type Pdwkes = crate::Reg<pdwkes::PdwkesSpec>;
#[doc = "desc PDWKES"]
pub mod pdwkes;
#[doc = "PDWKF0 (rw) register accessor: desc PDWKF0\n\nYou can [`read`](crate::Reg::read) this register and get [`pdwkf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdwkf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdwkf0`] module"]
#[doc(alias = "PDWKF0")]
pub type Pdwkf0 = crate::Reg<pdwkf0::Pdwkf0Spec>;
#[doc = "desc PDWKF0"]
pub mod pdwkf0;
#[doc = "PDWKF1 (rw) register accessor: desc PDWKF1\n\nYou can [`read`](crate::Reg::read) this register and get [`pdwkf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdwkf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdwkf1`] module"]
#[doc(alias = "PDWKF1")]
pub type Pdwkf1 = crate::Reg<pdwkf1::Pdwkf1Spec>;
#[doc = "desc PDWKF1"]
pub mod pdwkf1;
#[doc = "PWCMR (rw) register accessor: desc PWCMR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwcmr`] module"]
#[doc(alias = "PWCMR")]
pub type Pwcmr = crate::Reg<pwcmr::PwcmrSpec>;
#[doc = "desc PWCMR"]
pub mod pwcmr;
#[doc = "MDSWCR (rw) register accessor: desc MDSWCR\n\nYou can [`read`](crate::Reg::read) this register and get [`mdswcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdswcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdswcr`] module"]
#[doc(alias = "MDSWCR")]
pub type Mdswcr = crate::Reg<mdswcr::MdswcrSpec>;
#[doc = "desc MDSWCR"]
pub mod mdswcr;
#[doc = "PVDCR0 (rw) register accessor: desc PVDCR0\n\nYou can [`read`](crate::Reg::read) this register and get [`pvdcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvdcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvdcr0`] module"]
#[doc(alias = "PVDCR0")]
pub type Pvdcr0 = crate::Reg<pvdcr0::Pvdcr0Spec>;
#[doc = "desc PVDCR0"]
pub mod pvdcr0;
#[doc = "PVDCR1 (rw) register accessor: desc PVDCR1\n\nYou can [`read`](crate::Reg::read) this register and get [`pvdcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvdcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvdcr1`] module"]
#[doc(alias = "PVDCR1")]
pub type Pvdcr1 = crate::Reg<pvdcr1::Pvdcr1Spec>;
#[doc = "desc PVDCR1"]
pub mod pvdcr1;
#[doc = "PVDFCR (rw) register accessor: desc PVDFCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pvdfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvdfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvdfcr`] module"]
#[doc(alias = "PVDFCR")]
pub type Pvdfcr = crate::Reg<pvdfcr::PvdfcrSpec>;
#[doc = "desc PVDFCR"]
pub mod pvdfcr;
#[doc = "PVDLCR (rw) register accessor: desc PVDLCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pvdlcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvdlcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvdlcr`] module"]
#[doc(alias = "PVDLCR")]
pub type Pvdlcr = crate::Reg<pvdlcr::PvdlcrSpec>;
#[doc = "desc PVDLCR"]
pub mod pvdlcr;
#[doc = "XTAL32CS (rw) register accessor: desc XTAL32CS\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal32cs`] module"]
#[doc(alias = "XTAL32CS")]
pub type Xtal32cs = crate::Reg<xtal32cs::Xtal32csSpec>;
#[doc = "desc XTAL32CS"]
pub mod xtal32cs;
