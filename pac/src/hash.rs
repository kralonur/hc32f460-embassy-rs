#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    _reserved1: [u8; 0x0c],
    hr7: Hr7,
    hr6: Hr6,
    hr5: Hr5,
    hr4: Hr4,
    hr3: Hr3,
    hr2: Hr2,
    hr1: Hr1,
    hr0: Hr0,
    _reserved9: [u8; 0x10],
    dr15: Dr15,
    dr14: Dr14,
    dr13: Dr13,
    dr12: Dr12,
    dr11: Dr11,
    dr10: Dr10,
    dr9: Dr9,
    dr8: Dr8,
    dr7: Dr7,
    dr6: Dr6,
    dr5: Dr5,
    dr4: Dr4,
    dr3: Dr3,
    dr2: Dr2,
    dr1: Dr1,
    dr0: Dr0,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x10 - desc HR7"]
    #[inline(always)]
    pub const fn hr7(&self) -> &Hr7 {
        &self.hr7
    }
    #[doc = "0x14 - desc HR6"]
    #[inline(always)]
    pub const fn hr6(&self) -> &Hr6 {
        &self.hr6
    }
    #[doc = "0x18 - desc HR5"]
    #[inline(always)]
    pub const fn hr5(&self) -> &Hr5 {
        &self.hr5
    }
    #[doc = "0x1c - desc HR4"]
    #[inline(always)]
    pub const fn hr4(&self) -> &Hr4 {
        &self.hr4
    }
    #[doc = "0x20 - desc HR3"]
    #[inline(always)]
    pub const fn hr3(&self) -> &Hr3 {
        &self.hr3
    }
    #[doc = "0x24 - desc HR2"]
    #[inline(always)]
    pub const fn hr2(&self) -> &Hr2 {
        &self.hr2
    }
    #[doc = "0x28 - desc HR1"]
    #[inline(always)]
    pub const fn hr1(&self) -> &Hr1 {
        &self.hr1
    }
    #[doc = "0x2c - desc HR0"]
    #[inline(always)]
    pub const fn hr0(&self) -> &Hr0 {
        &self.hr0
    }
    #[doc = "0x40 - desc DR15"]
    #[inline(always)]
    pub const fn dr15(&self) -> &Dr15 {
        &self.dr15
    }
    #[doc = "0x44 - desc DR14"]
    #[inline(always)]
    pub const fn dr14(&self) -> &Dr14 {
        &self.dr14
    }
    #[doc = "0x48 - desc DR13"]
    #[inline(always)]
    pub const fn dr13(&self) -> &Dr13 {
        &self.dr13
    }
    #[doc = "0x4c - desc DR12"]
    #[inline(always)]
    pub const fn dr12(&self) -> &Dr12 {
        &self.dr12
    }
    #[doc = "0x50 - desc DR11"]
    #[inline(always)]
    pub const fn dr11(&self) -> &Dr11 {
        &self.dr11
    }
    #[doc = "0x54 - desc DR10"]
    #[inline(always)]
    pub const fn dr10(&self) -> &Dr10 {
        &self.dr10
    }
    #[doc = "0x58 - desc DR9"]
    #[inline(always)]
    pub const fn dr9(&self) -> &Dr9 {
        &self.dr9
    }
    #[doc = "0x5c - desc DR8"]
    #[inline(always)]
    pub const fn dr8(&self) -> &Dr8 {
        &self.dr8
    }
    #[doc = "0x60 - desc DR7"]
    #[inline(always)]
    pub const fn dr7(&self) -> &Dr7 {
        &self.dr7
    }
    #[doc = "0x64 - desc DR6"]
    #[inline(always)]
    pub const fn dr6(&self) -> &Dr6 {
        &self.dr6
    }
    #[doc = "0x68 - desc DR5"]
    #[inline(always)]
    pub const fn dr5(&self) -> &Dr5 {
        &self.dr5
    }
    #[doc = "0x6c - desc DR4"]
    #[inline(always)]
    pub const fn dr4(&self) -> &Dr4 {
        &self.dr4
    }
    #[doc = "0x70 - desc DR3"]
    #[inline(always)]
    pub const fn dr3(&self) -> &Dr3 {
        &self.dr3
    }
    #[doc = "0x74 - desc DR2"]
    #[inline(always)]
    pub const fn dr2(&self) -> &Dr2 {
        &self.dr2
    }
    #[doc = "0x78 - desc DR1"]
    #[inline(always)]
    pub const fn dr1(&self) -> &Dr1 {
        &self.dr1
    }
    #[doc = "0x7c - desc DR0"]
    #[inline(always)]
    pub const fn dr0(&self) -> &Dr0 {
        &self.dr0
    }
}
#[doc = "CR (rw) register accessor: desc CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "HR7 (rw) register accessor: desc HR7\n\nYou can [`read`](crate::Reg::read) this register and get [`hr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hr7`] module"]
#[doc(alias = "HR7")]
pub type Hr7 = crate::Reg<hr7::Hr7Spec>;
#[doc = "desc HR7"]
pub mod hr7;
#[doc = "HR6 (rw) register accessor: desc HR6\n\nYou can [`read`](crate::Reg::read) this register and get [`hr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hr6`] module"]
#[doc(alias = "HR6")]
pub type Hr6 = crate::Reg<hr6::Hr6Spec>;
#[doc = "desc HR6"]
pub mod hr6;
#[doc = "HR5 (rw) register accessor: desc HR5\n\nYou can [`read`](crate::Reg::read) this register and get [`hr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hr5`] module"]
#[doc(alias = "HR5")]
pub type Hr5 = crate::Reg<hr5::Hr5Spec>;
#[doc = "desc HR5"]
pub mod hr5;
#[doc = "HR4 (rw) register accessor: desc HR4\n\nYou can [`read`](crate::Reg::read) this register and get [`hr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hr4`] module"]
#[doc(alias = "HR4")]
pub type Hr4 = crate::Reg<hr4::Hr4Spec>;
#[doc = "desc HR4"]
pub mod hr4;
#[doc = "HR3 (rw) register accessor: desc HR3\n\nYou can [`read`](crate::Reg::read) this register and get [`hr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hr3`] module"]
#[doc(alias = "HR3")]
pub type Hr3 = crate::Reg<hr3::Hr3Spec>;
#[doc = "desc HR3"]
pub mod hr3;
#[doc = "HR2 (rw) register accessor: desc HR2\n\nYou can [`read`](crate::Reg::read) this register and get [`hr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hr2`] module"]
#[doc(alias = "HR2")]
pub type Hr2 = crate::Reg<hr2::Hr2Spec>;
#[doc = "desc HR2"]
pub mod hr2;
#[doc = "HR1 (rw) register accessor: desc HR1\n\nYou can [`read`](crate::Reg::read) this register and get [`hr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hr1`] module"]
#[doc(alias = "HR1")]
pub type Hr1 = crate::Reg<hr1::Hr1Spec>;
#[doc = "desc HR1"]
pub mod hr1;
#[doc = "HR0 (rw) register accessor: desc HR0\n\nYou can [`read`](crate::Reg::read) this register and get [`hr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hr0`] module"]
#[doc(alias = "HR0")]
pub type Hr0 = crate::Reg<hr0::Hr0Spec>;
#[doc = "desc HR0"]
pub mod hr0;
#[doc = "DR15 (rw) register accessor: desc DR15\n\nYou can [`read`](crate::Reg::read) this register and get [`dr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr15`] module"]
#[doc(alias = "DR15")]
pub type Dr15 = crate::Reg<dr15::Dr15Spec>;
#[doc = "desc DR15"]
pub mod dr15;
#[doc = "DR14 (rw) register accessor: desc DR14\n\nYou can [`read`](crate::Reg::read) this register and get [`dr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr14`] module"]
#[doc(alias = "DR14")]
pub type Dr14 = crate::Reg<dr14::Dr14Spec>;
#[doc = "desc DR14"]
pub mod dr14;
#[doc = "DR13 (rw) register accessor: desc DR13\n\nYou can [`read`](crate::Reg::read) this register and get [`dr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr13`] module"]
#[doc(alias = "DR13")]
pub type Dr13 = crate::Reg<dr13::Dr13Spec>;
#[doc = "desc DR13"]
pub mod dr13;
#[doc = "DR12 (rw) register accessor: desc DR12\n\nYou can [`read`](crate::Reg::read) this register and get [`dr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr12`] module"]
#[doc(alias = "DR12")]
pub type Dr12 = crate::Reg<dr12::Dr12Spec>;
#[doc = "desc DR12"]
pub mod dr12;
#[doc = "DR11 (rw) register accessor: desc DR11\n\nYou can [`read`](crate::Reg::read) this register and get [`dr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr11`] module"]
#[doc(alias = "DR11")]
pub type Dr11 = crate::Reg<dr11::Dr11Spec>;
#[doc = "desc DR11"]
pub mod dr11;
#[doc = "DR10 (rw) register accessor: desc DR10\n\nYou can [`read`](crate::Reg::read) this register and get [`dr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr10`] module"]
#[doc(alias = "DR10")]
pub type Dr10 = crate::Reg<dr10::Dr10Spec>;
#[doc = "desc DR10"]
pub mod dr10;
#[doc = "DR9 (rw) register accessor: desc DR9\n\nYou can [`read`](crate::Reg::read) this register and get [`dr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr9`] module"]
#[doc(alias = "DR9")]
pub type Dr9 = crate::Reg<dr9::Dr9Spec>;
#[doc = "desc DR9"]
pub mod dr9;
#[doc = "DR8 (rw) register accessor: desc DR8\n\nYou can [`read`](crate::Reg::read) this register and get [`dr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr8`] module"]
#[doc(alias = "DR8")]
pub type Dr8 = crate::Reg<dr8::Dr8Spec>;
#[doc = "desc DR8"]
pub mod dr8;
#[doc = "DR7 (rw) register accessor: desc DR7\n\nYou can [`read`](crate::Reg::read) this register and get [`dr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr7`] module"]
#[doc(alias = "DR7")]
pub type Dr7 = crate::Reg<dr7::Dr7Spec>;
#[doc = "desc DR7"]
pub mod dr7;
#[doc = "DR6 (rw) register accessor: desc DR6\n\nYou can [`read`](crate::Reg::read) this register and get [`dr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr6`] module"]
#[doc(alias = "DR6")]
pub type Dr6 = crate::Reg<dr6::Dr6Spec>;
#[doc = "desc DR6"]
pub mod dr6;
#[doc = "DR5 (rw) register accessor: desc DR5\n\nYou can [`read`](crate::Reg::read) this register and get [`dr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr5`] module"]
#[doc(alias = "DR5")]
pub type Dr5 = crate::Reg<dr5::Dr5Spec>;
#[doc = "desc DR5"]
pub mod dr5;
#[doc = "DR4 (rw) register accessor: desc DR4\n\nYou can [`read`](crate::Reg::read) this register and get [`dr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr4`] module"]
#[doc(alias = "DR4")]
pub type Dr4 = crate::Reg<dr4::Dr4Spec>;
#[doc = "desc DR4"]
pub mod dr4;
#[doc = "DR3 (rw) register accessor: desc DR3\n\nYou can [`read`](crate::Reg::read) this register and get [`dr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr3`] module"]
#[doc(alias = "DR3")]
pub type Dr3 = crate::Reg<dr3::Dr3Spec>;
#[doc = "desc DR3"]
pub mod dr3;
#[doc = "DR2 (rw) register accessor: desc DR2\n\nYou can [`read`](crate::Reg::read) this register and get [`dr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr2`] module"]
#[doc(alias = "DR2")]
pub type Dr2 = crate::Reg<dr2::Dr2Spec>;
#[doc = "desc DR2"]
pub mod dr2;
#[doc = "DR1 (rw) register accessor: desc DR1\n\nYou can [`read`](crate::Reg::read) this register and get [`dr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr1`] module"]
#[doc(alias = "DR1")]
pub type Dr1 = crate::Reg<dr1::Dr1Spec>;
#[doc = "desc DR1"]
pub mod dr1;
#[doc = "DR0 (rw) register accessor: desc DR0\n\nYou can [`read`](crate::Reg::read) this register and get [`dr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr0`] module"]
#[doc(alias = "DR0")]
pub type Dr0 = crate::Reg<dr0::Dr0Spec>;
#[doc = "desc DR0"]
pub mod dr0;
