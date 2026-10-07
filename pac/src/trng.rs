#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    mr: Mr,
    _reserved2: [u8; 0x04],
    dr0: Dr0,
    dr1: Dr1,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - desc MR"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x0c - desc DR0"]
    #[inline(always)]
    pub const fn dr0(&self) -> &Dr0 {
        &self.dr0
    }
    #[doc = "0x10 - desc DR1"]
    #[inline(always)]
    pub const fn dr1(&self) -> &Dr1 {
        &self.dr1
    }
}
#[doc = "CR (rw) register accessor: desc CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "MR (rw) register accessor: desc MR\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`] module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "desc MR"]
pub mod mr;
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
