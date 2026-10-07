#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rstf0: Rstf0,
}
impl RegisterBlock {
    #[doc = "0x00 - desc RSTF0"]
    #[inline(always)]
    pub const fn rstf0(&self) -> &Rstf0 {
        &self.rstf0
    }
}
#[doc = "RSTF0 (rw) register accessor: desc RSTF0\n\nYou can [`read`](crate::Reg::read) this register and get [`rstf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstf0`] module"]
#[doc(alias = "RSTF0")]
pub type Rstf0 = crate::Reg<rstf0::Rstf0Spec>;
#[doc = "desc RSTF0"]
pub mod rstf0;
