#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ecer1: Ecer1,
    ecer2: Ecer2,
    ecer3: Ecer3,
}
impl RegisterBlock {
    #[doc = "0x00 - desc ECER1"]
    #[inline(always)]
    pub const fn ecer1(&self) -> &Ecer1 {
        &self.ecer1
    }
    #[doc = "0x04 - desc ECER2"]
    #[inline(always)]
    pub const fn ecer2(&self) -> &Ecer2 {
        &self.ecer2
    }
    #[doc = "0x08 - desc ECER3"]
    #[inline(always)]
    pub const fn ecer3(&self) -> &Ecer3 {
        &self.ecer3
    }
}
#[doc = "ECER1 (rw) register accessor: desc ECER1\n\nYou can [`read`](crate::Reg::read) this register and get [`ecer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecer1`] module"]
#[doc(alias = "ECER1")]
pub type Ecer1 = crate::Reg<ecer1::Ecer1Spec>;
#[doc = "desc ECER1"]
pub mod ecer1;
#[doc = "ECER2 (rw) register accessor: desc ECER2\n\nYou can [`read`](crate::Reg::read) this register and get [`ecer2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecer2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecer2`] module"]
#[doc(alias = "ECER2")]
pub type Ecer2 = crate::Reg<ecer2::Ecer2Spec>;
#[doc = "desc ECER2"]
pub mod ecer2;
#[doc = "ECER3 (rw) register accessor: desc ECER3\n\nYou can [`read`](crate::Reg::read) this register and get [`ecer3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecer3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecer3`] module"]
#[doc(alias = "ECER3")]
pub type Ecer3 = crate::Reg<ecer3::Ecer3Spec>;
#[doc = "desc ECER3"]
pub mod ecer3;
