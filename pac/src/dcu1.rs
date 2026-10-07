#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    flag: Flag,
    data0: Data0,
    data1: Data1,
    data2: Data2,
    flagclr: Flagclr,
    intevtsel: Intevtsel,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CTL"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - desc FLAG"]
    #[inline(always)]
    pub const fn flag(&self) -> &Flag {
        &self.flag
    }
    #[doc = "0x08 - desc DATA0"]
    #[inline(always)]
    pub const fn data0(&self) -> &Data0 {
        &self.data0
    }
    #[doc = "0x0c - desc DATA1"]
    #[inline(always)]
    pub const fn data1(&self) -> &Data1 {
        &self.data1
    }
    #[doc = "0x10 - desc DATA2"]
    #[inline(always)]
    pub const fn data2(&self) -> &Data2 {
        &self.data2
    }
    #[doc = "0x14 - desc FLAGCLR"]
    #[inline(always)]
    pub const fn flagclr(&self) -> &Flagclr {
        &self.flagclr
    }
    #[doc = "0x18 - desc INTEVTSEL"]
    #[inline(always)]
    pub const fn intevtsel(&self) -> &Intevtsel {
        &self.intevtsel
    }
}
#[doc = "CTL (rw) register accessor: desc CTL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`] module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "desc CTL"]
pub mod ctl;
#[doc = "FLAG (r) register accessor: desc FLAG\n\nYou can [`read`](crate::Reg::read) this register and get [`flag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flag`] module"]
#[doc(alias = "FLAG")]
pub type Flag = crate::Reg<flag::FlagSpec>;
#[doc = "desc FLAG"]
pub mod flag;
#[doc = "DATA0 (rw) register accessor: desc DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0`] module"]
#[doc(alias = "DATA0")]
pub type Data0 = crate::Reg<data0::Data0Spec>;
#[doc = "desc DATA0"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: desc DATA1\n\nYou can [`read`](crate::Reg::read) this register and get [`data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1`] module"]
#[doc(alias = "DATA1")]
pub type Data1 = crate::Reg<data1::Data1Spec>;
#[doc = "desc DATA1"]
pub mod data1;
#[doc = "DATA2 (rw) register accessor: desc DATA2\n\nYou can [`read`](crate::Reg::read) this register and get [`data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2`] module"]
#[doc(alias = "DATA2")]
pub type Data2 = crate::Reg<data2::Data2Spec>;
#[doc = "desc DATA2"]
pub mod data2;
#[doc = "FLAGCLR (w) register accessor: desc FLAGCLR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flagclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flagclr`] module"]
#[doc(alias = "FLAGCLR")]
pub type Flagclr = crate::Reg<flagclr::FlagclrSpec>;
#[doc = "desc FLAGCLR"]
pub mod flagclr;
#[doc = "INTEVTSEL (rw) register accessor: desc INTEVTSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`intevtsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intevtsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intevtsel`] module"]
#[doc(alias = "INTEVTSEL")]
pub type Intevtsel = crate::Reg<intevtsel::IntevtselSpec>;
#[doc = "desc INTEVTSEL"]
pub mod intevtsel;
