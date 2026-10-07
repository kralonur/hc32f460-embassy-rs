#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cnter: Cnter,
    _reserved1: [u8; 0x02],
    perar: Perar,
    _reserved2: [u8; 0x3a],
    cmpar1: Cmpar1,
    _reserved3: [u8; 0x02],
    cmpar2: Cmpar2,
    _reserved4: [u8; 0x02],
    cmpar3: Cmpar3,
    _reserved5: [u8; 0x02],
    cmpar4: Cmpar4,
    _reserved6: [u8; 0x02],
    cmpar5: Cmpar5,
    _reserved7: [u8; 0x02],
    cmpar6: Cmpar6,
    _reserved8: [u8; 0x02],
    cmpar7: Cmpar7,
    _reserved9: [u8; 0x02],
    cmpar8: Cmpar8,
    _reserved10: [u8; 0x22],
    bcstrl: Bcstrl,
    bcstrh: Bcstrh,
    _reserved12: [u8; 0x02],
    hconr: Hconr,
    _reserved13: [u8; 0x02],
    hcupr: Hcupr,
    _reserved14: [u8; 0x02],
    hcdor: Hcdor,
    _reserved15: [u8; 0x02],
    iconr: Iconr,
    _reserved16: [u8; 0x02],
    econr: Econr,
    _reserved17: [u8; 0x02],
    fconr: Fconr,
    _reserved18: [u8; 0x02],
    stflr: Stflr,
    _reserved19: [u8; 0x22],
    bconr1: Bconr1,
    _reserved20: [u8; 0x06],
    bconr2: Bconr2,
    _reserved21: [u8; 0x06],
    bconr3: Bconr3,
    _reserved22: [u8; 0x06],
    bconr4: Bconr4,
    _reserved23: [u8; 0x26],
    cconr1: Cconr1,
    _reserved24: [u8; 0x02],
    cconr2: Cconr2,
    _reserved25: [u8; 0x02],
    cconr3: Cconr3,
    _reserved26: [u8; 0x02],
    cconr4: Cconr4,
    _reserved27: [u8; 0x02],
    cconr5: Cconr5,
    _reserved28: [u8; 0x02],
    cconr6: Cconr6,
    _reserved29: [u8; 0x02],
    cconr7: Cconr7,
    _reserved30: [u8; 0x02],
    cconr8: Cconr8,
    _reserved31: [u8; 0x22],
    pconr1: Pconr1,
    _reserved32: [u8; 0x02],
    pconr2: Pconr2,
    _reserved33: [u8; 0x02],
    pconr3: Pconr3,
    _reserved34: [u8; 0x02],
    pconr4: Pconr4,
    _reserved35: [u8; 0x02],
    pconr5: Pconr5,
    _reserved36: [u8; 0x02],
    pconr6: Pconr6,
    _reserved37: [u8; 0x02],
    pconr7: Pconr7,
    _reserved38: [u8; 0x02],
    pconr8: Pconr8,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CNTER"]
    #[inline(always)]
    pub const fn cnter(&self) -> &Cnter {
        &self.cnter
    }
    #[doc = "0x04 - desc PERAR"]
    #[inline(always)]
    pub const fn perar(&self) -> &Perar {
        &self.perar
    }
    #[doc = "0x40 - desc CMPAR1"]
    #[inline(always)]
    pub const fn cmpar1(&self) -> &Cmpar1 {
        &self.cmpar1
    }
    #[doc = "0x44 - desc CMPAR2"]
    #[inline(always)]
    pub const fn cmpar2(&self) -> &Cmpar2 {
        &self.cmpar2
    }
    #[doc = "0x48 - desc CMPAR3"]
    #[inline(always)]
    pub const fn cmpar3(&self) -> &Cmpar3 {
        &self.cmpar3
    }
    #[doc = "0x4c - desc CMPAR4"]
    #[inline(always)]
    pub const fn cmpar4(&self) -> &Cmpar4 {
        &self.cmpar4
    }
    #[doc = "0x50 - desc CMPAR5"]
    #[inline(always)]
    pub const fn cmpar5(&self) -> &Cmpar5 {
        &self.cmpar5
    }
    #[doc = "0x54 - desc CMPAR6"]
    #[inline(always)]
    pub const fn cmpar6(&self) -> &Cmpar6 {
        &self.cmpar6
    }
    #[doc = "0x58 - desc CMPAR7"]
    #[inline(always)]
    pub const fn cmpar7(&self) -> &Cmpar7 {
        &self.cmpar7
    }
    #[doc = "0x5c - desc CMPAR8"]
    #[inline(always)]
    pub const fn cmpar8(&self) -> &Cmpar8 {
        &self.cmpar8
    }
    #[doc = "0x80 - desc BCSTRL"]
    #[inline(always)]
    pub const fn bcstrl(&self) -> &Bcstrl {
        &self.bcstrl
    }
    #[doc = "0x81 - desc BCSTRH"]
    #[inline(always)]
    pub const fn bcstrh(&self) -> &Bcstrh {
        &self.bcstrh
    }
    #[doc = "0x84 - desc HCONR"]
    #[inline(always)]
    pub const fn hconr(&self) -> &Hconr {
        &self.hconr
    }
    #[doc = "0x88 - desc HCUPR"]
    #[inline(always)]
    pub const fn hcupr(&self) -> &Hcupr {
        &self.hcupr
    }
    #[doc = "0x8c - desc HCDOR"]
    #[inline(always)]
    pub const fn hcdor(&self) -> &Hcdor {
        &self.hcdor
    }
    #[doc = "0x90 - desc ICONR"]
    #[inline(always)]
    pub const fn iconr(&self) -> &Iconr {
        &self.iconr
    }
    #[doc = "0x94 - desc ECONR"]
    #[inline(always)]
    pub const fn econr(&self) -> &Econr {
        &self.econr
    }
    #[doc = "0x98 - desc FCONR"]
    #[inline(always)]
    pub const fn fconr(&self) -> &Fconr {
        &self.fconr
    }
    #[doc = "0x9c - desc STFLR"]
    #[inline(always)]
    pub const fn stflr(&self) -> &Stflr {
        &self.stflr
    }
    #[doc = "0xc0 - desc BCONR1"]
    #[inline(always)]
    pub const fn bconr1(&self) -> &Bconr1 {
        &self.bconr1
    }
    #[doc = "0xc8 - desc BCONR2"]
    #[inline(always)]
    pub const fn bconr2(&self) -> &Bconr2 {
        &self.bconr2
    }
    #[doc = "0xd0 - desc BCONR3"]
    #[inline(always)]
    pub const fn bconr3(&self) -> &Bconr3 {
        &self.bconr3
    }
    #[doc = "0xd8 - desc BCONR4"]
    #[inline(always)]
    pub const fn bconr4(&self) -> &Bconr4 {
        &self.bconr4
    }
    #[doc = "0x100 - desc CCONR1"]
    #[inline(always)]
    pub const fn cconr1(&self) -> &Cconr1 {
        &self.cconr1
    }
    #[doc = "0x104 - desc CCONR2"]
    #[inline(always)]
    pub const fn cconr2(&self) -> &Cconr2 {
        &self.cconr2
    }
    #[doc = "0x108 - desc CCONR3"]
    #[inline(always)]
    pub const fn cconr3(&self) -> &Cconr3 {
        &self.cconr3
    }
    #[doc = "0x10c - desc CCONR4"]
    #[inline(always)]
    pub const fn cconr4(&self) -> &Cconr4 {
        &self.cconr4
    }
    #[doc = "0x110 - desc CCONR5"]
    #[inline(always)]
    pub const fn cconr5(&self) -> &Cconr5 {
        &self.cconr5
    }
    #[doc = "0x114 - desc CCONR6"]
    #[inline(always)]
    pub const fn cconr6(&self) -> &Cconr6 {
        &self.cconr6
    }
    #[doc = "0x118 - desc CCONR7"]
    #[inline(always)]
    pub const fn cconr7(&self) -> &Cconr7 {
        &self.cconr7
    }
    #[doc = "0x11c - desc CCONR8"]
    #[inline(always)]
    pub const fn cconr8(&self) -> &Cconr8 {
        &self.cconr8
    }
    #[doc = "0x140 - desc PCONR1"]
    #[inline(always)]
    pub const fn pconr1(&self) -> &Pconr1 {
        &self.pconr1
    }
    #[doc = "0x144 - desc PCONR2"]
    #[inline(always)]
    pub const fn pconr2(&self) -> &Pconr2 {
        &self.pconr2
    }
    #[doc = "0x148 - desc PCONR3"]
    #[inline(always)]
    pub const fn pconr3(&self) -> &Pconr3 {
        &self.pconr3
    }
    #[doc = "0x14c - desc PCONR4"]
    #[inline(always)]
    pub const fn pconr4(&self) -> &Pconr4 {
        &self.pconr4
    }
    #[doc = "0x150 - desc PCONR5"]
    #[inline(always)]
    pub const fn pconr5(&self) -> &Pconr5 {
        &self.pconr5
    }
    #[doc = "0x154 - desc PCONR6"]
    #[inline(always)]
    pub const fn pconr6(&self) -> &Pconr6 {
        &self.pconr6
    }
    #[doc = "0x158 - desc PCONR7"]
    #[inline(always)]
    pub const fn pconr7(&self) -> &Pconr7 {
        &self.pconr7
    }
    #[doc = "0x15c - desc PCONR8"]
    #[inline(always)]
    pub const fn pconr8(&self) -> &Pconr8 {
        &self.pconr8
    }
}
#[doc = "CNTER (rw) register accessor: desc CNTER\n\nYou can [`read`](crate::Reg::read) this register and get [`cnter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnter`] module"]
#[doc(alias = "CNTER")]
pub type Cnter = crate::Reg<cnter::CnterSpec>;
#[doc = "desc CNTER"]
pub mod cnter;
#[doc = "PERAR (rw) register accessor: desc PERAR\n\nYou can [`read`](crate::Reg::read) this register and get [`perar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perar`] module"]
#[doc(alias = "PERAR")]
pub type Perar = crate::Reg<perar::PerarSpec>;
#[doc = "desc PERAR"]
pub mod perar;
#[doc = "CMPAR1 (rw) register accessor: desc CMPAR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpar1`] module"]
#[doc(alias = "CMPAR1")]
pub type Cmpar1 = crate::Reg<cmpar1::Cmpar1Spec>;
#[doc = "desc CMPAR1"]
pub mod cmpar1;
#[doc = "CMPAR2 (rw) register accessor: desc CMPAR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpar2`] module"]
#[doc(alias = "CMPAR2")]
pub type Cmpar2 = crate::Reg<cmpar2::Cmpar2Spec>;
#[doc = "desc CMPAR2"]
pub mod cmpar2;
#[doc = "CMPAR3 (rw) register accessor: desc CMPAR3\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpar3`] module"]
#[doc(alias = "CMPAR3")]
pub type Cmpar3 = crate::Reg<cmpar3::Cmpar3Spec>;
#[doc = "desc CMPAR3"]
pub mod cmpar3;
#[doc = "CMPAR4 (rw) register accessor: desc CMPAR4\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpar4`] module"]
#[doc(alias = "CMPAR4")]
pub type Cmpar4 = crate::Reg<cmpar4::Cmpar4Spec>;
#[doc = "desc CMPAR4"]
pub mod cmpar4;
#[doc = "CMPAR5 (rw) register accessor: desc CMPAR5\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpar5`] module"]
#[doc(alias = "CMPAR5")]
pub type Cmpar5 = crate::Reg<cmpar5::Cmpar5Spec>;
#[doc = "desc CMPAR5"]
pub mod cmpar5;
#[doc = "CMPAR6 (rw) register accessor: desc CMPAR6\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpar6`] module"]
#[doc(alias = "CMPAR6")]
pub type Cmpar6 = crate::Reg<cmpar6::Cmpar6Spec>;
#[doc = "desc CMPAR6"]
pub mod cmpar6;
#[doc = "CMPAR7 (rw) register accessor: desc CMPAR7\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpar7`] module"]
#[doc(alias = "CMPAR7")]
pub type Cmpar7 = crate::Reg<cmpar7::Cmpar7Spec>;
#[doc = "desc CMPAR7"]
pub mod cmpar7;
#[doc = "CMPAR8 (rw) register accessor: desc CMPAR8\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpar8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpar8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpar8`] module"]
#[doc(alias = "CMPAR8")]
pub type Cmpar8 = crate::Reg<cmpar8::Cmpar8Spec>;
#[doc = "desc CMPAR8"]
pub mod cmpar8;
#[doc = "BCSTRL (rw) register accessor: desc BCSTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`bcstrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcstrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcstrl`] module"]
#[doc(alias = "BCSTRL")]
pub type Bcstrl = crate::Reg<bcstrl::BcstrlSpec>;
#[doc = "desc BCSTRL"]
pub mod bcstrl;
#[doc = "BCSTRH (rw) register accessor: desc BCSTRH\n\nYou can [`read`](crate::Reg::read) this register and get [`bcstrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcstrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcstrh`] module"]
#[doc(alias = "BCSTRH")]
pub type Bcstrh = crate::Reg<bcstrh::BcstrhSpec>;
#[doc = "desc BCSTRH"]
pub mod bcstrh;
#[doc = "HCONR (rw) register accessor: desc HCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`hconr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hconr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hconr`] module"]
#[doc(alias = "HCONR")]
pub type Hconr = crate::Reg<hconr::HconrSpec>;
#[doc = "desc HCONR"]
pub mod hconr;
#[doc = "HCUPR (rw) register accessor: desc HCUPR\n\nYou can [`read`](crate::Reg::read) this register and get [`hcupr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcupr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcupr`] module"]
#[doc(alias = "HCUPR")]
pub type Hcupr = crate::Reg<hcupr::HcuprSpec>;
#[doc = "desc HCUPR"]
pub mod hcupr;
#[doc = "HCDOR (rw) register accessor: desc HCDOR\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdor`] module"]
#[doc(alias = "HCDOR")]
pub type Hcdor = crate::Reg<hcdor::HcdorSpec>;
#[doc = "desc HCDOR"]
pub mod hcdor;
#[doc = "ICONR (rw) register accessor: desc ICONR\n\nYou can [`read`](crate::Reg::read) this register and get [`iconr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iconr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iconr`] module"]
#[doc(alias = "ICONR")]
pub type Iconr = crate::Reg<iconr::IconrSpec>;
#[doc = "desc ICONR"]
pub mod iconr;
#[doc = "ECONR (rw) register accessor: desc ECONR\n\nYou can [`read`](crate::Reg::read) this register and get [`econr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`econr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@econr`] module"]
#[doc(alias = "ECONR")]
pub type Econr = crate::Reg<econr::EconrSpec>;
#[doc = "desc ECONR"]
pub mod econr;
#[doc = "FCONR (rw) register accessor: desc FCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`fconr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fconr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fconr`] module"]
#[doc(alias = "FCONR")]
pub type Fconr = crate::Reg<fconr::FconrSpec>;
#[doc = "desc FCONR"]
pub mod fconr;
#[doc = "STFLR (rw) register accessor: desc STFLR\n\nYou can [`read`](crate::Reg::read) this register and get [`stflr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stflr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stflr`] module"]
#[doc(alias = "STFLR")]
pub type Stflr = crate::Reg<stflr::StflrSpec>;
#[doc = "desc STFLR"]
pub mod stflr;
#[doc = "BCONR1 (rw) register accessor: desc BCONR1\n\nYou can [`read`](crate::Reg::read) this register and get [`bconr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bconr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bconr1`] module"]
#[doc(alias = "BCONR1")]
pub type Bconr1 = crate::Reg<bconr1::Bconr1Spec>;
#[doc = "desc BCONR1"]
pub mod bconr1;
#[doc = "BCONR2 (rw) register accessor: desc BCONR2\n\nYou can [`read`](crate::Reg::read) this register and get [`bconr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bconr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bconr2`] module"]
#[doc(alias = "BCONR2")]
pub type Bconr2 = crate::Reg<bconr2::Bconr2Spec>;
#[doc = "desc BCONR2"]
pub mod bconr2;
#[doc = "BCONR3 (rw) register accessor: desc BCONR3\n\nYou can [`read`](crate::Reg::read) this register and get [`bconr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bconr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bconr3`] module"]
#[doc(alias = "BCONR3")]
pub type Bconr3 = crate::Reg<bconr3::Bconr3Spec>;
#[doc = "desc BCONR3"]
pub mod bconr3;
#[doc = "BCONR4 (rw) register accessor: desc BCONR4\n\nYou can [`read`](crate::Reg::read) this register and get [`bconr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bconr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bconr4`] module"]
#[doc(alias = "BCONR4")]
pub type Bconr4 = crate::Reg<bconr4::Bconr4Spec>;
#[doc = "desc BCONR4"]
pub mod bconr4;
#[doc = "CCONR1 (rw) register accessor: desc CCONR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cconr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cconr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cconr1`] module"]
#[doc(alias = "CCONR1")]
pub type Cconr1 = crate::Reg<cconr1::Cconr1Spec>;
#[doc = "desc CCONR1"]
pub mod cconr1;
#[doc = "CCONR2 (rw) register accessor: desc CCONR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cconr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cconr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cconr2`] module"]
#[doc(alias = "CCONR2")]
pub type Cconr2 = crate::Reg<cconr2::Cconr2Spec>;
#[doc = "desc CCONR2"]
pub mod cconr2;
#[doc = "CCONR3 (rw) register accessor: desc CCONR3\n\nYou can [`read`](crate::Reg::read) this register and get [`cconr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cconr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cconr3`] module"]
#[doc(alias = "CCONR3")]
pub type Cconr3 = crate::Reg<cconr3::Cconr3Spec>;
#[doc = "desc CCONR3"]
pub mod cconr3;
#[doc = "CCONR4 (rw) register accessor: desc CCONR4\n\nYou can [`read`](crate::Reg::read) this register and get [`cconr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cconr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cconr4`] module"]
#[doc(alias = "CCONR4")]
pub type Cconr4 = crate::Reg<cconr4::Cconr4Spec>;
#[doc = "desc CCONR4"]
pub mod cconr4;
#[doc = "CCONR5 (rw) register accessor: desc CCONR5\n\nYou can [`read`](crate::Reg::read) this register and get [`cconr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cconr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cconr5`] module"]
#[doc(alias = "CCONR5")]
pub type Cconr5 = crate::Reg<cconr5::Cconr5Spec>;
#[doc = "desc CCONR5"]
pub mod cconr5;
#[doc = "CCONR6 (rw) register accessor: desc CCONR6\n\nYou can [`read`](crate::Reg::read) this register and get [`cconr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cconr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cconr6`] module"]
#[doc(alias = "CCONR6")]
pub type Cconr6 = crate::Reg<cconr6::Cconr6Spec>;
#[doc = "desc CCONR6"]
pub mod cconr6;
#[doc = "CCONR7 (rw) register accessor: desc CCONR7\n\nYou can [`read`](crate::Reg::read) this register and get [`cconr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cconr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cconr7`] module"]
#[doc(alias = "CCONR7")]
pub type Cconr7 = crate::Reg<cconr7::Cconr7Spec>;
#[doc = "desc CCONR7"]
pub mod cconr7;
#[doc = "CCONR8 (rw) register accessor: desc CCONR8\n\nYou can [`read`](crate::Reg::read) this register and get [`cconr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cconr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cconr8`] module"]
#[doc(alias = "CCONR8")]
pub type Cconr8 = crate::Reg<cconr8::Cconr8Spec>;
#[doc = "desc CCONR8"]
pub mod cconr8;
#[doc = "PCONR1 (rw) register accessor: desc PCONR1\n\nYou can [`read`](crate::Reg::read) this register and get [`pconr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconr1`] module"]
#[doc(alias = "PCONR1")]
pub type Pconr1 = crate::Reg<pconr1::Pconr1Spec>;
#[doc = "desc PCONR1"]
pub mod pconr1;
#[doc = "PCONR2 (rw) register accessor: desc PCONR2\n\nYou can [`read`](crate::Reg::read) this register and get [`pconr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconr2`] module"]
#[doc(alias = "PCONR2")]
pub type Pconr2 = crate::Reg<pconr2::Pconr2Spec>;
#[doc = "desc PCONR2"]
pub mod pconr2;
#[doc = "PCONR3 (rw) register accessor: desc PCONR3\n\nYou can [`read`](crate::Reg::read) this register and get [`pconr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconr3`] module"]
#[doc(alias = "PCONR3")]
pub type Pconr3 = crate::Reg<pconr3::Pconr3Spec>;
#[doc = "desc PCONR3"]
pub mod pconr3;
#[doc = "PCONR4 (rw) register accessor: desc PCONR4\n\nYou can [`read`](crate::Reg::read) this register and get [`pconr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconr4`] module"]
#[doc(alias = "PCONR4")]
pub type Pconr4 = crate::Reg<pconr4::Pconr4Spec>;
#[doc = "desc PCONR4"]
pub mod pconr4;
#[doc = "PCONR5 (rw) register accessor: desc PCONR5\n\nYou can [`read`](crate::Reg::read) this register and get [`pconr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconr5`] module"]
#[doc(alias = "PCONR5")]
pub type Pconr5 = crate::Reg<pconr5::Pconr5Spec>;
#[doc = "desc PCONR5"]
pub mod pconr5;
#[doc = "PCONR6 (rw) register accessor: desc PCONR6\n\nYou can [`read`](crate::Reg::read) this register and get [`pconr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconr6`] module"]
#[doc(alias = "PCONR6")]
pub type Pconr6 = crate::Reg<pconr6::Pconr6Spec>;
#[doc = "desc PCONR6"]
pub mod pconr6;
#[doc = "PCONR7 (rw) register accessor: desc PCONR7\n\nYou can [`read`](crate::Reg::read) this register and get [`pconr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconr7`] module"]
#[doc(alias = "PCONR7")]
pub type Pconr7 = crate::Reg<pconr7::Pconr7Spec>;
#[doc = "desc PCONR7"]
pub mod pconr7;
#[doc = "PCONR8 (rw) register accessor: desc PCONR8\n\nYou can [`read`](crate::Reg::read) this register and get [`pconr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconr8`] module"]
#[doc(alias = "PCONR8")]
pub type Pconr8 = crate::Reg<pconr8::Pconr8Spec>;
#[doc = "desc PCONR8"]
pub mod pconr8;
