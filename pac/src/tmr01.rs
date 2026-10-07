#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cntar: Cntar,
    cntbr: Cntbr,
    cmpar: Cmpar,
    cmpbr: Cmpbr,
    bconr: Bconr,
    stflr: Stflr,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CNTAR"]
    #[inline(always)]
    pub const fn cntar(&self) -> &Cntar {
        &self.cntar
    }
    #[doc = "0x04 - desc CNTBR"]
    #[inline(always)]
    pub const fn cntbr(&self) -> &Cntbr {
        &self.cntbr
    }
    #[doc = "0x08 - desc CMPAR"]
    #[inline(always)]
    pub const fn cmpar(&self) -> &Cmpar {
        &self.cmpar
    }
    #[doc = "0x0c - desc CMPBR"]
    #[inline(always)]
    pub const fn cmpbr(&self) -> &Cmpbr {
        &self.cmpbr
    }
    #[doc = "0x10 - desc BCONR"]
    #[inline(always)]
    pub const fn bconr(&self) -> &Bconr {
        &self.bconr
    }
    #[doc = "0x14 - desc STFLR"]
    #[inline(always)]
    pub const fn stflr(&self) -> &Stflr {
        &self.stflr
    }
}
#[doc = "CNTAR (rw) register accessor: desc CNTAR\n\nYou can [`read`](crate::Reg::read) this register and get [`cntar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntar`] module"]
#[doc(alias = "CNTAR")]
pub type Cntar = crate::Reg<cntar::CntarSpec>;
#[doc = "desc CNTAR"]
pub mod cntar;
#[doc = "CNTBR (rw) register accessor: desc CNTBR\n\nYou can [`read`](crate::Reg::read) this register and get [`cntbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntbr`] module"]
#[doc(alias = "CNTBR")]
pub type Cntbr = crate::Reg<cntbr::CntbrSpec>;
#[doc = "desc CNTBR"]
pub mod cntbr;
#[doc = "CMPAR (rw) register accessor: desc CMPAR\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpar`] module"]
#[doc(alias = "CMPAR")]
pub type Cmpar = crate::Reg<cmpar::CmparSpec>;
#[doc = "desc CMPAR"]
pub mod cmpar;
#[doc = "CMPBR (rw) register accessor: desc CMPBR\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpbr`] module"]
#[doc(alias = "CMPBR")]
pub type Cmpbr = crate::Reg<cmpbr::CmpbrSpec>;
#[doc = "desc CMPBR"]
pub mod cmpbr;
#[doc = "BCONR (rw) register accessor: desc BCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`bconr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bconr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bconr`] module"]
#[doc(alias = "BCONR")]
pub type Bconr = crate::Reg<bconr::BconrSpec>;
#[doc = "desc BCONR"]
pub mod bconr;
#[doc = "STFLR (rw) register accessor: desc STFLR\n\nYou can [`read`](crate::Reg::read) this register and get [`stflr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stflr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stflr`] module"]
#[doc(alias = "STFLR")]
pub type Stflr = crate::Reg<stflr::StflrSpec>;
#[doc = "desc STFLR"]
pub mod stflr;
