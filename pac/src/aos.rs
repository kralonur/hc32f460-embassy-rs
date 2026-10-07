#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intsfttrg: Intsfttrg,
    dcu_trgsel1: DcuTrgsel1,
    dcu_trgsel2: DcuTrgsel2,
    dcu_trgsel3: DcuTrgsel3,
    dcu_trgsel4: DcuTrgsel4,
    dma1_trgsel0: Dma1Trgsel0,
    dma1_trgsel1: Dma1Trgsel1,
    dma1_trgsel2: Dma1Trgsel2,
    dma1_trgsel3: Dma1Trgsel3,
    dma2_trgsel0: Dma2Trgsel0,
    dma2_trgsel1: Dma2Trgsel1,
    dma2_trgsel2: Dma2Trgsel2,
    dma2_trgsel3: Dma2Trgsel3,
    dma_rc_trgsel: DmaRcTrgsel,
    tmr6_trgsel0: Tmr6Trgsel0,
    tmr6_trgsel1: Tmr6Trgsel1,
    tmr0_trgsel: Tmr0Trgsel,
    pevnt_trgsel12: PevntTrgsel12,
    pevnt_trgsel34: PevntTrgsel34,
    tmra_trgsel0: TmraTrgsel0,
    tmra_trgsel1: TmraTrgsel1,
    ots_trgsel: OtsTrgsel,
    adc1_trgsel0: Adc1Trgsel0,
    adc1_trgsel1: Adc1Trgsel1,
    adc2_trgsel0: Adc2Trgsel0,
    adc2_trgsel1: Adc2Trgsel1,
    comtrg1: Comtrg1,
    comtrg2: Comtrg2,
    _reserved28: [u8; 0x90],
    pevntdirr1: Pevntdirr1,
    pevntidr1: Pevntidr1,
    pevntodr1: Pevntodr1,
    pevntorr1: Pevntorr1,
    pevntosr1: Pevntosr1,
    pevntrisr1: Pevntrisr1,
    pevntfalr1: Pevntfalr1,
    pevntdirr2: Pevntdirr2,
    pevntidr2: Pevntidr2,
    pevntodr2: Pevntodr2,
    pevntorr2: Pevntorr2,
    pevntosr2: Pevntosr2,
    pevntrisr2: Pevntrisr2,
    pevntfalr2: Pevntfalr2,
    pevntdirr3: Pevntdirr3,
    pevntidr3: Pevntidr3,
    pevntodr3: Pevntodr3,
    pevntorr3: Pevntorr3,
    pevntosr3: Pevntosr3,
    pevntrisr3: Pevntrisr3,
    pevntfalr3: Pevntfalr3,
    pevntdirr4: Pevntdirr4,
    pevntidr4: Pevntidr4,
    pevntodr4: Pevntodr4,
    pevntorr4: Pevntorr4,
    pevntosr4: Pevntosr4,
    pevntrisr4: Pevntrisr4,
    pevntfalr4: Pevntfalr4,
    pevntnfcr: Pevntnfcr,
}
impl RegisterBlock {
    #[doc = "0x00 - desc INTSFTTRG"]
    #[inline(always)]
    pub const fn intsfttrg(&self) -> &Intsfttrg {
        &self.intsfttrg
    }
    #[doc = "0x04 - desc DCU_TRGSEL1"]
    #[inline(always)]
    pub const fn dcu_trgsel1(&self) -> &DcuTrgsel1 {
        &self.dcu_trgsel1
    }
    #[doc = "0x08 - desc DCU_TRGSEL2"]
    #[inline(always)]
    pub const fn dcu_trgsel2(&self) -> &DcuTrgsel2 {
        &self.dcu_trgsel2
    }
    #[doc = "0x0c - desc DCU_TRGSEL3"]
    #[inline(always)]
    pub const fn dcu_trgsel3(&self) -> &DcuTrgsel3 {
        &self.dcu_trgsel3
    }
    #[doc = "0x10 - desc DCU_TRGSEL4"]
    #[inline(always)]
    pub const fn dcu_trgsel4(&self) -> &DcuTrgsel4 {
        &self.dcu_trgsel4
    }
    #[doc = "0x14 - desc DMA1_TRGSEL0"]
    #[inline(always)]
    pub const fn dma1_trgsel0(&self) -> &Dma1Trgsel0 {
        &self.dma1_trgsel0
    }
    #[doc = "0x18 - desc DMA1_TRGSEL1"]
    #[inline(always)]
    pub const fn dma1_trgsel1(&self) -> &Dma1Trgsel1 {
        &self.dma1_trgsel1
    }
    #[doc = "0x1c - desc DMA1_TRGSEL2"]
    #[inline(always)]
    pub const fn dma1_trgsel2(&self) -> &Dma1Trgsel2 {
        &self.dma1_trgsel2
    }
    #[doc = "0x20 - desc DMA1_TRGSEL3"]
    #[inline(always)]
    pub const fn dma1_trgsel3(&self) -> &Dma1Trgsel3 {
        &self.dma1_trgsel3
    }
    #[doc = "0x24 - desc DMA2_TRGSEL0"]
    #[inline(always)]
    pub const fn dma2_trgsel0(&self) -> &Dma2Trgsel0 {
        &self.dma2_trgsel0
    }
    #[doc = "0x28 - desc DMA2_TRGSEL1"]
    #[inline(always)]
    pub const fn dma2_trgsel1(&self) -> &Dma2Trgsel1 {
        &self.dma2_trgsel1
    }
    #[doc = "0x2c - desc DMA2_TRGSEL2"]
    #[inline(always)]
    pub const fn dma2_trgsel2(&self) -> &Dma2Trgsel2 {
        &self.dma2_trgsel2
    }
    #[doc = "0x30 - desc DMA2_TRGSEL3"]
    #[inline(always)]
    pub const fn dma2_trgsel3(&self) -> &Dma2Trgsel3 {
        &self.dma2_trgsel3
    }
    #[doc = "0x34 - desc DMA_RC_TRGSEL"]
    #[inline(always)]
    pub const fn dma_rc_trgsel(&self) -> &DmaRcTrgsel {
        &self.dma_rc_trgsel
    }
    #[doc = "0x38 - desc TMR6_TRGSEL0"]
    #[inline(always)]
    pub const fn tmr6_trgsel0(&self) -> &Tmr6Trgsel0 {
        &self.tmr6_trgsel0
    }
    #[doc = "0x3c - desc TMR6_TRGSEL1"]
    #[inline(always)]
    pub const fn tmr6_trgsel1(&self) -> &Tmr6Trgsel1 {
        &self.tmr6_trgsel1
    }
    #[doc = "0x40 - desc TMR0_TRGSEL"]
    #[inline(always)]
    pub const fn tmr0_trgsel(&self) -> &Tmr0Trgsel {
        &self.tmr0_trgsel
    }
    #[doc = "0x44 - desc PEVNT_TRGSEL12"]
    #[inline(always)]
    pub const fn pevnt_trgsel12(&self) -> &PevntTrgsel12 {
        &self.pevnt_trgsel12
    }
    #[doc = "0x48 - desc PEVNT_TRGSEL34"]
    #[inline(always)]
    pub const fn pevnt_trgsel34(&self) -> &PevntTrgsel34 {
        &self.pevnt_trgsel34
    }
    #[doc = "0x4c - desc TMRA_TRGSEL0"]
    #[inline(always)]
    pub const fn tmra_trgsel0(&self) -> &TmraTrgsel0 {
        &self.tmra_trgsel0
    }
    #[doc = "0x50 - desc TMRA_TRGSEL1"]
    #[inline(always)]
    pub const fn tmra_trgsel1(&self) -> &TmraTrgsel1 {
        &self.tmra_trgsel1
    }
    #[doc = "0x54 - desc OTS_TRGSEL"]
    #[inline(always)]
    pub const fn ots_trgsel(&self) -> &OtsTrgsel {
        &self.ots_trgsel
    }
    #[doc = "0x58 - desc ADC1_TRGSEL0"]
    #[inline(always)]
    pub const fn adc1_trgsel0(&self) -> &Adc1Trgsel0 {
        &self.adc1_trgsel0
    }
    #[doc = "0x5c - desc ADC1_TRGSEL1"]
    #[inline(always)]
    pub const fn adc1_trgsel1(&self) -> &Adc1Trgsel1 {
        &self.adc1_trgsel1
    }
    #[doc = "0x60 - desc ADC2_TRGSEL0"]
    #[inline(always)]
    pub const fn adc2_trgsel0(&self) -> &Adc2Trgsel0 {
        &self.adc2_trgsel0
    }
    #[doc = "0x64 - desc ADC2_TRGSEL1"]
    #[inline(always)]
    pub const fn adc2_trgsel1(&self) -> &Adc2Trgsel1 {
        &self.adc2_trgsel1
    }
    #[doc = "0x68 - desc COMTRG1"]
    #[inline(always)]
    pub const fn comtrg1(&self) -> &Comtrg1 {
        &self.comtrg1
    }
    #[doc = "0x6c - desc COMTRG2"]
    #[inline(always)]
    pub const fn comtrg2(&self) -> &Comtrg2 {
        &self.comtrg2
    }
    #[doc = "0x100 - desc PEVNTDIRR1"]
    #[inline(always)]
    pub const fn pevntdirr1(&self) -> &Pevntdirr1 {
        &self.pevntdirr1
    }
    #[doc = "0x104 - desc PEVNTIDR1"]
    #[inline(always)]
    pub const fn pevntidr1(&self) -> &Pevntidr1 {
        &self.pevntidr1
    }
    #[doc = "0x108 - desc PEVNTODR1"]
    #[inline(always)]
    pub const fn pevntodr1(&self) -> &Pevntodr1 {
        &self.pevntodr1
    }
    #[doc = "0x10c - desc PEVNTORR1"]
    #[inline(always)]
    pub const fn pevntorr1(&self) -> &Pevntorr1 {
        &self.pevntorr1
    }
    #[doc = "0x110 - desc PEVNTOSR1"]
    #[inline(always)]
    pub const fn pevntosr1(&self) -> &Pevntosr1 {
        &self.pevntosr1
    }
    #[doc = "0x114 - desc PEVNTRISR1"]
    #[inline(always)]
    pub const fn pevntrisr1(&self) -> &Pevntrisr1 {
        &self.pevntrisr1
    }
    #[doc = "0x118 - desc PEVNTFALR1"]
    #[inline(always)]
    pub const fn pevntfalr1(&self) -> &Pevntfalr1 {
        &self.pevntfalr1
    }
    #[doc = "0x11c - desc PEVNTDIRR2"]
    #[inline(always)]
    pub const fn pevntdirr2(&self) -> &Pevntdirr2 {
        &self.pevntdirr2
    }
    #[doc = "0x120 - desc PEVNTIDR2"]
    #[inline(always)]
    pub const fn pevntidr2(&self) -> &Pevntidr2 {
        &self.pevntidr2
    }
    #[doc = "0x124 - desc PEVNTODR2"]
    #[inline(always)]
    pub const fn pevntodr2(&self) -> &Pevntodr2 {
        &self.pevntodr2
    }
    #[doc = "0x128 - desc PEVNTORR2"]
    #[inline(always)]
    pub const fn pevntorr2(&self) -> &Pevntorr2 {
        &self.pevntorr2
    }
    #[doc = "0x12c - desc PEVNTOSR2"]
    #[inline(always)]
    pub const fn pevntosr2(&self) -> &Pevntosr2 {
        &self.pevntosr2
    }
    #[doc = "0x130 - desc PEVNTRISR2"]
    #[inline(always)]
    pub const fn pevntrisr2(&self) -> &Pevntrisr2 {
        &self.pevntrisr2
    }
    #[doc = "0x134 - desc PEVNTFALR2"]
    #[inline(always)]
    pub const fn pevntfalr2(&self) -> &Pevntfalr2 {
        &self.pevntfalr2
    }
    #[doc = "0x138 - desc PEVNTDIRR3"]
    #[inline(always)]
    pub const fn pevntdirr3(&self) -> &Pevntdirr3 {
        &self.pevntdirr3
    }
    #[doc = "0x13c - desc PEVNTIDR3"]
    #[inline(always)]
    pub const fn pevntidr3(&self) -> &Pevntidr3 {
        &self.pevntidr3
    }
    #[doc = "0x140 - desc PEVNTODR3"]
    #[inline(always)]
    pub const fn pevntodr3(&self) -> &Pevntodr3 {
        &self.pevntodr3
    }
    #[doc = "0x144 - desc PEVNTORR3"]
    #[inline(always)]
    pub const fn pevntorr3(&self) -> &Pevntorr3 {
        &self.pevntorr3
    }
    #[doc = "0x148 - desc PEVNTOSR3"]
    #[inline(always)]
    pub const fn pevntosr3(&self) -> &Pevntosr3 {
        &self.pevntosr3
    }
    #[doc = "0x14c - desc PEVNTRISR3"]
    #[inline(always)]
    pub const fn pevntrisr3(&self) -> &Pevntrisr3 {
        &self.pevntrisr3
    }
    #[doc = "0x150 - desc PEVNTFALR3"]
    #[inline(always)]
    pub const fn pevntfalr3(&self) -> &Pevntfalr3 {
        &self.pevntfalr3
    }
    #[doc = "0x154 - desc PEVNTDIRR4"]
    #[inline(always)]
    pub const fn pevntdirr4(&self) -> &Pevntdirr4 {
        &self.pevntdirr4
    }
    #[doc = "0x158 - desc PEVNTIDR4"]
    #[inline(always)]
    pub const fn pevntidr4(&self) -> &Pevntidr4 {
        &self.pevntidr4
    }
    #[doc = "0x15c - desc PEVNTODR4"]
    #[inline(always)]
    pub const fn pevntodr4(&self) -> &Pevntodr4 {
        &self.pevntodr4
    }
    #[doc = "0x160 - desc PEVNTORR4"]
    #[inline(always)]
    pub const fn pevntorr4(&self) -> &Pevntorr4 {
        &self.pevntorr4
    }
    #[doc = "0x164 - desc PEVNTOSR4"]
    #[inline(always)]
    pub const fn pevntosr4(&self) -> &Pevntosr4 {
        &self.pevntosr4
    }
    #[doc = "0x168 - desc PEVNTRISR4"]
    #[inline(always)]
    pub const fn pevntrisr4(&self) -> &Pevntrisr4 {
        &self.pevntrisr4
    }
    #[doc = "0x16c - desc PEVNTFALR4"]
    #[inline(always)]
    pub const fn pevntfalr4(&self) -> &Pevntfalr4 {
        &self.pevntfalr4
    }
    #[doc = "0x170 - desc PEVNTNFCR"]
    #[inline(always)]
    pub const fn pevntnfcr(&self) -> &Pevntnfcr {
        &self.pevntnfcr
    }
}
#[doc = "INTSFTTRG (w) register accessor: desc INTSFTTRG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsfttrg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsfttrg`] module"]
#[doc(alias = "INTSFTTRG")]
pub type Intsfttrg = crate::Reg<intsfttrg::IntsfttrgSpec>;
#[doc = "desc INTSFTTRG"]
pub mod intsfttrg;
#[doc = "DCU_TRGSEL1 (rw) register accessor: desc DCU_TRGSEL1\n\nYou can [`read`](crate::Reg::read) this register and get [`dcu_trgsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcu_trgsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcu_trgsel1`] module"]
#[doc(alias = "DCU_TRGSEL1")]
pub type DcuTrgsel1 = crate::Reg<dcu_trgsel1::DcuTrgsel1Spec>;
#[doc = "desc DCU_TRGSEL1"]
pub mod dcu_trgsel1;
#[doc = "DCU_TRGSEL2 (rw) register accessor: desc DCU_TRGSEL2\n\nYou can [`read`](crate::Reg::read) this register and get [`dcu_trgsel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcu_trgsel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcu_trgsel2`] module"]
#[doc(alias = "DCU_TRGSEL2")]
pub type DcuTrgsel2 = crate::Reg<dcu_trgsel2::DcuTrgsel2Spec>;
#[doc = "desc DCU_TRGSEL2"]
pub mod dcu_trgsel2;
#[doc = "DCU_TRGSEL3 (rw) register accessor: desc DCU_TRGSEL3\n\nYou can [`read`](crate::Reg::read) this register and get [`dcu_trgsel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcu_trgsel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcu_trgsel3`] module"]
#[doc(alias = "DCU_TRGSEL3")]
pub type DcuTrgsel3 = crate::Reg<dcu_trgsel3::DcuTrgsel3Spec>;
#[doc = "desc DCU_TRGSEL3"]
pub mod dcu_trgsel3;
#[doc = "DCU_TRGSEL4 (rw) register accessor: desc DCU_TRGSEL4\n\nYou can [`read`](crate::Reg::read) this register and get [`dcu_trgsel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcu_trgsel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcu_trgsel4`] module"]
#[doc(alias = "DCU_TRGSEL4")]
pub type DcuTrgsel4 = crate::Reg<dcu_trgsel4::DcuTrgsel4Spec>;
#[doc = "desc DCU_TRGSEL4"]
pub mod dcu_trgsel4;
#[doc = "DMA1_TRGSEL0 (rw) register accessor: desc DMA1_TRGSEL0\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1_trgsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_trgsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1_trgsel0`] module"]
#[doc(alias = "DMA1_TRGSEL0")]
pub type Dma1Trgsel0 = crate::Reg<dma1_trgsel0::Dma1Trgsel0Spec>;
#[doc = "desc DMA1_TRGSEL0"]
pub mod dma1_trgsel0;
#[doc = "DMA1_TRGSEL1 (rw) register accessor: desc DMA1_TRGSEL1\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1_trgsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_trgsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1_trgsel1`] module"]
#[doc(alias = "DMA1_TRGSEL1")]
pub type Dma1Trgsel1 = crate::Reg<dma1_trgsel1::Dma1Trgsel1Spec>;
#[doc = "desc DMA1_TRGSEL1"]
pub mod dma1_trgsel1;
#[doc = "DMA1_TRGSEL2 (rw) register accessor: desc DMA1_TRGSEL2\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1_trgsel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_trgsel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1_trgsel2`] module"]
#[doc(alias = "DMA1_TRGSEL2")]
pub type Dma1Trgsel2 = crate::Reg<dma1_trgsel2::Dma1Trgsel2Spec>;
#[doc = "desc DMA1_TRGSEL2"]
pub mod dma1_trgsel2;
#[doc = "DMA1_TRGSEL3 (rw) register accessor: desc DMA1_TRGSEL3\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1_trgsel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_trgsel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1_trgsel3`] module"]
#[doc(alias = "DMA1_TRGSEL3")]
pub type Dma1Trgsel3 = crate::Reg<dma1_trgsel3::Dma1Trgsel3Spec>;
#[doc = "desc DMA1_TRGSEL3"]
pub mod dma1_trgsel3;
#[doc = "DMA2_TRGSEL0 (rw) register accessor: desc DMA2_TRGSEL0\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2_trgsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2_trgsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2_trgsel0`] module"]
#[doc(alias = "DMA2_TRGSEL0")]
pub type Dma2Trgsel0 = crate::Reg<dma2_trgsel0::Dma2Trgsel0Spec>;
#[doc = "desc DMA2_TRGSEL0"]
pub mod dma2_trgsel0;
#[doc = "DMA2_TRGSEL1 (rw) register accessor: desc DMA2_TRGSEL1\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2_trgsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2_trgsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2_trgsel1`] module"]
#[doc(alias = "DMA2_TRGSEL1")]
pub type Dma2Trgsel1 = crate::Reg<dma2_trgsel1::Dma2Trgsel1Spec>;
#[doc = "desc DMA2_TRGSEL1"]
pub mod dma2_trgsel1;
#[doc = "DMA2_TRGSEL2 (rw) register accessor: desc DMA2_TRGSEL2\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2_trgsel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2_trgsel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2_trgsel2`] module"]
#[doc(alias = "DMA2_TRGSEL2")]
pub type Dma2Trgsel2 = crate::Reg<dma2_trgsel2::Dma2Trgsel2Spec>;
#[doc = "desc DMA2_TRGSEL2"]
pub mod dma2_trgsel2;
#[doc = "DMA2_TRGSEL3 (rw) register accessor: desc DMA2_TRGSEL3\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2_trgsel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2_trgsel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2_trgsel3`] module"]
#[doc(alias = "DMA2_TRGSEL3")]
pub type Dma2Trgsel3 = crate::Reg<dma2_trgsel3::Dma2Trgsel3Spec>;
#[doc = "desc DMA2_TRGSEL3"]
pub mod dma2_trgsel3;
#[doc = "DMA_RC_TRGSEL (rw) register accessor: desc DMA_RC_TRGSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_rc_trgsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rc_trgsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_rc_trgsel`] module"]
#[doc(alias = "DMA_RC_TRGSEL")]
pub type DmaRcTrgsel = crate::Reg<dma_rc_trgsel::DmaRcTrgselSpec>;
#[doc = "desc DMA_RC_TRGSEL"]
pub mod dma_rc_trgsel;
#[doc = "TMR6_TRGSEL0 (rw) register accessor: desc TMR6_TRGSEL0\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr6_trgsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr6_trgsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr6_trgsel0`] module"]
#[doc(alias = "TMR6_TRGSEL0")]
pub type Tmr6Trgsel0 = crate::Reg<tmr6_trgsel0::Tmr6Trgsel0Spec>;
#[doc = "desc TMR6_TRGSEL0"]
pub mod tmr6_trgsel0;
#[doc = "TMR6_TRGSEL1 (rw) register accessor: desc TMR6_TRGSEL1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr6_trgsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr6_trgsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr6_trgsel1`] module"]
#[doc(alias = "TMR6_TRGSEL1")]
pub type Tmr6Trgsel1 = crate::Reg<tmr6_trgsel1::Tmr6Trgsel1Spec>;
#[doc = "desc TMR6_TRGSEL1"]
pub mod tmr6_trgsel1;
#[doc = "TMR0_TRGSEL (rw) register accessor: desc TMR0_TRGSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr0_trgsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr0_trgsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr0_trgsel`] module"]
#[doc(alias = "TMR0_TRGSEL")]
pub type Tmr0Trgsel = crate::Reg<tmr0_trgsel::Tmr0TrgselSpec>;
#[doc = "desc TMR0_TRGSEL"]
pub mod tmr0_trgsel;
#[doc = "PEVNT_TRGSEL12 (rw) register accessor: desc PEVNT_TRGSEL12\n\nYou can [`read`](crate::Reg::read) this register and get [`pevnt_trgsel12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevnt_trgsel12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevnt_trgsel12`] module"]
#[doc(alias = "PEVNT_TRGSEL12")]
pub type PevntTrgsel12 = crate::Reg<pevnt_trgsel12::PevntTrgsel12Spec>;
#[doc = "desc PEVNT_TRGSEL12"]
pub mod pevnt_trgsel12;
#[doc = "PEVNT_TRGSEL34 (rw) register accessor: desc PEVNT_TRGSEL34\n\nYou can [`read`](crate::Reg::read) this register and get [`pevnt_trgsel34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevnt_trgsel34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevnt_trgsel34`] module"]
#[doc(alias = "PEVNT_TRGSEL34")]
pub type PevntTrgsel34 = crate::Reg<pevnt_trgsel34::PevntTrgsel34Spec>;
#[doc = "desc PEVNT_TRGSEL34"]
pub mod pevnt_trgsel34;
#[doc = "TMRA_TRGSEL0 (rw) register accessor: desc TMRA_TRGSEL0\n\nYou can [`read`](crate::Reg::read) this register and get [`tmra_trgsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmra_trgsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmra_trgsel0`] module"]
#[doc(alias = "TMRA_TRGSEL0")]
pub type TmraTrgsel0 = crate::Reg<tmra_trgsel0::TmraTrgsel0Spec>;
#[doc = "desc TMRA_TRGSEL0"]
pub mod tmra_trgsel0;
#[doc = "TMRA_TRGSEL1 (rw) register accessor: desc TMRA_TRGSEL1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmra_trgsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmra_trgsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmra_trgsel1`] module"]
#[doc(alias = "TMRA_TRGSEL1")]
pub type TmraTrgsel1 = crate::Reg<tmra_trgsel1::TmraTrgsel1Spec>;
#[doc = "desc TMRA_TRGSEL1"]
pub mod tmra_trgsel1;
#[doc = "OTS_TRGSEL (rw) register accessor: desc OTS_TRGSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`ots_trgsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ots_trgsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ots_trgsel`] module"]
#[doc(alias = "OTS_TRGSEL")]
pub type OtsTrgsel = crate::Reg<ots_trgsel::OtsTrgselSpec>;
#[doc = "desc OTS_TRGSEL"]
pub mod ots_trgsel;
#[doc = "ADC1_TRGSEL0 (rw) register accessor: desc ADC1_TRGSEL0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc1_trgsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc1_trgsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc1_trgsel0`] module"]
#[doc(alias = "ADC1_TRGSEL0")]
pub type Adc1Trgsel0 = crate::Reg<adc1_trgsel0::Adc1Trgsel0Spec>;
#[doc = "desc ADC1_TRGSEL0"]
pub mod adc1_trgsel0;
#[doc = "ADC1_TRGSEL1 (rw) register accessor: desc ADC1_TRGSEL1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc1_trgsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc1_trgsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc1_trgsel1`] module"]
#[doc(alias = "ADC1_TRGSEL1")]
pub type Adc1Trgsel1 = crate::Reg<adc1_trgsel1::Adc1Trgsel1Spec>;
#[doc = "desc ADC1_TRGSEL1"]
pub mod adc1_trgsel1;
#[doc = "ADC2_TRGSEL0 (rw) register accessor: desc ADC2_TRGSEL0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc2_trgsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc2_trgsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc2_trgsel0`] module"]
#[doc(alias = "ADC2_TRGSEL0")]
pub type Adc2Trgsel0 = crate::Reg<adc2_trgsel0::Adc2Trgsel0Spec>;
#[doc = "desc ADC2_TRGSEL0"]
pub mod adc2_trgsel0;
#[doc = "ADC2_TRGSEL1 (rw) register accessor: desc ADC2_TRGSEL1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc2_trgsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc2_trgsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc2_trgsel1`] module"]
#[doc(alias = "ADC2_TRGSEL1")]
pub type Adc2Trgsel1 = crate::Reg<adc2_trgsel1::Adc2Trgsel1Spec>;
#[doc = "desc ADC2_TRGSEL1"]
pub mod adc2_trgsel1;
#[doc = "COMTRG1 (rw) register accessor: desc COMTRG1\n\nYou can [`read`](crate::Reg::read) this register and get [`comtrg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comtrg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comtrg1`] module"]
#[doc(alias = "COMTRG1")]
pub type Comtrg1 = crate::Reg<comtrg1::Comtrg1Spec>;
#[doc = "desc COMTRG1"]
pub mod comtrg1;
#[doc = "COMTRG2 (rw) register accessor: desc COMTRG2\n\nYou can [`read`](crate::Reg::read) this register and get [`comtrg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comtrg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comtrg2`] module"]
#[doc(alias = "COMTRG2")]
pub type Comtrg2 = crate::Reg<comtrg2::Comtrg2Spec>;
#[doc = "desc COMTRG2"]
pub mod comtrg2;
#[doc = "PEVNTDIRR1 (rw) register accessor: desc PEVNTDIRR1\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntdirr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntdirr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntdirr1`] module"]
#[doc(alias = "PEVNTDIRR1")]
pub type Pevntdirr1 = crate::Reg<pevntdirr1::Pevntdirr1Spec>;
#[doc = "desc PEVNTDIRR1"]
pub mod pevntdirr1;
#[doc = "PEVNTIDR1 (r) register accessor: desc PEVNTIDR1\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntidr1`] module"]
#[doc(alias = "PEVNTIDR1")]
pub type Pevntidr1 = crate::Reg<pevntidr1::Pevntidr1Spec>;
#[doc = "desc PEVNTIDR1"]
pub mod pevntidr1;
#[doc = "PEVNTODR1 (rw) register accessor: desc PEVNTODR1\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntodr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntodr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntodr1`] module"]
#[doc(alias = "PEVNTODR1")]
pub type Pevntodr1 = crate::Reg<pevntodr1::Pevntodr1Spec>;
#[doc = "desc PEVNTODR1"]
pub mod pevntodr1;
#[doc = "PEVNTORR1 (rw) register accessor: desc PEVNTORR1\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntorr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntorr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntorr1`] module"]
#[doc(alias = "PEVNTORR1")]
pub type Pevntorr1 = crate::Reg<pevntorr1::Pevntorr1Spec>;
#[doc = "desc PEVNTORR1"]
pub mod pevntorr1;
#[doc = "PEVNTOSR1 (rw) register accessor: desc PEVNTOSR1\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntosr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntosr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntosr1`] module"]
#[doc(alias = "PEVNTOSR1")]
pub type Pevntosr1 = crate::Reg<pevntosr1::Pevntosr1Spec>;
#[doc = "desc PEVNTOSR1"]
pub mod pevntosr1;
#[doc = "PEVNTRISR1 (rw) register accessor: desc PEVNTRISR1\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntrisr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntrisr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntrisr1`] module"]
#[doc(alias = "PEVNTRISR1")]
pub type Pevntrisr1 = crate::Reg<pevntrisr1::Pevntrisr1Spec>;
#[doc = "desc PEVNTRISR1"]
pub mod pevntrisr1;
#[doc = "PEVNTFALR1 (rw) register accessor: desc PEVNTFALR1\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntfalr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntfalr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntfalr1`] module"]
#[doc(alias = "PEVNTFALR1")]
pub type Pevntfalr1 = crate::Reg<pevntfalr1::Pevntfalr1Spec>;
#[doc = "desc PEVNTFALR1"]
pub mod pevntfalr1;
#[doc = "PEVNTDIRR2 (rw) register accessor: desc PEVNTDIRR2\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntdirr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntdirr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntdirr2`] module"]
#[doc(alias = "PEVNTDIRR2")]
pub type Pevntdirr2 = crate::Reg<pevntdirr2::Pevntdirr2Spec>;
#[doc = "desc PEVNTDIRR2"]
pub mod pevntdirr2;
#[doc = "PEVNTIDR2 (r) register accessor: desc PEVNTIDR2\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntidr2`] module"]
#[doc(alias = "PEVNTIDR2")]
pub type Pevntidr2 = crate::Reg<pevntidr2::Pevntidr2Spec>;
#[doc = "desc PEVNTIDR2"]
pub mod pevntidr2;
#[doc = "PEVNTODR2 (rw) register accessor: desc PEVNTODR2\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntodr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntodr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntodr2`] module"]
#[doc(alias = "PEVNTODR2")]
pub type Pevntodr2 = crate::Reg<pevntodr2::Pevntodr2Spec>;
#[doc = "desc PEVNTODR2"]
pub mod pevntodr2;
#[doc = "PEVNTORR2 (rw) register accessor: desc PEVNTORR2\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntorr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntorr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntorr2`] module"]
#[doc(alias = "PEVNTORR2")]
pub type Pevntorr2 = crate::Reg<pevntorr2::Pevntorr2Spec>;
#[doc = "desc PEVNTORR2"]
pub mod pevntorr2;
#[doc = "PEVNTOSR2 (rw) register accessor: desc PEVNTOSR2\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntosr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntosr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntosr2`] module"]
#[doc(alias = "PEVNTOSR2")]
pub type Pevntosr2 = crate::Reg<pevntosr2::Pevntosr2Spec>;
#[doc = "desc PEVNTOSR2"]
pub mod pevntosr2;
#[doc = "PEVNTRISR2 (rw) register accessor: desc PEVNTRISR2\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntrisr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntrisr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntrisr2`] module"]
#[doc(alias = "PEVNTRISR2")]
pub type Pevntrisr2 = crate::Reg<pevntrisr2::Pevntrisr2Spec>;
#[doc = "desc PEVNTRISR2"]
pub mod pevntrisr2;
#[doc = "PEVNTFALR2 (rw) register accessor: desc PEVNTFALR2\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntfalr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntfalr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntfalr2`] module"]
#[doc(alias = "PEVNTFALR2")]
pub type Pevntfalr2 = crate::Reg<pevntfalr2::Pevntfalr2Spec>;
#[doc = "desc PEVNTFALR2"]
pub mod pevntfalr2;
#[doc = "PEVNTDIRR3 (rw) register accessor: desc PEVNTDIRR3\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntdirr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntdirr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntdirr3`] module"]
#[doc(alias = "PEVNTDIRR3")]
pub type Pevntdirr3 = crate::Reg<pevntdirr3::Pevntdirr3Spec>;
#[doc = "desc PEVNTDIRR3"]
pub mod pevntdirr3;
#[doc = "PEVNTIDR3 (r) register accessor: desc PEVNTIDR3\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntidr3`] module"]
#[doc(alias = "PEVNTIDR3")]
pub type Pevntidr3 = crate::Reg<pevntidr3::Pevntidr3Spec>;
#[doc = "desc PEVNTIDR3"]
pub mod pevntidr3;
#[doc = "PEVNTODR3 (rw) register accessor: desc PEVNTODR3\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntodr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntodr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntodr3`] module"]
#[doc(alias = "PEVNTODR3")]
pub type Pevntodr3 = crate::Reg<pevntodr3::Pevntodr3Spec>;
#[doc = "desc PEVNTODR3"]
pub mod pevntodr3;
#[doc = "PEVNTORR3 (rw) register accessor: desc PEVNTORR3\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntorr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntorr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntorr3`] module"]
#[doc(alias = "PEVNTORR3")]
pub type Pevntorr3 = crate::Reg<pevntorr3::Pevntorr3Spec>;
#[doc = "desc PEVNTORR3"]
pub mod pevntorr3;
#[doc = "PEVNTOSR3 (rw) register accessor: desc PEVNTOSR3\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntosr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntosr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntosr3`] module"]
#[doc(alias = "PEVNTOSR3")]
pub type Pevntosr3 = crate::Reg<pevntosr3::Pevntosr3Spec>;
#[doc = "desc PEVNTOSR3"]
pub mod pevntosr3;
#[doc = "PEVNTRISR3 (rw) register accessor: desc PEVNTRISR3\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntrisr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntrisr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntrisr3`] module"]
#[doc(alias = "PEVNTRISR3")]
pub type Pevntrisr3 = crate::Reg<pevntrisr3::Pevntrisr3Spec>;
#[doc = "desc PEVNTRISR3"]
pub mod pevntrisr3;
#[doc = "PEVNTFALR3 (rw) register accessor: desc PEVNTFALR3\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntfalr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntfalr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntfalr3`] module"]
#[doc(alias = "PEVNTFALR3")]
pub type Pevntfalr3 = crate::Reg<pevntfalr3::Pevntfalr3Spec>;
#[doc = "desc PEVNTFALR3"]
pub mod pevntfalr3;
#[doc = "PEVNTDIRR4 (rw) register accessor: desc PEVNTDIRR4\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntdirr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntdirr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntdirr4`] module"]
#[doc(alias = "PEVNTDIRR4")]
pub type Pevntdirr4 = crate::Reg<pevntdirr4::Pevntdirr4Spec>;
#[doc = "desc PEVNTDIRR4"]
pub mod pevntdirr4;
#[doc = "PEVNTIDR4 (r) register accessor: desc PEVNTIDR4\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntidr4`] module"]
#[doc(alias = "PEVNTIDR4")]
pub type Pevntidr4 = crate::Reg<pevntidr4::Pevntidr4Spec>;
#[doc = "desc PEVNTIDR4"]
pub mod pevntidr4;
#[doc = "PEVNTODR4 (rw) register accessor: desc PEVNTODR4\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntodr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntodr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntodr4`] module"]
#[doc(alias = "PEVNTODR4")]
pub type Pevntodr4 = crate::Reg<pevntodr4::Pevntodr4Spec>;
#[doc = "desc PEVNTODR4"]
pub mod pevntodr4;
#[doc = "PEVNTORR4 (rw) register accessor: desc PEVNTORR4\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntorr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntorr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntorr4`] module"]
#[doc(alias = "PEVNTORR4")]
pub type Pevntorr4 = crate::Reg<pevntorr4::Pevntorr4Spec>;
#[doc = "desc PEVNTORR4"]
pub mod pevntorr4;
#[doc = "PEVNTOSR4 (rw) register accessor: desc PEVNTOSR4\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntosr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntosr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntosr4`] module"]
#[doc(alias = "PEVNTOSR4")]
pub type Pevntosr4 = crate::Reg<pevntosr4::Pevntosr4Spec>;
#[doc = "desc PEVNTOSR4"]
pub mod pevntosr4;
#[doc = "PEVNTRISR4 (rw) register accessor: desc PEVNTRISR4\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntrisr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntrisr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntrisr4`] module"]
#[doc(alias = "PEVNTRISR4")]
pub type Pevntrisr4 = crate::Reg<pevntrisr4::Pevntrisr4Spec>;
#[doc = "desc PEVNTRISR4"]
pub mod pevntrisr4;
#[doc = "PEVNTFALR4 (rw) register accessor: desc PEVNTFALR4\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntfalr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntfalr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntfalr4`] module"]
#[doc(alias = "PEVNTFALR4")]
pub type Pevntfalr4 = crate::Reg<pevntfalr4::Pevntfalr4Spec>;
#[doc = "desc PEVNTFALR4"]
pub mod pevntfalr4;
#[doc = "PEVNTNFCR (rw) register accessor: desc PEVNTNFCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntnfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntnfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pevntnfcr`] module"]
#[doc(alias = "PEVNTNFCR")]
pub type Pevntnfcr = crate::Reg<pevntnfcr::PevntnfcrSpec>;
#[doc = "desc PEVNTNFCR"]
pub mod pevntnfcr;
