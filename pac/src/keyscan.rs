#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    scr: Scr,
    ser: Ser,
    ssr: Ssr,
}
impl RegisterBlock {
    #[doc = "0x00 - desc SCR"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x04 - desc SER"]
    #[inline(always)]
    pub const fn ser(&self) -> &Ser {
        &self.ser
    }
    #[doc = "0x08 - desc SSR"]
    #[inline(always)]
    pub const fn ssr(&self) -> &Ssr {
        &self.ssr
    }
}
#[doc = "SCR (rw) register accessor: desc SCR\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "desc SCR"]
pub mod scr;
#[doc = "SER (rw) register accessor: desc SER\n\nYou can [`read`](crate::Reg::read) this register and get [`ser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ser`] module"]
#[doc(alias = "SER")]
pub type Ser = crate::Reg<ser::SerSpec>;
#[doc = "desc SER"]
pub mod ser;
#[doc = "SSR (rw) register accessor: desc SSR\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr`] module"]
#[doc(alias = "SSR")]
pub type Ssr = crate::Reg<ssr::SsrSpec>;
#[doc = "desc SSR"]
pub mod ssr;
