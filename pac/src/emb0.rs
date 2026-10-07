#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    pwmlv: Pwmlv,
    soe: Soe,
    stat: Stat,
    statclr: Statclr,
    inten: Inten,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CTL"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - desc PWMLV"]
    #[inline(always)]
    pub const fn pwmlv(&self) -> &Pwmlv {
        &self.pwmlv
    }
    #[doc = "0x08 - desc SOE"]
    #[inline(always)]
    pub const fn soe(&self) -> &Soe {
        &self.soe
    }
    #[doc = "0x0c - desc STAT"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x10 - desc STATCLR"]
    #[inline(always)]
    pub const fn statclr(&self) -> &Statclr {
        &self.statclr
    }
    #[doc = "0x14 - desc INTEN"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
}
#[doc = "CTL (rw) register accessor: desc CTL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`] module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "desc CTL"]
pub mod ctl;
#[doc = "PWMLV (rw) register accessor: desc PWMLV\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmlv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmlv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmlv`] module"]
#[doc(alias = "PWMLV")]
pub type Pwmlv = crate::Reg<pwmlv::PwmlvSpec>;
#[doc = "desc PWMLV"]
pub mod pwmlv;
#[doc = "SOE (rw) register accessor: desc SOE\n\nYou can [`read`](crate::Reg::read) this register and get [`soe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soe`] module"]
#[doc(alias = "SOE")]
pub type Soe = crate::Reg<soe::SoeSpec>;
#[doc = "desc SOE"]
pub mod soe;
#[doc = "STAT (r) register accessor: desc STAT\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`] module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "desc STAT"]
pub mod stat;
#[doc = "STATCLR (w) register accessor: desc STATCLR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statclr`] module"]
#[doc(alias = "STATCLR")]
pub type Statclr = crate::Reg<statclr::StatclrSpec>;
#[doc = "desc STATCLR"]
pub mod statclr;
#[doc = "INTEN (rw) register accessor: desc INTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "desc INTEN"]
pub mod inten;
