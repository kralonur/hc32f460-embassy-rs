#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    en: En,
    intstat0: Intstat0,
    intstat1: Intstat1,
    intmask0: Intmask0,
    intmask1: Intmask1,
    intclr0: Intclr0,
    intclr1: Intclr1,
    chen: Chen,
    reqstat: Reqstat,
    chstat: Chstat,
    _reserved10: [u8; 0x04],
    rcfgctl: Rcfgctl,
    _reserved11: [u8; 0x10],
    sar0: Sar0,
    dar0: Dar0,
    dtctl0: Dtctl0,
    _reserved_14_rpt0: [u8; 0x04],
    _reserved_15_snseqctl0: [u8; 0x04],
    _reserved_16_dnseqctl0: [u8; 0x04],
    llp0: Llp0,
    chctl0: Chctl0,
    monsar0: Monsar0,
    mondar0: Mondar0,
    mondtctl0: Mondtctl0,
    monrpt0: Monrpt0,
    monsnseqctl0: Monsnseqctl0,
    mondnseqctl0: Mondnseqctl0,
    _reserved25: [u8; 0x08],
    sar1: Sar1,
    dar1: Dar1,
    dtctl1: Dtctl1,
    _reserved_28_rpt1: [u8; 0x04],
    _reserved_29_snseqctl1: [u8; 0x04],
    _reserved_30_dnseqctl1: [u8; 0x04],
    llp1: Llp1,
    chctl1: Chctl1,
    monsar1: Monsar1,
    mondar1: Mondar1,
    mondtctl1: Mondtctl1,
    monrpt1: Monrpt1,
    monsnseqctl1: Monsnseqctl1,
    mondnseqctl1: Mondnseqctl1,
    _reserved39: [u8; 0x08],
    sar2: Sar2,
    dar2: Dar2,
    dtctl2: Dtctl2,
    _reserved_42_rpt2: [u8; 0x04],
    _reserved_43_snseqctl2: [u8; 0x04],
    _reserved_44_dnseqctl2: [u8; 0x04],
    llp2: Llp2,
    chctl2: Chctl2,
    monsar2: Monsar2,
    mondar2: Mondar2,
    mondtctl2: Mondtctl2,
    monrpt2: Monrpt2,
    monsnseqctl2: Monsnseqctl2,
    mondnseqctl2: Mondnseqctl2,
    _reserved53: [u8; 0x08],
    sar3: Sar3,
    dar3: Dar3,
    dtctl3: Dtctl3,
    _reserved_56_rpt3: [u8; 0x04],
    _reserved_57_snseqctl3: [u8; 0x04],
    _reserved_58_dnseqctl3: [u8; 0x04],
    llp3: Llp3,
    chctl3: Chctl3,
    monsar3: Monsar3,
    mondar3: Mondar3,
    mondtctl3: Mondtctl3,
    monrpt3: Monrpt3,
    monsnseqctl3: Monsnseqctl3,
    mondnseqctl3: Mondnseqctl3,
}
impl RegisterBlock {
    #[doc = "0x00 - desc EN"]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x04 - desc INTSTAT0"]
    #[inline(always)]
    pub const fn intstat0(&self) -> &Intstat0 {
        &self.intstat0
    }
    #[doc = "0x08 - desc INTSTAT1"]
    #[inline(always)]
    pub const fn intstat1(&self) -> &Intstat1 {
        &self.intstat1
    }
    #[doc = "0x0c - desc INTMASK0"]
    #[inline(always)]
    pub const fn intmask0(&self) -> &Intmask0 {
        &self.intmask0
    }
    #[doc = "0x10 - desc INTMASK1"]
    #[inline(always)]
    pub const fn intmask1(&self) -> &Intmask1 {
        &self.intmask1
    }
    #[doc = "0x14 - desc INTCLR0"]
    #[inline(always)]
    pub const fn intclr0(&self) -> &Intclr0 {
        &self.intclr0
    }
    #[doc = "0x18 - desc INTCLR1"]
    #[inline(always)]
    pub const fn intclr1(&self) -> &Intclr1 {
        &self.intclr1
    }
    #[doc = "0x1c - desc CHEN"]
    #[inline(always)]
    pub const fn chen(&self) -> &Chen {
        &self.chen
    }
    #[doc = "0x20 - desc REQSTAT"]
    #[inline(always)]
    pub const fn reqstat(&self) -> &Reqstat {
        &self.reqstat
    }
    #[doc = "0x24 - desc CHSTAT"]
    #[inline(always)]
    pub const fn chstat(&self) -> &Chstat {
        &self.chstat
    }
    #[doc = "0x2c - desc RCFGCTL"]
    #[inline(always)]
    pub const fn rcfgctl(&self) -> &Rcfgctl {
        &self.rcfgctl
    }
    #[doc = "0x40 - desc SAR0"]
    #[inline(always)]
    pub const fn sar0(&self) -> &Sar0 {
        &self.sar0
    }
    #[doc = "0x44 - desc DAR0"]
    #[inline(always)]
    pub const fn dar0(&self) -> &Dar0 {
        &self.dar0
    }
    #[doc = "0x48 - desc DTCTL0"]
    #[inline(always)]
    pub const fn dtctl0(&self) -> &Dtctl0 {
        &self.dtctl0
    }
    #[doc = "0x4c - desc RPTB0"]
    #[inline(always)]
    pub const fn rptb0(&self) -> &Rptb0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4c - desc RPT0"]
    #[inline(always)]
    pub const fn rpt0(&self) -> &Rpt0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x50 - desc SNSEQCTLB0"]
    #[inline(always)]
    pub const fn snseqctlb0(&self) -> &Snseqctlb0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x50 - desc SNSEQCTL0"]
    #[inline(always)]
    pub const fn snseqctl0(&self) -> &Snseqctl0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x54 - desc DNSEQCTLB0"]
    #[inline(always)]
    pub const fn dnseqctlb0(&self) -> &Dnseqctlb0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x54 - desc DNSEQCTL0"]
    #[inline(always)]
    pub const fn dnseqctl0(&self) -> &Dnseqctl0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x58 - desc LLP0"]
    #[inline(always)]
    pub const fn llp0(&self) -> &Llp0 {
        &self.llp0
    }
    #[doc = "0x5c - desc CHCTL0"]
    #[inline(always)]
    pub const fn chctl0(&self) -> &Chctl0 {
        &self.chctl0
    }
    #[doc = "0x60 - desc MONSAR0"]
    #[inline(always)]
    pub const fn monsar0(&self) -> &Monsar0 {
        &self.monsar0
    }
    #[doc = "0x64 - desc MONDAR0"]
    #[inline(always)]
    pub const fn mondar0(&self) -> &Mondar0 {
        &self.mondar0
    }
    #[doc = "0x68 - desc MONDTCTL0"]
    #[inline(always)]
    pub const fn mondtctl0(&self) -> &Mondtctl0 {
        &self.mondtctl0
    }
    #[doc = "0x6c - desc MONRPT0"]
    #[inline(always)]
    pub const fn monrpt0(&self) -> &Monrpt0 {
        &self.monrpt0
    }
    #[doc = "0x70 - desc MONSNSEQCTL0"]
    #[inline(always)]
    pub const fn monsnseqctl0(&self) -> &Monsnseqctl0 {
        &self.monsnseqctl0
    }
    #[doc = "0x74 - desc MONDNSEQCTL0"]
    #[inline(always)]
    pub const fn mondnseqctl0(&self) -> &Mondnseqctl0 {
        &self.mondnseqctl0
    }
    #[doc = "0x80 - desc SAR1"]
    #[inline(always)]
    pub const fn sar1(&self) -> &Sar1 {
        &self.sar1
    }
    #[doc = "0x84 - desc DAR1"]
    #[inline(always)]
    pub const fn dar1(&self) -> &Dar1 {
        &self.dar1
    }
    #[doc = "0x88 - desc DTCTL1"]
    #[inline(always)]
    pub const fn dtctl1(&self) -> &Dtctl1 {
        &self.dtctl1
    }
    #[doc = "0x8c - desc RPTB1"]
    #[inline(always)]
    pub const fn rptb1(&self) -> &Rptb1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x8c - desc RPT1"]
    #[inline(always)]
    pub const fn rpt1(&self) -> &Rpt1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x90 - desc SNSEQCTLB1"]
    #[inline(always)]
    pub const fn snseqctlb1(&self) -> &Snseqctlb1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x90 - desc SNSEQCTL1"]
    #[inline(always)]
    pub const fn snseqctl1(&self) -> &Snseqctl1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x94 - desc DNSEQCTLB1"]
    #[inline(always)]
    pub const fn dnseqctlb1(&self) -> &Dnseqctlb1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(148).cast() }
    }
    #[doc = "0x94 - desc DNSEQCTL1"]
    #[inline(always)]
    pub const fn dnseqctl1(&self) -> &Dnseqctl1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(148).cast() }
    }
    #[doc = "0x98 - desc LLP1"]
    #[inline(always)]
    pub const fn llp1(&self) -> &Llp1 {
        &self.llp1
    }
    #[doc = "0x9c - desc CHCTL1"]
    #[inline(always)]
    pub const fn chctl1(&self) -> &Chctl1 {
        &self.chctl1
    }
    #[doc = "0xa0 - desc MONSAR1"]
    #[inline(always)]
    pub const fn monsar1(&self) -> &Monsar1 {
        &self.monsar1
    }
    #[doc = "0xa4 - desc MONDAR1"]
    #[inline(always)]
    pub const fn mondar1(&self) -> &Mondar1 {
        &self.mondar1
    }
    #[doc = "0xa8 - desc MONDTCTL1"]
    #[inline(always)]
    pub const fn mondtctl1(&self) -> &Mondtctl1 {
        &self.mondtctl1
    }
    #[doc = "0xac - desc MONRPT1"]
    #[inline(always)]
    pub const fn monrpt1(&self) -> &Monrpt1 {
        &self.monrpt1
    }
    #[doc = "0xb0 - desc MONSNSEQCTL1"]
    #[inline(always)]
    pub const fn monsnseqctl1(&self) -> &Monsnseqctl1 {
        &self.monsnseqctl1
    }
    #[doc = "0xb4 - desc MONDNSEQCTL1"]
    #[inline(always)]
    pub const fn mondnseqctl1(&self) -> &Mondnseqctl1 {
        &self.mondnseqctl1
    }
    #[doc = "0xc0 - desc SAR2"]
    #[inline(always)]
    pub const fn sar2(&self) -> &Sar2 {
        &self.sar2
    }
    #[doc = "0xc4 - desc DAR2"]
    #[inline(always)]
    pub const fn dar2(&self) -> &Dar2 {
        &self.dar2
    }
    #[doc = "0xc8 - desc DTCTL2"]
    #[inline(always)]
    pub const fn dtctl2(&self) -> &Dtctl2 {
        &self.dtctl2
    }
    #[doc = "0xcc - desc RPTB2"]
    #[inline(always)]
    pub const fn rptb2(&self) -> &Rptb2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(204).cast() }
    }
    #[doc = "0xcc - desc RPT2"]
    #[inline(always)]
    pub const fn rpt2(&self) -> &Rpt2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(204).cast() }
    }
    #[doc = "0xd0 - desc SNSEQCTLB2"]
    #[inline(always)]
    pub const fn snseqctlb2(&self) -> &Snseqctlb2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(208).cast() }
    }
    #[doc = "0xd0 - desc SNSEQCTL2"]
    #[inline(always)]
    pub const fn snseqctl2(&self) -> &Snseqctl2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(208).cast() }
    }
    #[doc = "0xd4 - desc DNSEQCTLB2"]
    #[inline(always)]
    pub const fn dnseqctlb2(&self) -> &Dnseqctlb2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(212).cast() }
    }
    #[doc = "0xd4 - desc DNSEQCTL2"]
    #[inline(always)]
    pub const fn dnseqctl2(&self) -> &Dnseqctl2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(212).cast() }
    }
    #[doc = "0xd8 - desc LLP2"]
    #[inline(always)]
    pub const fn llp2(&self) -> &Llp2 {
        &self.llp2
    }
    #[doc = "0xdc - desc CHCTL2"]
    #[inline(always)]
    pub const fn chctl2(&self) -> &Chctl2 {
        &self.chctl2
    }
    #[doc = "0xe0 - desc MONSAR2"]
    #[inline(always)]
    pub const fn monsar2(&self) -> &Monsar2 {
        &self.monsar2
    }
    #[doc = "0xe4 - desc MONDAR2"]
    #[inline(always)]
    pub const fn mondar2(&self) -> &Mondar2 {
        &self.mondar2
    }
    #[doc = "0xe8 - desc MONDTCTL2"]
    #[inline(always)]
    pub const fn mondtctl2(&self) -> &Mondtctl2 {
        &self.mondtctl2
    }
    #[doc = "0xec - desc MONRPT2"]
    #[inline(always)]
    pub const fn monrpt2(&self) -> &Monrpt2 {
        &self.monrpt2
    }
    #[doc = "0xf0 - desc MONSNSEQCTL2"]
    #[inline(always)]
    pub const fn monsnseqctl2(&self) -> &Monsnseqctl2 {
        &self.monsnseqctl2
    }
    #[doc = "0xf4 - desc MONDNSEQCTL2"]
    #[inline(always)]
    pub const fn mondnseqctl2(&self) -> &Mondnseqctl2 {
        &self.mondnseqctl2
    }
    #[doc = "0x100 - desc SAR3"]
    #[inline(always)]
    pub const fn sar3(&self) -> &Sar3 {
        &self.sar3
    }
    #[doc = "0x104 - desc DAR3"]
    #[inline(always)]
    pub const fn dar3(&self) -> &Dar3 {
        &self.dar3
    }
    #[doc = "0x108 - desc DTCTL3"]
    #[inline(always)]
    pub const fn dtctl3(&self) -> &Dtctl3 {
        &self.dtctl3
    }
    #[doc = "0x10c - desc RPTB3"]
    #[inline(always)]
    pub const fn rptb3(&self) -> &Rptb3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x10c - desc RPT3"]
    #[inline(always)]
    pub const fn rpt3(&self) -> &Rpt3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x110 - desc SNSEQCTLB3"]
    #[inline(always)]
    pub const fn snseqctlb3(&self) -> &Snseqctlb3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(272).cast() }
    }
    #[doc = "0x110 - desc SNSEQCTL3"]
    #[inline(always)]
    pub const fn snseqctl3(&self) -> &Snseqctl3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(272).cast() }
    }
    #[doc = "0x114 - desc DNSEQCTLB3"]
    #[inline(always)]
    pub const fn dnseqctlb3(&self) -> &Dnseqctlb3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x114 - desc DNSEQCTL3"]
    #[inline(always)]
    pub const fn dnseqctl3(&self) -> &Dnseqctl3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x118 - desc LLP3"]
    #[inline(always)]
    pub const fn llp3(&self) -> &Llp3 {
        &self.llp3
    }
    #[doc = "0x11c - desc CHCTL3"]
    #[inline(always)]
    pub const fn chctl3(&self) -> &Chctl3 {
        &self.chctl3
    }
    #[doc = "0x120 - desc MONSAR3"]
    #[inline(always)]
    pub const fn monsar3(&self) -> &Monsar3 {
        &self.monsar3
    }
    #[doc = "0x124 - desc MONDAR3"]
    #[inline(always)]
    pub const fn mondar3(&self) -> &Mondar3 {
        &self.mondar3
    }
    #[doc = "0x128 - desc MONDTCTL3"]
    #[inline(always)]
    pub const fn mondtctl3(&self) -> &Mondtctl3 {
        &self.mondtctl3
    }
    #[doc = "0x12c - desc MONRPT3"]
    #[inline(always)]
    pub const fn monrpt3(&self) -> &Monrpt3 {
        &self.monrpt3
    }
    #[doc = "0x130 - desc MONSNSEQCTL3"]
    #[inline(always)]
    pub const fn monsnseqctl3(&self) -> &Monsnseqctl3 {
        &self.monsnseqctl3
    }
    #[doc = "0x134 - desc MONDNSEQCTL3"]
    #[inline(always)]
    pub const fn mondnseqctl3(&self) -> &Mondnseqctl3 {
        &self.mondnseqctl3
    }
}
#[doc = "EN (rw) register accessor: desc EN\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`] module"]
#[doc(alias = "EN")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "desc EN"]
pub mod en;
#[doc = "INTSTAT0 (r) register accessor: desc INTSTAT0\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat0`] module"]
#[doc(alias = "INTSTAT0")]
pub type Intstat0 = crate::Reg<intstat0::Intstat0Spec>;
#[doc = "desc INTSTAT0"]
pub mod intstat0;
#[doc = "INTSTAT1 (r) register accessor: desc INTSTAT1\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat1`] module"]
#[doc(alias = "INTSTAT1")]
pub type Intstat1 = crate::Reg<intstat1::Intstat1Spec>;
#[doc = "desc INTSTAT1"]
pub mod intstat1;
#[doc = "INTMASK0 (rw) register accessor: desc INTMASK0\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmask0`] module"]
#[doc(alias = "INTMASK0")]
pub type Intmask0 = crate::Reg<intmask0::Intmask0Spec>;
#[doc = "desc INTMASK0"]
pub mod intmask0;
#[doc = "INTMASK1 (rw) register accessor: desc INTMASK1\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmask1`] module"]
#[doc(alias = "INTMASK1")]
pub type Intmask1 = crate::Reg<intmask1::Intmask1Spec>;
#[doc = "desc INTMASK1"]
pub mod intmask1;
#[doc = "INTCLR0 (w) register accessor: desc INTCLR0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr0`] module"]
#[doc(alias = "INTCLR0")]
pub type Intclr0 = crate::Reg<intclr0::Intclr0Spec>;
#[doc = "desc INTCLR0"]
pub mod intclr0;
#[doc = "INTCLR1 (w) register accessor: desc INTCLR1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr1`] module"]
#[doc(alias = "INTCLR1")]
pub type Intclr1 = crate::Reg<intclr1::Intclr1Spec>;
#[doc = "desc INTCLR1"]
pub mod intclr1;
#[doc = "CHEN (rw) register accessor: desc CHEN\n\nYou can [`read`](crate::Reg::read) this register and get [`chen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen`] module"]
#[doc(alias = "CHEN")]
pub type Chen = crate::Reg<chen::ChenSpec>;
#[doc = "desc CHEN"]
pub mod chen;
#[doc = "REQSTAT (r) register accessor: desc REQSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`reqstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqstat`] module"]
#[doc(alias = "REQSTAT")]
pub type Reqstat = crate::Reg<reqstat::ReqstatSpec>;
#[doc = "desc REQSTAT"]
pub mod reqstat;
#[doc = "CHSTAT (r) register accessor: desc CHSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`chstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chstat`] module"]
#[doc(alias = "CHSTAT")]
pub type Chstat = crate::Reg<chstat::ChstatSpec>;
#[doc = "desc CHSTAT"]
pub mod chstat;
#[doc = "RCFGCTL (rw) register accessor: desc RCFGCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`rcfgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcfgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcfgctl`] module"]
#[doc(alias = "RCFGCTL")]
pub type Rcfgctl = crate::Reg<rcfgctl::RcfgctlSpec>;
#[doc = "desc RCFGCTL"]
pub mod rcfgctl;
#[doc = "SAR0 (rw) register accessor: desc SAR0\n\nYou can [`read`](crate::Reg::read) this register and get [`sar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar0`] module"]
#[doc(alias = "SAR0")]
pub type Sar0 = crate::Reg<sar0::Sar0Spec>;
#[doc = "desc SAR0"]
pub mod sar0;
#[doc = "DAR0 (rw) register accessor: desc DAR0\n\nYou can [`read`](crate::Reg::read) this register and get [`dar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar0`] module"]
#[doc(alias = "DAR0")]
pub type Dar0 = crate::Reg<dar0::Dar0Spec>;
#[doc = "desc DAR0"]
pub mod dar0;
#[doc = "DTCTL0 (rw) register accessor: desc DTCTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`dtctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtctl0`] module"]
#[doc(alias = "DTCTL0")]
pub type Dtctl0 = crate::Reg<dtctl0::Dtctl0Spec>;
#[doc = "desc DTCTL0"]
pub mod dtctl0;
#[doc = "RPT0 (rw) register accessor: desc RPT0\n\nYou can [`read`](crate::Reg::read) this register and get [`rpt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpt0`] module"]
#[doc(alias = "RPT0")]
pub type Rpt0 = crate::Reg<rpt0::Rpt0Spec>;
#[doc = "desc RPT0"]
pub mod rpt0;
#[doc = "RPTB0 (rw) register accessor: desc RPTB0\n\nYou can [`read`](crate::Reg::read) this register and get [`rptb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rptb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rptb0`] module"]
#[doc(alias = "RPTB0")]
pub type Rptb0 = crate::Reg<rptb0::Rptb0Spec>;
#[doc = "desc RPTB0"]
pub mod rptb0;
#[doc = "SNSEQCTL0 (rw) register accessor: desc SNSEQCTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`snseqctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snseqctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snseqctl0`] module"]
#[doc(alias = "SNSEQCTL0")]
pub type Snseqctl0 = crate::Reg<snseqctl0::Snseqctl0Spec>;
#[doc = "desc SNSEQCTL0"]
pub mod snseqctl0;
#[doc = "SNSEQCTLB0 (rw) register accessor: desc SNSEQCTLB0\n\nYou can [`read`](crate::Reg::read) this register and get [`snseqctlb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snseqctlb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snseqctlb0`] module"]
#[doc(alias = "SNSEQCTLB0")]
pub type Snseqctlb0 = crate::Reg<snseqctlb0::Snseqctlb0Spec>;
#[doc = "desc SNSEQCTLB0"]
pub mod snseqctlb0;
#[doc = "DNSEQCTL0 (rw) register accessor: desc DNSEQCTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`dnseqctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dnseqctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dnseqctl0`] module"]
#[doc(alias = "DNSEQCTL0")]
pub type Dnseqctl0 = crate::Reg<dnseqctl0::Dnseqctl0Spec>;
#[doc = "desc DNSEQCTL0"]
pub mod dnseqctl0;
#[doc = "DNSEQCTLB0 (rw) register accessor: desc DNSEQCTLB0\n\nYou can [`read`](crate::Reg::read) this register and get [`dnseqctlb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dnseqctlb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dnseqctlb0`] module"]
#[doc(alias = "DNSEQCTLB0")]
pub type Dnseqctlb0 = crate::Reg<dnseqctlb0::Dnseqctlb0Spec>;
#[doc = "desc DNSEQCTLB0"]
pub mod dnseqctlb0;
#[doc = "LLP0 (rw) register accessor: desc LLP0\n\nYou can [`read`](crate::Reg::read) this register and get [`llp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp0`] module"]
#[doc(alias = "LLP0")]
pub type Llp0 = crate::Reg<llp0::Llp0Spec>;
#[doc = "desc LLP0"]
pub mod llp0;
#[doc = "CHCTL0 (rw) register accessor: desc CHCTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctl0`] module"]
#[doc(alias = "CHCTL0")]
pub type Chctl0 = crate::Reg<chctl0::Chctl0Spec>;
#[doc = "desc CHCTL0"]
pub mod chctl0;
#[doc = "MONSAR0 (r) register accessor: desc MONSAR0\n\nYou can [`read`](crate::Reg::read) this register and get [`monsar0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monsar0`] module"]
#[doc(alias = "MONSAR0")]
pub type Monsar0 = crate::Reg<monsar0::Monsar0Spec>;
#[doc = "desc MONSAR0"]
pub mod monsar0;
#[doc = "MONDAR0 (r) register accessor: desc MONDAR0\n\nYou can [`read`](crate::Reg::read) this register and get [`mondar0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mondar0`] module"]
#[doc(alias = "MONDAR0")]
pub type Mondar0 = crate::Reg<mondar0::Mondar0Spec>;
#[doc = "desc MONDAR0"]
pub mod mondar0;
#[doc = "MONDTCTL0 (r) register accessor: desc MONDTCTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`mondtctl0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mondtctl0`] module"]
#[doc(alias = "MONDTCTL0")]
pub type Mondtctl0 = crate::Reg<mondtctl0::Mondtctl0Spec>;
#[doc = "desc MONDTCTL0"]
pub mod mondtctl0;
#[doc = "MONRPT0 (r) register accessor: desc MONRPT0\n\nYou can [`read`](crate::Reg::read) this register and get [`monrpt0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monrpt0`] module"]
#[doc(alias = "MONRPT0")]
pub type Monrpt0 = crate::Reg<monrpt0::Monrpt0Spec>;
#[doc = "desc MONRPT0"]
pub mod monrpt0;
#[doc = "MONSNSEQCTL0 (r) register accessor: desc MONSNSEQCTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`monsnseqctl0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monsnseqctl0`] module"]
#[doc(alias = "MONSNSEQCTL0")]
pub type Monsnseqctl0 = crate::Reg<monsnseqctl0::Monsnseqctl0Spec>;
#[doc = "desc MONSNSEQCTL0"]
pub mod monsnseqctl0;
#[doc = "MONDNSEQCTL0 (r) register accessor: desc MONDNSEQCTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`mondnseqctl0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mondnseqctl0`] module"]
#[doc(alias = "MONDNSEQCTL0")]
pub type Mondnseqctl0 = crate::Reg<mondnseqctl0::Mondnseqctl0Spec>;
#[doc = "desc MONDNSEQCTL0"]
pub mod mondnseqctl0;
#[doc = "SAR1 (rw) register accessor: desc SAR1\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1`] module"]
#[doc(alias = "SAR1")]
pub type Sar1 = crate::Reg<sar1::Sar1Spec>;
#[doc = "desc SAR1"]
pub mod sar1;
#[doc = "DAR1 (rw) register accessor: desc DAR1\n\nYou can [`read`](crate::Reg::read) this register and get [`dar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar1`] module"]
#[doc(alias = "DAR1")]
pub type Dar1 = crate::Reg<dar1::Dar1Spec>;
#[doc = "desc DAR1"]
pub mod dar1;
#[doc = "DTCTL1 (rw) register accessor: desc DTCTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`dtctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtctl1`] module"]
#[doc(alias = "DTCTL1")]
pub type Dtctl1 = crate::Reg<dtctl1::Dtctl1Spec>;
#[doc = "desc DTCTL1"]
pub mod dtctl1;
#[doc = "RPT1 (rw) register accessor: desc RPT1\n\nYou can [`read`](crate::Reg::read) this register and get [`rpt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpt1`] module"]
#[doc(alias = "RPT1")]
pub type Rpt1 = crate::Reg<rpt1::Rpt1Spec>;
#[doc = "desc RPT1"]
pub mod rpt1;
#[doc = "RPTB1 (rw) register accessor: desc RPTB1\n\nYou can [`read`](crate::Reg::read) this register and get [`rptb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rptb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rptb1`] module"]
#[doc(alias = "RPTB1")]
pub type Rptb1 = crate::Reg<rptb1::Rptb1Spec>;
#[doc = "desc RPTB1"]
pub mod rptb1;
#[doc = "SNSEQCTL1 (rw) register accessor: desc SNSEQCTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`snseqctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snseqctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snseqctl1`] module"]
#[doc(alias = "SNSEQCTL1")]
pub type Snseqctl1 = crate::Reg<snseqctl1::Snseqctl1Spec>;
#[doc = "desc SNSEQCTL1"]
pub mod snseqctl1;
#[doc = "SNSEQCTLB1 (rw) register accessor: desc SNSEQCTLB1\n\nYou can [`read`](crate::Reg::read) this register and get [`snseqctlb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snseqctlb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snseqctlb1`] module"]
#[doc(alias = "SNSEQCTLB1")]
pub type Snseqctlb1 = crate::Reg<snseqctlb1::Snseqctlb1Spec>;
#[doc = "desc SNSEQCTLB1"]
pub mod snseqctlb1;
#[doc = "DNSEQCTL1 (rw) register accessor: desc DNSEQCTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`dnseqctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dnseqctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dnseqctl1`] module"]
#[doc(alias = "DNSEQCTL1")]
pub type Dnseqctl1 = crate::Reg<dnseqctl1::Dnseqctl1Spec>;
#[doc = "desc DNSEQCTL1"]
pub mod dnseqctl1;
#[doc = "DNSEQCTLB1 (rw) register accessor: desc DNSEQCTLB1\n\nYou can [`read`](crate::Reg::read) this register and get [`dnseqctlb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dnseqctlb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dnseqctlb1`] module"]
#[doc(alias = "DNSEQCTLB1")]
pub type Dnseqctlb1 = crate::Reg<dnseqctlb1::Dnseqctlb1Spec>;
#[doc = "desc DNSEQCTLB1"]
pub mod dnseqctlb1;
#[doc = "LLP1 (rw) register accessor: desc LLP1\n\nYou can [`read`](crate::Reg::read) this register and get [`llp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp1`] module"]
#[doc(alias = "LLP1")]
pub type Llp1 = crate::Reg<llp1::Llp1Spec>;
#[doc = "desc LLP1"]
pub mod llp1;
#[doc = "CHCTL1 (rw) register accessor: desc CHCTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctl1`] module"]
#[doc(alias = "CHCTL1")]
pub type Chctl1 = crate::Reg<chctl1::Chctl1Spec>;
#[doc = "desc CHCTL1"]
pub mod chctl1;
#[doc = "MONSAR1 (r) register accessor: desc MONSAR1\n\nYou can [`read`](crate::Reg::read) this register and get [`monsar1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monsar1`] module"]
#[doc(alias = "MONSAR1")]
pub type Monsar1 = crate::Reg<monsar1::Monsar1Spec>;
#[doc = "desc MONSAR1"]
pub mod monsar1;
#[doc = "MONDAR1 (r) register accessor: desc MONDAR1\n\nYou can [`read`](crate::Reg::read) this register and get [`mondar1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mondar1`] module"]
#[doc(alias = "MONDAR1")]
pub type Mondar1 = crate::Reg<mondar1::Mondar1Spec>;
#[doc = "desc MONDAR1"]
pub mod mondar1;
#[doc = "MONDTCTL1 (r) register accessor: desc MONDTCTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`mondtctl1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mondtctl1`] module"]
#[doc(alias = "MONDTCTL1")]
pub type Mondtctl1 = crate::Reg<mondtctl1::Mondtctl1Spec>;
#[doc = "desc MONDTCTL1"]
pub mod mondtctl1;
#[doc = "MONRPT1 (r) register accessor: desc MONRPT1\n\nYou can [`read`](crate::Reg::read) this register and get [`monrpt1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monrpt1`] module"]
#[doc(alias = "MONRPT1")]
pub type Monrpt1 = crate::Reg<monrpt1::Monrpt1Spec>;
#[doc = "desc MONRPT1"]
pub mod monrpt1;
#[doc = "MONSNSEQCTL1 (r) register accessor: desc MONSNSEQCTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`monsnseqctl1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monsnseqctl1`] module"]
#[doc(alias = "MONSNSEQCTL1")]
pub type Monsnseqctl1 = crate::Reg<monsnseqctl1::Monsnseqctl1Spec>;
#[doc = "desc MONSNSEQCTL1"]
pub mod monsnseqctl1;
#[doc = "MONDNSEQCTL1 (r) register accessor: desc MONDNSEQCTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`mondnseqctl1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mondnseqctl1`] module"]
#[doc(alias = "MONDNSEQCTL1")]
pub type Mondnseqctl1 = crate::Reg<mondnseqctl1::Mondnseqctl1Spec>;
#[doc = "desc MONDNSEQCTL1"]
pub mod mondnseqctl1;
#[doc = "SAR2 (rw) register accessor: desc SAR2\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2`] module"]
#[doc(alias = "SAR2")]
pub type Sar2 = crate::Reg<sar2::Sar2Spec>;
#[doc = "desc SAR2"]
pub mod sar2;
#[doc = "DAR2 (rw) register accessor: desc DAR2\n\nYou can [`read`](crate::Reg::read) this register and get [`dar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar2`] module"]
#[doc(alias = "DAR2")]
pub type Dar2 = crate::Reg<dar2::Dar2Spec>;
#[doc = "desc DAR2"]
pub mod dar2;
#[doc = "DTCTL2 (rw) register accessor: desc DTCTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`dtctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtctl2`] module"]
#[doc(alias = "DTCTL2")]
pub type Dtctl2 = crate::Reg<dtctl2::Dtctl2Spec>;
#[doc = "desc DTCTL2"]
pub mod dtctl2;
#[doc = "RPT2 (rw) register accessor: desc RPT2\n\nYou can [`read`](crate::Reg::read) this register and get [`rpt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpt2`] module"]
#[doc(alias = "RPT2")]
pub type Rpt2 = crate::Reg<rpt2::Rpt2Spec>;
#[doc = "desc RPT2"]
pub mod rpt2;
#[doc = "RPTB2 (rw) register accessor: desc RPTB2\n\nYou can [`read`](crate::Reg::read) this register and get [`rptb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rptb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rptb2`] module"]
#[doc(alias = "RPTB2")]
pub type Rptb2 = crate::Reg<rptb2::Rptb2Spec>;
#[doc = "desc RPTB2"]
pub mod rptb2;
#[doc = "SNSEQCTL2 (rw) register accessor: desc SNSEQCTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`snseqctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snseqctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snseqctl2`] module"]
#[doc(alias = "SNSEQCTL2")]
pub type Snseqctl2 = crate::Reg<snseqctl2::Snseqctl2Spec>;
#[doc = "desc SNSEQCTL2"]
pub mod snseqctl2;
#[doc = "SNSEQCTLB2 (rw) register accessor: desc SNSEQCTLB2\n\nYou can [`read`](crate::Reg::read) this register and get [`snseqctlb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snseqctlb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snseqctlb2`] module"]
#[doc(alias = "SNSEQCTLB2")]
pub type Snseqctlb2 = crate::Reg<snseqctlb2::Snseqctlb2Spec>;
#[doc = "desc SNSEQCTLB2"]
pub mod snseqctlb2;
#[doc = "DNSEQCTL2 (rw) register accessor: desc DNSEQCTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`dnseqctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dnseqctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dnseqctl2`] module"]
#[doc(alias = "DNSEQCTL2")]
pub type Dnseqctl2 = crate::Reg<dnseqctl2::Dnseqctl2Spec>;
#[doc = "desc DNSEQCTL2"]
pub mod dnseqctl2;
#[doc = "DNSEQCTLB2 (rw) register accessor: desc DNSEQCTLB2\n\nYou can [`read`](crate::Reg::read) this register and get [`dnseqctlb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dnseqctlb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dnseqctlb2`] module"]
#[doc(alias = "DNSEQCTLB2")]
pub type Dnseqctlb2 = crate::Reg<dnseqctlb2::Dnseqctlb2Spec>;
#[doc = "desc DNSEQCTLB2"]
pub mod dnseqctlb2;
#[doc = "LLP2 (rw) register accessor: desc LLP2\n\nYou can [`read`](crate::Reg::read) this register and get [`llp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp2`] module"]
#[doc(alias = "LLP2")]
pub type Llp2 = crate::Reg<llp2::Llp2Spec>;
#[doc = "desc LLP2"]
pub mod llp2;
#[doc = "CHCTL2 (rw) register accessor: desc CHCTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctl2`] module"]
#[doc(alias = "CHCTL2")]
pub type Chctl2 = crate::Reg<chctl2::Chctl2Spec>;
#[doc = "desc CHCTL2"]
pub mod chctl2;
#[doc = "MONSAR2 (r) register accessor: desc MONSAR2\n\nYou can [`read`](crate::Reg::read) this register and get [`monsar2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monsar2`] module"]
#[doc(alias = "MONSAR2")]
pub type Monsar2 = crate::Reg<monsar2::Monsar2Spec>;
#[doc = "desc MONSAR2"]
pub mod monsar2;
#[doc = "MONDAR2 (r) register accessor: desc MONDAR2\n\nYou can [`read`](crate::Reg::read) this register and get [`mondar2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mondar2`] module"]
#[doc(alias = "MONDAR2")]
pub type Mondar2 = crate::Reg<mondar2::Mondar2Spec>;
#[doc = "desc MONDAR2"]
pub mod mondar2;
#[doc = "MONDTCTL2 (r) register accessor: desc MONDTCTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`mondtctl2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mondtctl2`] module"]
#[doc(alias = "MONDTCTL2")]
pub type Mondtctl2 = crate::Reg<mondtctl2::Mondtctl2Spec>;
#[doc = "desc MONDTCTL2"]
pub mod mondtctl2;
#[doc = "MONRPT2 (r) register accessor: desc MONRPT2\n\nYou can [`read`](crate::Reg::read) this register and get [`monrpt2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monrpt2`] module"]
#[doc(alias = "MONRPT2")]
pub type Monrpt2 = crate::Reg<monrpt2::Monrpt2Spec>;
#[doc = "desc MONRPT2"]
pub mod monrpt2;
#[doc = "MONSNSEQCTL2 (r) register accessor: desc MONSNSEQCTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`monsnseqctl2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monsnseqctl2`] module"]
#[doc(alias = "MONSNSEQCTL2")]
pub type Monsnseqctl2 = crate::Reg<monsnseqctl2::Monsnseqctl2Spec>;
#[doc = "desc MONSNSEQCTL2"]
pub mod monsnseqctl2;
#[doc = "MONDNSEQCTL2 (r) register accessor: desc MONDNSEQCTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`mondnseqctl2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mondnseqctl2`] module"]
#[doc(alias = "MONDNSEQCTL2")]
pub type Mondnseqctl2 = crate::Reg<mondnseqctl2::Mondnseqctl2Spec>;
#[doc = "desc MONDNSEQCTL2"]
pub mod mondnseqctl2;
#[doc = "SAR3 (rw) register accessor: desc SAR3\n\nYou can [`read`](crate::Reg::read) this register and get [`sar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar3`] module"]
#[doc(alias = "SAR3")]
pub type Sar3 = crate::Reg<sar3::Sar3Spec>;
#[doc = "desc SAR3"]
pub mod sar3;
#[doc = "DAR3 (rw) register accessor: desc DAR3\n\nYou can [`read`](crate::Reg::read) this register and get [`dar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar3`] module"]
#[doc(alias = "DAR3")]
pub type Dar3 = crate::Reg<dar3::Dar3Spec>;
#[doc = "desc DAR3"]
pub mod dar3;
#[doc = "DTCTL3 (rw) register accessor: desc DTCTL3\n\nYou can [`read`](crate::Reg::read) this register and get [`dtctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtctl3`] module"]
#[doc(alias = "DTCTL3")]
pub type Dtctl3 = crate::Reg<dtctl3::Dtctl3Spec>;
#[doc = "desc DTCTL3"]
pub mod dtctl3;
#[doc = "RPT3 (rw) register accessor: desc RPT3\n\nYou can [`read`](crate::Reg::read) this register and get [`rpt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpt3`] module"]
#[doc(alias = "RPT3")]
pub type Rpt3 = crate::Reg<rpt3::Rpt3Spec>;
#[doc = "desc RPT3"]
pub mod rpt3;
#[doc = "RPTB3 (rw) register accessor: desc RPTB3\n\nYou can [`read`](crate::Reg::read) this register and get [`rptb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rptb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rptb3`] module"]
#[doc(alias = "RPTB3")]
pub type Rptb3 = crate::Reg<rptb3::Rptb3Spec>;
#[doc = "desc RPTB3"]
pub mod rptb3;
#[doc = "SNSEQCTL3 (rw) register accessor: desc SNSEQCTL3\n\nYou can [`read`](crate::Reg::read) this register and get [`snseqctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snseqctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snseqctl3`] module"]
#[doc(alias = "SNSEQCTL3")]
pub type Snseqctl3 = crate::Reg<snseqctl3::Snseqctl3Spec>;
#[doc = "desc SNSEQCTL3"]
pub mod snseqctl3;
#[doc = "SNSEQCTLB3 (rw) register accessor: desc SNSEQCTLB3\n\nYou can [`read`](crate::Reg::read) this register and get [`snseqctlb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snseqctlb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snseqctlb3`] module"]
#[doc(alias = "SNSEQCTLB3")]
pub type Snseqctlb3 = crate::Reg<snseqctlb3::Snseqctlb3Spec>;
#[doc = "desc SNSEQCTLB3"]
pub mod snseqctlb3;
#[doc = "DNSEQCTL3 (rw) register accessor: desc DNSEQCTL3\n\nYou can [`read`](crate::Reg::read) this register and get [`dnseqctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dnseqctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dnseqctl3`] module"]
#[doc(alias = "DNSEQCTL3")]
pub type Dnseqctl3 = crate::Reg<dnseqctl3::Dnseqctl3Spec>;
#[doc = "desc DNSEQCTL3"]
pub mod dnseqctl3;
#[doc = "DNSEQCTLB3 (rw) register accessor: desc DNSEQCTLB3\n\nYou can [`read`](crate::Reg::read) this register and get [`dnseqctlb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dnseqctlb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dnseqctlb3`] module"]
#[doc(alias = "DNSEQCTLB3")]
pub type Dnseqctlb3 = crate::Reg<dnseqctlb3::Dnseqctlb3Spec>;
#[doc = "desc DNSEQCTLB3"]
pub mod dnseqctlb3;
#[doc = "LLP3 (rw) register accessor: desc LLP3\n\nYou can [`read`](crate::Reg::read) this register and get [`llp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp3`] module"]
#[doc(alias = "LLP3")]
pub type Llp3 = crate::Reg<llp3::Llp3Spec>;
#[doc = "desc LLP3"]
pub mod llp3;
#[doc = "CHCTL3 (rw) register accessor: desc CHCTL3\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctl3`] module"]
#[doc(alias = "CHCTL3")]
pub type Chctl3 = crate::Reg<chctl3::Chctl3Spec>;
#[doc = "desc CHCTL3"]
pub mod chctl3;
#[doc = "MONSAR3 (r) register accessor: desc MONSAR3\n\nYou can [`read`](crate::Reg::read) this register and get [`monsar3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monsar3`] module"]
#[doc(alias = "MONSAR3")]
pub type Monsar3 = crate::Reg<monsar3::Monsar3Spec>;
#[doc = "desc MONSAR3"]
pub mod monsar3;
#[doc = "MONDAR3 (r) register accessor: desc MONDAR3\n\nYou can [`read`](crate::Reg::read) this register and get [`mondar3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mondar3`] module"]
#[doc(alias = "MONDAR3")]
pub type Mondar3 = crate::Reg<mondar3::Mondar3Spec>;
#[doc = "desc MONDAR3"]
pub mod mondar3;
#[doc = "MONDTCTL3 (r) register accessor: desc MONDTCTL3\n\nYou can [`read`](crate::Reg::read) this register and get [`mondtctl3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mondtctl3`] module"]
#[doc(alias = "MONDTCTL3")]
pub type Mondtctl3 = crate::Reg<mondtctl3::Mondtctl3Spec>;
#[doc = "desc MONDTCTL3"]
pub mod mondtctl3;
#[doc = "MONRPT3 (r) register accessor: desc MONRPT3\n\nYou can [`read`](crate::Reg::read) this register and get [`monrpt3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monrpt3`] module"]
#[doc(alias = "MONRPT3")]
pub type Monrpt3 = crate::Reg<monrpt3::Monrpt3Spec>;
#[doc = "desc MONRPT3"]
pub mod monrpt3;
#[doc = "MONSNSEQCTL3 (r) register accessor: desc MONSNSEQCTL3\n\nYou can [`read`](crate::Reg::read) this register and get [`monsnseqctl3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monsnseqctl3`] module"]
#[doc(alias = "MONSNSEQCTL3")]
pub type Monsnseqctl3 = crate::Reg<monsnseqctl3::Monsnseqctl3Spec>;
#[doc = "desc MONSNSEQCTL3"]
pub mod monsnseqctl3;
#[doc = "MONDNSEQCTL3 (r) register accessor: desc MONDNSEQCTL3\n\nYou can [`read`](crate::Reg::read) this register and get [`mondnseqctl3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mondnseqctl3`] module"]
#[doc(alias = "MONDNSEQCTL3")]
pub type Mondnseqctl3 = crate::Reg<mondnseqctl3::Mondnseqctl3Spec>;
#[doc = "desc MONDNSEQCTL3"]
pub mod mondnseqctl3;
