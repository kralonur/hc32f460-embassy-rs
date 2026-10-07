#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    authid0: Authid0,
    authid1: Authid1,
    authid2: Authid2,
    _reserved3: [u8; 0x04],
    mcustat: Mcustat,
    _reserved4: [u8; 0x04],
    fersctl: Fersctl,
    mcudbgstat: Mcudbgstat,
    mcustpctl: Mcustpctl,
    mcutracectl: Mcutracectl,
}
impl RegisterBlock {
    #[doc = "0x00 - desc AUTHID0"]
    #[inline(always)]
    pub const fn authid0(&self) -> &Authid0 {
        &self.authid0
    }
    #[doc = "0x04 - desc AUTHID1"]
    #[inline(always)]
    pub const fn authid1(&self) -> &Authid1 {
        &self.authid1
    }
    #[doc = "0x08 - desc AUTHID2"]
    #[inline(always)]
    pub const fn authid2(&self) -> &Authid2 {
        &self.authid2
    }
    #[doc = "0x10 - desc MCUSTAT"]
    #[inline(always)]
    pub const fn mcustat(&self) -> &Mcustat {
        &self.mcustat
    }
    #[doc = "0x18 - desc FERSCTL"]
    #[inline(always)]
    pub const fn fersctl(&self) -> &Fersctl {
        &self.fersctl
    }
    #[doc = "0x1c - desc MCUDBGSTAT"]
    #[inline(always)]
    pub const fn mcudbgstat(&self) -> &Mcudbgstat {
        &self.mcudbgstat
    }
    #[doc = "0x20 - desc MCUSTPCTL"]
    #[inline(always)]
    pub const fn mcustpctl(&self) -> &Mcustpctl {
        &self.mcustpctl
    }
    #[doc = "0x24 - desc MCUTRACECTL"]
    #[inline(always)]
    pub const fn mcutracectl(&self) -> &Mcutracectl {
        &self.mcutracectl
    }
}
#[doc = "AUTHID0 (rw) register accessor: desc AUTHID0\n\nYou can [`read`](crate::Reg::read) this register and get [`authid0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`authid0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@authid0`] module"]
#[doc(alias = "AUTHID0")]
pub type Authid0 = crate::Reg<authid0::Authid0Spec>;
#[doc = "desc AUTHID0"]
pub mod authid0;
#[doc = "AUTHID1 (rw) register accessor: desc AUTHID1\n\nYou can [`read`](crate::Reg::read) this register and get [`authid1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`authid1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@authid1`] module"]
#[doc(alias = "AUTHID1")]
pub type Authid1 = crate::Reg<authid1::Authid1Spec>;
#[doc = "desc AUTHID1"]
pub mod authid1;
#[doc = "AUTHID2 (rw) register accessor: desc AUTHID2\n\nYou can [`read`](crate::Reg::read) this register and get [`authid2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`authid2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@authid2`] module"]
#[doc(alias = "AUTHID2")]
pub type Authid2 = crate::Reg<authid2::Authid2Spec>;
#[doc = "desc AUTHID2"]
pub mod authid2;
#[doc = "MCUSTAT (rw) register accessor: desc MCUSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`mcustat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcustat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcustat`] module"]
#[doc(alias = "MCUSTAT")]
pub type Mcustat = crate::Reg<mcustat::McustatSpec>;
#[doc = "desc MCUSTAT"]
pub mod mcustat;
#[doc = "FERSCTL (rw) register accessor: desc FERSCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`fersctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fersctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fersctl`] module"]
#[doc(alias = "FERSCTL")]
pub type Fersctl = crate::Reg<fersctl::FersctlSpec>;
#[doc = "desc FERSCTL"]
pub mod fersctl;
#[doc = "MCUDBGSTAT (rw) register accessor: desc MCUDBGSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`mcudbgstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcudbgstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcudbgstat`] module"]
#[doc(alias = "MCUDBGSTAT")]
pub type Mcudbgstat = crate::Reg<mcudbgstat::McudbgstatSpec>;
#[doc = "desc MCUDBGSTAT"]
pub mod mcudbgstat;
#[doc = "MCUSTPCTL (rw) register accessor: desc MCUSTPCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`mcustpctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcustpctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcustpctl`] module"]
#[doc(alias = "MCUSTPCTL")]
pub type Mcustpctl = crate::Reg<mcustpctl::McustpctlSpec>;
#[doc = "desc MCUSTPCTL"]
pub mod mcustpctl;
#[doc = "MCUTRACECTL (rw) register accessor: desc MCUTRACECTL\n\nYou can [`read`](crate::Reg::read) this register and get [`mcutracectl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcutracectl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcutracectl`] module"]
#[doc(alias = "MCUTRACECTL")]
pub type Mcutracectl = crate::Reg<mcutracectl::McutracectlSpec>;
#[doc = "desc MCUTRACECTL"]
pub mod mcutracectl;
