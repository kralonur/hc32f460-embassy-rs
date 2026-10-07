#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    sr: Sr,
    rr: Rr,
}
impl RegisterBlock {
    #[doc = "0x04 - desc SR"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x08 - desc RR"]
    #[inline(always)]
    pub const fn rr(&self) -> &Rr {
        &self.rr
    }
}
#[doc = "SR (rw) register accessor: desc SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "desc SR"]
pub mod sr;
#[doc = "RR (rw) register accessor: desc RR\n\nYou can [`read`](crate::Reg::read) this register and get [`rr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rr`] module"]
#[doc(alias = "RR")]
pub type Rr = crate::Reg<rr::RrSpec>;
#[doc = "desc RR"]
pub mod rr;
