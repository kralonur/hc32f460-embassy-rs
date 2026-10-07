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
    sstr9: Sstr9,
    sstr10: Sstr10,
    sstr11: Sstr11,
    sstr12: Sstr12,
    sstr13: Sstr13,
    sstr14: Sstr14,
    sstr15: Sstr15,
    sstrl: Sstrl,
    _reserved24: [u8; 0x07],
    chmuxr0: Chmuxr0,
    chmuxr1: Chmuxr1,
    chmuxr2: Chmuxr2,
    chmuxr3: Chmuxr3,
    _reserved28: [u8; 0x06],
    isr: Isr,
    icr: Icr,
    _reserved30: [u8; 0x04],
    synccr: Synccr,
    _reserved31: [u8; 0x02],
    dr0: Dr0,
    dr1: Dr1,
    dr2: Dr2,
    dr3: Dr3,
    dr4: Dr4,
    dr5: Dr5,
    dr6: Dr6,
    dr7: Dr7,
    dr8: Dr8,
    dr9: Dr9,
    dr10: Dr10,
    dr11: Dr11,
    dr12: Dr12,
    dr13: Dr13,
    dr14: Dr14,
    dr15: Dr15,
    dr16: Dr16,
    _reserved48: [u8; 0x2e],
    awdcr: Awdcr,
    _reserved49: [u8; 0x02],
    awddr0: Awddr0,
    awddr1: Awddr1,
    _reserved51: [u8; 0x04],
    awdchsr: Awdchsr,
    awdsr: Awdsr,
    _reserved53: [u8; 0x0c],
    pgacr: Pgacr,
    pgagsr: Pgagsr,
    _reserved55: [u8; 0x08],
    pgainsr0: Pgainsr0,
    pgainsr1: Pgainsr1,
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
    #[doc = "0x29 - desc SSTR9"]
    #[inline(always)]
    pub const fn sstr9(&self) -> &Sstr9 {
        &self.sstr9
    }
    #[doc = "0x2a - desc SSTR10"]
    #[inline(always)]
    pub const fn sstr10(&self) -> &Sstr10 {
        &self.sstr10
    }
    #[doc = "0x2b - desc SSTR11"]
    #[inline(always)]
    pub const fn sstr11(&self) -> &Sstr11 {
        &self.sstr11
    }
    #[doc = "0x2c - desc SSTR12"]
    #[inline(always)]
    pub const fn sstr12(&self) -> &Sstr12 {
        &self.sstr12
    }
    #[doc = "0x2d - desc SSTR13"]
    #[inline(always)]
    pub const fn sstr13(&self) -> &Sstr13 {
        &self.sstr13
    }
    #[doc = "0x2e - desc SSTR14"]
    #[inline(always)]
    pub const fn sstr14(&self) -> &Sstr14 {
        &self.sstr14
    }
    #[doc = "0x2f - desc SSTR15"]
    #[inline(always)]
    pub const fn sstr15(&self) -> &Sstr15 {
        &self.sstr15
    }
    #[doc = "0x30 - desc SSTRL"]
    #[inline(always)]
    pub const fn sstrl(&self) -> &Sstrl {
        &self.sstrl
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
    #[doc = "0x3e - desc CHMUXR3"]
    #[inline(always)]
    pub const fn chmuxr3(&self) -> &Chmuxr3 {
        &self.chmuxr3
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
    #[doc = "0x4c - desc SYNCCR"]
    #[inline(always)]
    pub const fn synccr(&self) -> &Synccr {
        &self.synccr
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
    #[doc = "0x62 - desc DR9"]
    #[inline(always)]
    pub const fn dr9(&self) -> &Dr9 {
        &self.dr9
    }
    #[doc = "0x64 - desc DR10"]
    #[inline(always)]
    pub const fn dr10(&self) -> &Dr10 {
        &self.dr10
    }
    #[doc = "0x66 - desc DR11"]
    #[inline(always)]
    pub const fn dr11(&self) -> &Dr11 {
        &self.dr11
    }
    #[doc = "0x68 - desc DR12"]
    #[inline(always)]
    pub const fn dr12(&self) -> &Dr12 {
        &self.dr12
    }
    #[doc = "0x6a - desc DR13"]
    #[inline(always)]
    pub const fn dr13(&self) -> &Dr13 {
        &self.dr13
    }
    #[doc = "0x6c - desc DR14"]
    #[inline(always)]
    pub const fn dr14(&self) -> &Dr14 {
        &self.dr14
    }
    #[doc = "0x6e - desc DR15"]
    #[inline(always)]
    pub const fn dr15(&self) -> &Dr15 {
        &self.dr15
    }
    #[doc = "0x70 - desc DR16"]
    #[inline(always)]
    pub const fn dr16(&self) -> &Dr16 {
        &self.dr16
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
    #[doc = "0xc0 - desc PGACR"]
    #[inline(always)]
    pub const fn pgacr(&self) -> &Pgacr {
        &self.pgacr
    }
    #[doc = "0xc2 - desc PGAGSR"]
    #[inline(always)]
    pub const fn pgagsr(&self) -> &Pgagsr {
        &self.pgagsr
    }
    #[doc = "0xcc - desc PGAINSR0"]
    #[inline(always)]
    pub const fn pgainsr0(&self) -> &Pgainsr0 {
        &self.pgainsr0
    }
    #[doc = "0xce - desc PGAINSR1"]
    #[inline(always)]
    pub const fn pgainsr1(&self) -> &Pgainsr1 {
        &self.pgainsr1
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
#[doc = "SSTR9 (rw) register accessor: desc SSTR9\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr9`] module"]
#[doc(alias = "SSTR9")]
pub type Sstr9 = crate::Reg<sstr9::Sstr9Spec>;
#[doc = "desc SSTR9"]
pub mod sstr9;
#[doc = "SSTR10 (rw) register accessor: desc SSTR10\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr10`] module"]
#[doc(alias = "SSTR10")]
pub type Sstr10 = crate::Reg<sstr10::Sstr10Spec>;
#[doc = "desc SSTR10"]
pub mod sstr10;
#[doc = "SSTR11 (rw) register accessor: desc SSTR11\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr11`] module"]
#[doc(alias = "SSTR11")]
pub type Sstr11 = crate::Reg<sstr11::Sstr11Spec>;
#[doc = "desc SSTR11"]
pub mod sstr11;
#[doc = "SSTR12 (rw) register accessor: desc SSTR12\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr12`] module"]
#[doc(alias = "SSTR12")]
pub type Sstr12 = crate::Reg<sstr12::Sstr12Spec>;
#[doc = "desc SSTR12"]
pub mod sstr12;
#[doc = "SSTR13 (rw) register accessor: desc SSTR13\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr13`] module"]
#[doc(alias = "SSTR13")]
pub type Sstr13 = crate::Reg<sstr13::Sstr13Spec>;
#[doc = "desc SSTR13"]
pub mod sstr13;
#[doc = "SSTR14 (rw) register accessor: desc SSTR14\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr14`] module"]
#[doc(alias = "SSTR14")]
pub type Sstr14 = crate::Reg<sstr14::Sstr14Spec>;
#[doc = "desc SSTR14"]
pub mod sstr14;
#[doc = "SSTR15 (rw) register accessor: desc SSTR15\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstr15`] module"]
#[doc(alias = "SSTR15")]
pub type Sstr15 = crate::Reg<sstr15::Sstr15Spec>;
#[doc = "desc SSTR15"]
pub mod sstr15;
#[doc = "SSTRL (rw) register accessor: desc SSTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`sstrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstrl`] module"]
#[doc(alias = "SSTRL")]
pub type Sstrl = crate::Reg<sstrl::SstrlSpec>;
#[doc = "desc SSTRL"]
pub mod sstrl;
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
#[doc = "CHMUXR3 (rw) register accessor: desc CHMUXR3\n\nYou can [`read`](crate::Reg::read) this register and get [`chmuxr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmuxr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chmuxr3`] module"]
#[doc(alias = "CHMUXR3")]
pub type Chmuxr3 = crate::Reg<chmuxr3::Chmuxr3Spec>;
#[doc = "desc CHMUXR3"]
pub mod chmuxr3;
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
#[doc = "SYNCCR (rw) register accessor: desc SYNCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`synccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synccr`] module"]
#[doc(alias = "SYNCCR")]
pub type Synccr = crate::Reg<synccr::SynccrSpec>;
#[doc = "desc SYNCCR"]
pub mod synccr;
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
#[doc = "DR9 (r) register accessor: desc DR9\n\nYou can [`read`](crate::Reg::read) this register and get [`dr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr9`] module"]
#[doc(alias = "DR9")]
pub type Dr9 = crate::Reg<dr9::Dr9Spec>;
#[doc = "desc DR9"]
pub mod dr9;
#[doc = "DR10 (r) register accessor: desc DR10\n\nYou can [`read`](crate::Reg::read) this register and get [`dr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr10`] module"]
#[doc(alias = "DR10")]
pub type Dr10 = crate::Reg<dr10::Dr10Spec>;
#[doc = "desc DR10"]
pub mod dr10;
#[doc = "DR11 (r) register accessor: desc DR11\n\nYou can [`read`](crate::Reg::read) this register and get [`dr11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr11`] module"]
#[doc(alias = "DR11")]
pub type Dr11 = crate::Reg<dr11::Dr11Spec>;
#[doc = "desc DR11"]
pub mod dr11;
#[doc = "DR12 (r) register accessor: desc DR12\n\nYou can [`read`](crate::Reg::read) this register and get [`dr12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr12`] module"]
#[doc(alias = "DR12")]
pub type Dr12 = crate::Reg<dr12::Dr12Spec>;
#[doc = "desc DR12"]
pub mod dr12;
#[doc = "DR13 (r) register accessor: desc DR13\n\nYou can [`read`](crate::Reg::read) this register and get [`dr13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr13`] module"]
#[doc(alias = "DR13")]
pub type Dr13 = crate::Reg<dr13::Dr13Spec>;
#[doc = "desc DR13"]
pub mod dr13;
#[doc = "DR14 (r) register accessor: desc DR14\n\nYou can [`read`](crate::Reg::read) this register and get [`dr14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr14`] module"]
#[doc(alias = "DR14")]
pub type Dr14 = crate::Reg<dr14::Dr14Spec>;
#[doc = "desc DR14"]
pub mod dr14;
#[doc = "DR15 (r) register accessor: desc DR15\n\nYou can [`read`](crate::Reg::read) this register and get [`dr15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr15`] module"]
#[doc(alias = "DR15")]
pub type Dr15 = crate::Reg<dr15::Dr15Spec>;
#[doc = "desc DR15"]
pub mod dr15;
#[doc = "DR16 (r) register accessor: desc DR16\n\nYou can [`read`](crate::Reg::read) this register and get [`dr16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr16`] module"]
#[doc(alias = "DR16")]
pub type Dr16 = crate::Reg<dr16::Dr16Spec>;
#[doc = "desc DR16"]
pub mod dr16;
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
#[doc = "PGACR (rw) register accessor: desc PGACR\n\nYou can [`read`](crate::Reg::read) this register and get [`pgacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgacr`] module"]
#[doc(alias = "PGACR")]
pub type Pgacr = crate::Reg<pgacr::PgacrSpec>;
#[doc = "desc PGACR"]
pub mod pgacr;
#[doc = "PGAGSR (rw) register accessor: desc PGAGSR\n\nYou can [`read`](crate::Reg::read) this register and get [`pgagsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgagsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgagsr`] module"]
#[doc(alias = "PGAGSR")]
pub type Pgagsr = crate::Reg<pgagsr::PgagsrSpec>;
#[doc = "desc PGAGSR"]
pub mod pgagsr;
#[doc = "PGAINSR0 (rw) register accessor: desc PGAINSR0\n\nYou can [`read`](crate::Reg::read) this register and get [`pgainsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgainsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgainsr0`] module"]
#[doc(alias = "PGAINSR0")]
pub type Pgainsr0 = crate::Reg<pgainsr0::Pgainsr0Spec>;
#[doc = "desc PGAINSR0"]
pub mod pgainsr0;
#[doc = "PGAINSR1 (rw) register accessor: desc PGAINSR1\n\nYou can [`read`](crate::Reg::read) this register and get [`pgainsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgainsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgainsr1`] module"]
#[doc(alias = "PGAINSR1")]
pub type Pgainsr1 = crate::Reg<pgainsr1::Pgainsr1Spec>;
#[doc = "desc PGAINSR1"]
pub mod pgainsr1;
