#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: Cr1,
    cr2: Cr2,
    cr3: Cr3,
    cr4: Cr4,
    slr0: Slr0,
    slr1: Slr1,
    sltr: Sltr,
    sr: Sr,
    clr: Clr,
    dtr: Dtr,
    _reserved10: [u8; 0x03],
    drr: Drr,
    _reserved11: [u8; 0x03],
    ccr: Ccr,
    fltr: Fltr,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CR1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x04 - desc CR2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x08 - desc CR3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &Cr3 {
        &self.cr3
    }
    #[doc = "0x0c - desc CR4"]
    #[inline(always)]
    pub const fn cr4(&self) -> &Cr4 {
        &self.cr4
    }
    #[doc = "0x10 - desc SLR0"]
    #[inline(always)]
    pub const fn slr0(&self) -> &Slr0 {
        &self.slr0
    }
    #[doc = "0x14 - desc SLR1"]
    #[inline(always)]
    pub const fn slr1(&self) -> &Slr1 {
        &self.slr1
    }
    #[doc = "0x18 - desc SLTR"]
    #[inline(always)]
    pub const fn sltr(&self) -> &Sltr {
        &self.sltr
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
    #[doc = "0x24 - desc DTR"]
    #[inline(always)]
    pub const fn dtr(&self) -> &Dtr {
        &self.dtr
    }
    #[doc = "0x28 - desc DRR"]
    #[inline(always)]
    pub const fn drr(&self) -> &Drr {
        &self.drr
    }
    #[doc = "0x2c - desc CCR"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x30 - desc FLTR"]
    #[inline(always)]
    pub const fn fltr(&self) -> &Fltr {
        &self.fltr
    }
}
#[doc = "CR1 (rw) register accessor: desc CR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "desc CR1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: desc CR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "desc CR2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: desc CR3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`] module"]
#[doc(alias = "CR3")]
pub type Cr3 = crate::Reg<cr3::Cr3Spec>;
#[doc = "desc CR3"]
pub mod cr3;
#[doc = "CR4 (rw) register accessor: desc CR4\n\nYou can [`read`](crate::Reg::read) this register and get [`cr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr4`] module"]
#[doc(alias = "CR4")]
pub type Cr4 = crate::Reg<cr4::Cr4Spec>;
#[doc = "desc CR4"]
pub mod cr4;
#[doc = "SLR0 (rw) register accessor: desc SLR0\n\nYou can [`read`](crate::Reg::read) this register and get [`slr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slr0`] module"]
#[doc(alias = "SLR0")]
pub type Slr0 = crate::Reg<slr0::Slr0Spec>;
#[doc = "desc SLR0"]
pub mod slr0;
#[doc = "SLR1 (rw) register accessor: desc SLR1\n\nYou can [`read`](crate::Reg::read) this register and get [`slr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slr1`] module"]
#[doc(alias = "SLR1")]
pub type Slr1 = crate::Reg<slr1::Slr1Spec>;
#[doc = "desc SLR1"]
pub mod slr1;
#[doc = "SLTR (rw) register accessor: desc SLTR\n\nYou can [`read`](crate::Reg::read) this register and get [`sltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sltr`] module"]
#[doc(alias = "SLTR")]
pub type Sltr = crate::Reg<sltr::SltrSpec>;
#[doc = "desc SLTR"]
pub mod sltr;
#[doc = "SR (rw) register accessor: desc SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "desc SR"]
pub mod sr;
#[doc = "CLR (w) register accessor: desc CLR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`] module"]
#[doc(alias = "CLR")]
pub type Clr = crate::Reg<clr::ClrSpec>;
#[doc = "desc CLR"]
pub mod clr;
#[doc = "DTR (w) register accessor: desc DTR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtr`] module"]
#[doc(alias = "DTR")]
pub type Dtr = crate::Reg<dtr::DtrSpec>;
#[doc = "desc DTR"]
pub mod dtr;
#[doc = "DRR (r) register accessor: desc DRR\n\nYou can [`read`](crate::Reg::read) this register and get [`drr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drr`] module"]
#[doc(alias = "DRR")]
pub type Drr = crate::Reg<drr::DrrSpec>;
#[doc = "desc DRR"]
pub mod drr;
#[doc = "CCR (rw) register accessor: desc CCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "desc CCR"]
pub mod ccr;
#[doc = "FLTR (rw) register accessor: desc FLTR\n\nYou can [`read`](crate::Reg::read) this register and get [`fltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltr`] module"]
#[doc(alias = "FLTR")]
pub type Fltr = crate::Reg<fltr::FltrSpec>;
#[doc = "desc FLTR"]
pub mod fltr;
