#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    _reserved1: [u8; 0x0c],
    dr0: Dr0,
    dr1: Dr1,
    dr2: Dr2,
    dr3: Dr3,
    kr0: Kr0,
    kr1: Kr1,
    kr2: Kr2,
    kr3: Kr3,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x10 - desc DR0"]
    #[inline(always)]
    pub const fn dr0(&self) -> &Dr0 {
        &self.dr0
    }
    #[doc = "0x14 - desc DR1"]
    #[inline(always)]
    pub const fn dr1(&self) -> &Dr1 {
        &self.dr1
    }
    #[doc = "0x18 - desc DR2"]
    #[inline(always)]
    pub const fn dr2(&self) -> &Dr2 {
        &self.dr2
    }
    #[doc = "0x1c - desc DR3"]
    #[inline(always)]
    pub const fn dr3(&self) -> &Dr3 {
        &self.dr3
    }
    #[doc = "0x20 - desc KR0"]
    #[inline(always)]
    pub const fn kr0(&self) -> &Kr0 {
        &self.kr0
    }
    #[doc = "0x24 - desc KR1"]
    #[inline(always)]
    pub const fn kr1(&self) -> &Kr1 {
        &self.kr1
    }
    #[doc = "0x28 - desc KR2"]
    #[inline(always)]
    pub const fn kr2(&self) -> &Kr2 {
        &self.kr2
    }
    #[doc = "0x2c - desc KR3"]
    #[inline(always)]
    pub const fn kr3(&self) -> &Kr3 {
        &self.kr3
    }
}
#[doc = "CR (rw) register accessor: desc CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "DR0 (rw) register accessor: desc DR0\n\nYou can [`read`](crate::Reg::read) this register and get [`dr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr0`] module"]
#[doc(alias = "DR0")]
pub type Dr0 = crate::Reg<dr0::Dr0Spec>;
#[doc = "desc DR0"]
pub mod dr0;
#[doc = "DR1 (rw) register accessor: desc DR1\n\nYou can [`read`](crate::Reg::read) this register and get [`dr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr1`] module"]
#[doc(alias = "DR1")]
pub type Dr1 = crate::Reg<dr1::Dr1Spec>;
#[doc = "desc DR1"]
pub mod dr1;
#[doc = "DR2 (rw) register accessor: desc DR2\n\nYou can [`read`](crate::Reg::read) this register and get [`dr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr2`] module"]
#[doc(alias = "DR2")]
pub type Dr2 = crate::Reg<dr2::Dr2Spec>;
#[doc = "desc DR2"]
pub mod dr2;
#[doc = "DR3 (rw) register accessor: desc DR3\n\nYou can [`read`](crate::Reg::read) this register and get [`dr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr3`] module"]
#[doc(alias = "DR3")]
pub type Dr3 = crate::Reg<dr3::Dr3Spec>;
#[doc = "desc DR3"]
pub mod dr3;
#[doc = "KR0 (rw) register accessor: desc KR0\n\nYou can [`read`](crate::Reg::read) this register and get [`kr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kr0`] module"]
#[doc(alias = "KR0")]
pub type Kr0 = crate::Reg<kr0::Kr0Spec>;
#[doc = "desc KR0"]
pub mod kr0;
#[doc = "KR1 (rw) register accessor: desc KR1\n\nYou can [`read`](crate::Reg::read) this register and get [`kr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kr1`] module"]
#[doc(alias = "KR1")]
pub type Kr1 = crate::Reg<kr1::Kr1Spec>;
#[doc = "desc KR1"]
pub mod kr1;
#[doc = "KR2 (rw) register accessor: desc KR2\n\nYou can [`read`](crate::Reg::read) this register and get [`kr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kr2`] module"]
#[doc(alias = "KR2")]
pub type Kr2 = crate::Reg<kr2::Kr2Spec>;
#[doc = "desc KR2"]
pub mod kr2;
#[doc = "KR3 (rw) register accessor: desc KR3\n\nYou can [`read`](crate::Reg::read) this register and get [`kr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kr3`] module"]
#[doc(alias = "KR3")]
pub type Kr3 = crate::Reg<kr3::Kr3Spec>;
#[doc = "desc KR3"]
pub mod kr3;
