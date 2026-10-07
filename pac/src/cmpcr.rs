#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    dadr1: Dadr1,
    dadr2: Dadr2,
    _reserved2: [u8; 0x04],
    dacr: Dacr,
    _reserved3: [u8; 0x02],
    rvadc: Rvadc,
}
impl RegisterBlock {
    #[doc = "0x100 - desc DADR1"]
    #[inline(always)]
    pub const fn dadr1(&self) -> &Dadr1 {
        &self.dadr1
    }
    #[doc = "0x102 - desc DADR2"]
    #[inline(always)]
    pub const fn dadr2(&self) -> &Dadr2 {
        &self.dadr2
    }
    #[doc = "0x108 - desc DACR"]
    #[inline(always)]
    pub const fn dacr(&self) -> &Dacr {
        &self.dacr
    }
    #[doc = "0x10c - desc RVADC"]
    #[inline(always)]
    pub const fn rvadc(&self) -> &Rvadc {
        &self.rvadc
    }
}
#[doc = "DADR1 (rw) register accessor: desc DADR1\n\nYou can [`read`](crate::Reg::read) this register and get [`dadr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dadr1`] module"]
#[doc(alias = "DADR1")]
pub type Dadr1 = crate::Reg<dadr1::Dadr1Spec>;
#[doc = "desc DADR1"]
pub mod dadr1;
#[doc = "DADR2 (rw) register accessor: desc DADR2\n\nYou can [`read`](crate::Reg::read) this register and get [`dadr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dadr2`] module"]
#[doc(alias = "DADR2")]
pub type Dadr2 = crate::Reg<dadr2::Dadr2Spec>;
#[doc = "desc DADR2"]
pub mod dadr2;
#[doc = "DACR (rw) register accessor: desc DACR\n\nYou can [`read`](crate::Reg::read) this register and get [`dacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacr`] module"]
#[doc(alias = "DACR")]
pub type Dacr = crate::Reg<dacr::DacrSpec>;
#[doc = "desc DACR"]
pub mod dacr;
#[doc = "RVADC (rw) register accessor: desc RVADC\n\nYou can [`read`](crate::Reg::read) this register and get [`rvadc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rvadc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rvadc`] module"]
#[doc(alias = "RVADC")]
pub type Rvadc = crate::Reg<rvadc::RvadcSpec>;
#[doc = "desc RVADC"]
pub mod rvadc;
