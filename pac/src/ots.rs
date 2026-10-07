#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    dr1: Dr1,
    dr2: Dr2,
    ecr: Ecr,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CTL"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x02 - desc DR1"]
    #[inline(always)]
    pub const fn dr1(&self) -> &Dr1 {
        &self.dr1
    }
    #[doc = "0x04 - desc DR2"]
    #[inline(always)]
    pub const fn dr2(&self) -> &Dr2 {
        &self.dr2
    }
    #[doc = "0x06 - desc ECR"]
    #[inline(always)]
    pub const fn ecr(&self) -> &Ecr {
        &self.ecr
    }
}
#[doc = "CTL (rw) register accessor: desc CTL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`] module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "desc CTL"]
pub mod ctl;
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
#[doc = "ECR (rw) register accessor: desc ECR\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`] module"]
#[doc(alias = "ECR")]
pub type Ecr = crate::Reg<ecr::EcrSpec>;
#[doc = "desc ECR"]
pub mod ecr;
