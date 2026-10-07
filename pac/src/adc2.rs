#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    str: Str,
    _reserved1: [u8; 0x01],
    cr0: Cr0,
    cr1: Cr1,
    _reserved3: [u8; 0x04],
    trgsr: Trgsr,
    chselra: Chselra,
    chselrb: Chselrb,
    avchselr: Avchselr,
    _reserved7: [u8; 0x08],
    sstr0: Sstr0,
    sstr1: Sstr1,
    sstr2: Sstr2,
    sstr3: Sstr3,
    sstr4: Sstr4,
    sstr5: Sstr5,
    sstr6: Sstr6,
    sstr7: Sstr7,
    sstr8: Sstr8,
    _reserved16: [u8; 0x0f],
    chmuxr0: Chmuxr0,
    chmuxr1: Chmuxr1,
    chmuxr2: Chmuxr2,
    _reserved19: [u8; 0x08],
    isr: Isr,
    icr: Icr,
    _reserved21: [u8; 0x08],
    dr0: Dr0,
    dr1: Dr1,
    dr2: Dr2,
    dr3: Dr3,
    dr4: Dr4,
    dr5: Dr5,
    dr6: Dr6,
    dr7: Dr7,
    dr8: Dr8,
    _reserved30: [u8; 0x3e],
    awdcr: Awdcr,
    _reserved31: [u8; 0x02],
    awddr0: Awddr0,
    awddr1: Awddr1,
    _reserved33: [u8; 0x04],
    awdchsr: Awdchsr,
    awdsr: Awdsr,
}
impl RegisterBlock {
    #[doc = "0x00 - desc STR"]
    #[inline(always)]
    pub const fn str(&self) -> &Str {
        &self.str
    }
    #[doc = "0x02 - desc CR0"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0x04 - desc CR1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x0a - desc TRGSR"]
    #[inline(always)]
    pub const fn trgsr(&self) -> &Trgsr {
        &self.trgsr
    }
    #[doc = "0x0c - desc CHSELRA"]
    #[inline(always)]
    pub const fn chselra(&self) -> &Chselra {
        &self.chselra
    }
    #[doc = "0x10 - desc CHSELRB"]
    #[inline(always)]
    pub const fn chselrb(&self) -> &Chselrb {
        &self.chselrb
    }
    #[doc = "0x14 - desc AVCHSELR"]
    #[inline(always)]
    pub const fn avchselr(&self) -> &Avchselr {
        &self.avchselr
    }
    #[doc = "0x20 - desc SSTR0"]
    #[inline(always)]
    pub const fn sstr0(&self) -> &Sstr0 {
        &self.sstr0
    }
    #[doc = "0x21 - desc SSTR1"]
    #[inline(always)]
    pub const fn sstr1(&self) -> &Sstr1 {
        &self.sstr1
    }
    #[doc = "0x22 - desc SSTR2"]
    #[inline(always)]
    pub const fn sstr2(&self) -> &Sstr2 {
        &self.sstr2
    }
    #[doc = "0x23 - desc SSTR3"]
    #[inline(always)]
    pub const fn sstr3(&self) -> &Sstr3 {
        &self.sstr3
    }
    #[doc = "0x24 - desc SSTR4"]
    #[inline(always)]
    pub const fn sstr4(&self) -> &Sstr4 {
        &self.sstr4
    }
    #[doc = "0x25 - desc SSTR5"]
    #[inline(always)]
    pub const fn sstr5(&self) -> &Sstr5 {
        &self.sstr5
    }
    #[doc = "0x26 - desc SSTR6"]
    #[inline(always)]
    pub const fn sstr6(&self) -> &Sstr6 {
        &self.sstr6
    }
    #[doc = "0x27 - desc SSTR7"]
    #[inline(always)]
    pub const fn sstr7(&self) -> &Sstr7 {
        &self.sstr7
    }
    #[doc = "0x28 - desc SSTR8"]
    #[inline(always)]
    pub const fn sstr8(&self) -> &Sstr8 {
        &self.sstr8
    }
    #[doc = "0x38 - desc CHMUXR0"]
    #[inline(always)]
    pub const fn chmuxr0(&self) -> &Chmuxr0 {
        &self.chmuxr0
    }
    #[doc = "0x3a - desc CHMUXR1"]
    #[inline(always)]
    pub const fn chmuxr1(&self) -> &Chmuxr1 {
        &self.chmuxr1
    }
    #[doc = "0x3c - desc CHMUXR2"]
    #[inline(always)]
    pub const fn chmuxr2(&self) -> &Chmuxr2 {
        &self.chmuxr2
    }
    #[doc = "0x46 - desc ISR"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x47 - desc ICR"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x50 - desc DR0"]
    #[inline(always)]
    pub const fn dr0(&self) -> &Dr0 {
        &self.dr0
    }
    #[doc = "0x52 - desc DR1"]
    #[inline(always)]
    pub const fn dr1(&self) -> &Dr1 {
        &self.dr1
    }
    #[doc = "0x54 - desc DR2"]
    #[inline(always)]
    pub const fn dr2(&self) -> &Dr2 {
        &self.dr2
    }
    #[doc = "0x56 - desc DR3"]
    #[inline(always)]
    pub const fn dr3(&self) -> &Dr3 {
        &self.dr3
    }
    #[doc = "0x58 - desc DR4"]
    #[inline(always)]
    pub const fn dr4(&self) -> &Dr4 {
        &self.dr4
    }
    #[doc = "0x5a - desc DR5"]
    #[inline(always)]
    pub const fn dr5(&self) -> &Dr5 {
        &self.dr5
    }
    #[doc = "0x5c - desc DR6"]
    #[inline(always)]
    pub const fn dr6(&self) -> &Dr6 {
        &self.dr6
    }
    #[doc = "0x5e - desc DR7"]
    #[inline(always)]
    pub const fn dr7(&self) -> &Dr7 {
        &self.dr7
    }
    #[doc = "0x60 - desc DR8"]
    #[inline(always)]
    pub const fn dr8(&self) -> &Dr8 {
        &self.dr8
    }
    #[doc = "0xa0 - desc AWDCR"]
    #[inline(always)]
    pub const fn awdcr(&self) -> &Awdcr {
        &self.awdcr
    }
    #[doc = "0xa4 - desc AWDDR0"]
    #[inline(always)]
    pub const fn awddr0(&self) -> &Awddr0 {
        &self.awddr0
    }
    #[doc = "0xa6 - desc AWDDR1"]
    #[inline(always)]
    pub const fn awddr1(&self) -> &Awddr1 {
        &self.awddr1
    }
    #[doc = "0xac - desc AWDCHSR"]
    #[inline(always)]
    pub const fn awdchsr(&self) -> &Awdchsr {
        &self.awdchsr
    }
    #[doc = "0xb0 - desc AWDSR"]
    #[inline(always)]
    pub const fn awdsr(&self) -> &Awdsr {
        &self.awdsr
    }
}
#[doc = "STR (rw) register accessor: desc STR\n\nYou can [`read`](crate::Reg::read) this register and get [`str::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`str::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@str`] module"]
#[doc(alias = "STR")]
pub type Str = crate::Reg<str::StrSpec>;
#[doc = "desc STR"]
pub mod str;
#[doc = "CR0 (rw) register accessor: desc CR0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`] module"]
#[doc(alias = "CR0")]
pub type Cr0 = crate::Reg<cr0::Cr0Spec>;
#[doc = "desc CR0"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: desc CR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "desc CR1"]
pub mod cr1;
#[doc = "TRGSR (rw) register accessor: desc TRGSR\n\nYou can [`read`](crate::Reg::read) this register and get [`trgsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trgsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trgsr`] module"]
#[doc(alias = "TRGSR")]
pub type Trgsr = crate::Reg<trgsr::TrgsrSpec>;
#[doc = "desc TRGSR"]
pub mod trgsr;
#[doc = "CHSELRA (rw) register accessor: desc CHSELRA\n\nYou can [`read`](crate::Reg::read) this register and get [`chselra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chselra`] module"]
#[doc(alias = "CHSELRA")]
pub type Chselra = crate::Reg<chselra::ChselraSpec>;
#[doc = "desc CHSELRA"]
pub mod chselra;
#[doc = "CHSELRB (rw) register accessor: desc CHSELRB\n\nYou can [`read`](crate::Reg::read) this register and get [`chselrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chselrb`] module"]
#[doc(alias = "CHSELRB")]
pub type Chselrb = crate::Reg<chselrb::ChselrbSpec>;
#[doc = "desc CHSELRB"]
pub mod chselrb;
#[doc = "AVCHSELR (rw) register accessor: desc AVCHSELR\n\nYou can [`read`](crate::Reg::read) this register and get [`avchselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`avchselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avchselr`] module"]
#[doc(alias = "AVCHSELR")]
pub type Avchselr = crate::Reg<avchselr::AvchselrSpec>;
#[doc = "desc AVCHSELR"]
pub mod avchselr;
#[doc = "SSTR0 (rw) register accessor: desc SSTR0\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr0`] module"]
#[doc(alias = "SSTR0")]
pub type Sstr0 = crate::Reg<sstr0::Sstr0Spec>;
#[doc = "desc SSTR0"]
pub mod sstr0;
#[doc = "SSTR1 (rw) register accessor: desc SSTR1\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr1`] module"]
#[doc(alias = "SSTR1")]
pub type Sstr1 = crate::Reg<sstr1::Sstr1Spec>;
#[doc = "desc SSTR1"]
pub mod sstr1;
#[doc = "SSTR2 (rw) register accessor: desc SSTR2\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr2`] module"]
#[doc(alias = "SSTR2")]
pub type Sstr2 = crate::Reg<sstr2::Sstr2Spec>;
#[doc = "desc SSTR2"]
pub mod sstr2;
#[doc = "SSTR3 (rw) register accessor: desc SSTR3\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr3`] module"]
#[doc(alias = "SSTR3")]
pub type Sstr3 = crate::Reg<sstr3::Sstr3Spec>;
#[doc = "desc SSTR3"]
pub mod sstr3;
#[doc = "SSTR4 (rw) register accessor: desc SSTR4\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr4`] module"]
#[doc(alias = "SSTR4")]
pub type Sstr4 = crate::Reg<sstr4::Sstr4Spec>;
#[doc = "desc SSTR4"]
pub mod sstr4;
#[doc = "SSTR5 (rw) register accessor: desc SSTR5\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr5`] module"]
#[doc(alias = "SSTR5")]
pub type Sstr5 = crate::Reg<sstr5::Sstr5Spec>;
#[doc = "desc SSTR5"]
pub mod sstr5;
#[doc = "SSTR6 (rw) register accessor: desc SSTR6\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr6`] module"]
#[doc(alias = "SSTR6")]
pub type Sstr6 = crate::Reg<sstr6::Sstr6Spec>;
#[doc = "desc SSTR6"]
pub mod sstr6;
#[doc = "SSTR7 (rw) register accessor: desc SSTR7\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr7`] module"]
#[doc(alias = "SSTR7")]
pub type Sstr7 = crate::Reg<sstr7::Sstr7Spec>;
#[doc = "desc SSTR7"]
pub mod sstr7;
#[doc = "SSTR8 (rw) register accessor: desc SSTR8\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr8`] module"]
#[doc(alias = "SSTR8")]
pub type Sstr8 = crate::Reg<sstr8::Sstr8Spec>;
#[doc = "desc SSTR8"]
pub mod sstr8;
#[doc = "CHMUXR0 (rw) register accessor: desc CHMUXR0\n\nYou can [`read`](crate::Reg::read) this register and get [`chmuxr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmuxr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chmuxr0`] module"]
#[doc(alias = "CHMUXR0")]
pub type Chmuxr0 = crate::Reg<chmuxr0::Chmuxr0Spec>;
#[doc = "desc CHMUXR0"]
pub mod chmuxr0;
#[doc = "CHMUXR1 (rw) register accessor: desc CHMUXR1\n\nYou can [`read`](crate::Reg::read) this register and get [`chmuxr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmuxr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chmuxr1`] module"]
#[doc(alias = "CHMUXR1")]
pub type Chmuxr1 = crate::Reg<chmuxr1::Chmuxr1Spec>;
#[doc = "desc CHMUXR1"]
pub mod chmuxr1;
#[doc = "CHMUXR2 (rw) register accessor: desc CHMUXR2\n\nYou can [`read`](crate::Reg::read) this register and get [`chmuxr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmuxr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chmuxr2`] module"]
#[doc(alias = "CHMUXR2")]
pub type Chmuxr2 = crate::Reg<chmuxr2::Chmuxr2Spec>;
#[doc = "desc CHMUXR2"]
pub mod chmuxr2;
#[doc = "ISR (rw) register accessor: desc ISR\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "desc ISR"]
pub mod isr;
#[doc = "ICR (rw) register accessor: desc ICR\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "desc ICR"]
pub mod icr;
#[doc = "DR0 (r) register accessor: desc DR0\n\nYou can [`read`](crate::Reg::read) this register and get [`dr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr0`] module"]
#[doc(alias = "DR0")]
pub type Dr0 = crate::Reg<dr0::Dr0Spec>;
#[doc = "desc DR0"]
pub mod dr0;
#[doc = "DR1 (r) register accessor: desc DR1\n\nYou can [`read`](crate::Reg::read) this register and get [`dr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr1`] module"]
#[doc(alias = "DR1")]
pub type Dr1 = crate::Reg<dr1::Dr1Spec>;
#[doc = "desc DR1"]
pub mod dr1;
#[doc = "DR2 (r) register accessor: desc DR2\n\nYou can [`read`](crate::Reg::read) this register and get [`dr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr2`] module"]
#[doc(alias = "DR2")]
pub type Dr2 = crate::Reg<dr2::Dr2Spec>;
#[doc = "desc DR2"]
pub mod dr2;
#[doc = "DR3 (r) register accessor: desc DR3\n\nYou can [`read`](crate::Reg::read) this register and get [`dr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr3`] module"]
#[doc(alias = "DR3")]
pub type Dr3 = crate::Reg<dr3::Dr3Spec>;
#[doc = "desc DR3"]
pub mod dr3;
#[doc = "DR4 (r) register accessor: desc DR4\n\nYou can [`read`](crate::Reg::read) this register and get [`dr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr4`] module"]
#[doc(alias = "DR4")]
pub type Dr4 = crate::Reg<dr4::Dr4Spec>;
#[doc = "desc DR4"]
pub mod dr4;
#[doc = "DR5 (r) register accessor: desc DR5\n\nYou can [`read`](crate::Reg::read) this register and get [`dr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr5`] module"]
#[doc(alias = "DR5")]
pub type Dr5 = crate::Reg<dr5::Dr5Spec>;
#[doc = "desc DR5"]
pub mod dr5;
#[doc = "DR6 (r) register accessor: desc DR6\n\nYou can [`read`](crate::Reg::read) this register and get [`dr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr6`] module"]
#[doc(alias = "DR6")]
pub type Dr6 = crate::Reg<dr6::Dr6Spec>;
#[doc = "desc DR6"]
pub mod dr6;
#[doc = "DR7 (r) register accessor: desc DR7\n\nYou can [`read`](crate::Reg::read) this register and get [`dr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr7`] module"]
#[doc(alias = "DR7")]
pub type Dr7 = crate::Reg<dr7::Dr7Spec>;
#[doc = "desc DR7"]
pub mod dr7;
#[doc = "DR8 (r) register accessor: desc DR8\n\nYou can [`read`](crate::Reg::read) this register and get [`dr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr8`] module"]
#[doc(alias = "DR8")]
pub type Dr8 = crate::Reg<dr8::Dr8Spec>;
#[doc = "desc DR8"]
pub mod dr8;
#[doc = "AWDCR (rw) register accessor: desc AWDCR\n\nYou can [`read`](crate::Reg::read) this register and get [`awdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awdcr`] module"]
#[doc(alias = "AWDCR")]
pub type Awdcr = crate::Reg<awdcr::AwdcrSpec>;
#[doc = "desc AWDCR"]
pub mod awdcr;
#[doc = "AWDDR0 (rw) register accessor: desc AWDDR0\n\nYou can [`read`](crate::Reg::read) this register and get [`awddr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awddr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awddr0`] module"]
#[doc(alias = "AWDDR0")]
pub type Awddr0 = crate::Reg<awddr0::Awddr0Spec>;
#[doc = "desc AWDDR0"]
pub mod awddr0;
#[doc = "AWDDR1 (rw) register accessor: desc AWDDR1\n\nYou can [`read`](crate::Reg::read) this register and get [`awddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awddr1`] module"]
#[doc(alias = "AWDDR1")]
pub type Awddr1 = crate::Reg<awddr1::Awddr1Spec>;
#[doc = "desc AWDDR1"]
pub mod awddr1;
#[doc = "AWDCHSR (rw) register accessor: desc AWDCHSR\n\nYou can [`read`](crate::Reg::read) this register and get [`awdchsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awdchsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awdchsr`] module"]
#[doc(alias = "AWDCHSR")]
pub type Awdchsr = crate::Reg<awdchsr::AwdchsrSpec>;
#[doc = "desc AWDCHSR"]
pub mod awdchsr;
#[doc = "AWDSR (rw) register accessor: desc AWDSR\n\nYou can [`read`](crate::Reg::read) this register and get [`awdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awdsr`] module"]
#[doc(alias = "AWDSR")]
pub type Awdsr = crate::Reg<awdsr::AwdsrSpec>;
#[doc = "desc AWDSR"]
pub mod awdsr;
