#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    icg0: Icg0,
    icg1: Icg1,
    icg2: Icg2,
    icg3: Icg3,
    icg4: Icg4,
    icg5: Icg5,
    icg6: Icg6,
    icg7: Icg7,
}
impl RegisterBlock {
    #[doc = "0x00 - desc ICG0"]
    #[inline(always)]
    pub const fn icg0(&self) -> &Icg0 {
        &self.icg0
    }
    #[doc = "0x04 - desc ICG1"]
    #[inline(always)]
    pub const fn icg1(&self) -> &Icg1 {
        &self.icg1
    }
    #[doc = "0x08 - desc ICG2"]
    #[inline(always)]
    pub const fn icg2(&self) -> &Icg2 {
        &self.icg2
    }
    #[doc = "0x0c - desc ICG3"]
    #[inline(always)]
    pub const fn icg3(&self) -> &Icg3 {
        &self.icg3
    }
    #[doc = "0x10 - desc ICG4"]
    #[inline(always)]
    pub const fn icg4(&self) -> &Icg4 {
        &self.icg4
    }
    #[doc = "0x14 - desc ICG5"]
    #[inline(always)]
    pub const fn icg5(&self) -> &Icg5 {
        &self.icg5
    }
    #[doc = "0x18 - desc ICG6"]
    #[inline(always)]
    pub const fn icg6(&self) -> &Icg6 {
        &self.icg6
    }
    #[doc = "0x1c - desc ICG7"]
    #[inline(always)]
    pub const fn icg7(&self) -> &Icg7 {
        &self.icg7
    }
}
#[doc = "ICG0 (r) register accessor: desc ICG0\n\nYou can [`read`](crate::Reg::read) this register and get [`icg0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icg0`] module"]
#[doc(alias = "ICG0")]
pub type Icg0 = crate::Reg<icg0::Icg0Spec>;
#[doc = "desc ICG0"]
pub mod icg0;
#[doc = "ICG1 (r) register accessor: desc ICG1\n\nYou can [`read`](crate::Reg::read) this register and get [`icg1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icg1`] module"]
#[doc(alias = "ICG1")]
pub type Icg1 = crate::Reg<icg1::Icg1Spec>;
#[doc = "desc ICG1"]
pub mod icg1;
#[doc = "ICG2 (r) register accessor: desc ICG2\n\nYou can [`read`](crate::Reg::read) this register and get [`icg2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icg2`] module"]
#[doc(alias = "ICG2")]
pub type Icg2 = crate::Reg<icg2::Icg2Spec>;
#[doc = "desc ICG2"]
pub mod icg2;
#[doc = "ICG3 (r) register accessor: desc ICG3\n\nYou can [`read`](crate::Reg::read) this register and get [`icg3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icg3`] module"]
#[doc(alias = "ICG3")]
pub type Icg3 = crate::Reg<icg3::Icg3Spec>;
#[doc = "desc ICG3"]
pub mod icg3;
#[doc = "ICG4 (r) register accessor: desc ICG4\n\nYou can [`read`](crate::Reg::read) this register and get [`icg4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icg4`] module"]
#[doc(alias = "ICG4")]
pub type Icg4 = crate::Reg<icg4::Icg4Spec>;
#[doc = "desc ICG4"]
pub mod icg4;
#[doc = "ICG5 (r) register accessor: desc ICG5\n\nYou can [`read`](crate::Reg::read) this register and get [`icg5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icg5`] module"]
#[doc(alias = "ICG5")]
pub type Icg5 = crate::Reg<icg5::Icg5Spec>;
#[doc = "desc ICG5"]
pub mod icg5;
#[doc = "ICG6 (r) register accessor: desc ICG6\n\nYou can [`read`](crate::Reg::read) this register and get [`icg6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icg6`] module"]
#[doc(alias = "ICG6")]
pub type Icg6 = crate::Reg<icg6::Icg6Spec>;
#[doc = "desc ICG6"]
pub mod icg6;
#[doc = "ICG7 (r) register accessor: desc ICG7\n\nYou can [`read`](crate::Reg::read) this register and get [`icg7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icg7`] module"]
#[doc(alias = "ICG7")]
pub type Icg7 = crate::Reg<icg7::Icg7Spec>;
#[doc = "desc ICG7"]
pub mod icg7;
