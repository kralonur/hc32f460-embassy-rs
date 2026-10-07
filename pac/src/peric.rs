#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    usbfs_syctlreg: UsbfsSyctlreg,
    sdioc_syctlreg: SdiocSyctlreg,
}
impl RegisterBlock {
    #[doc = "0x00 - desc USBFS_SYCTLREG"]
    #[inline(always)]
    pub const fn usbfs_syctlreg(&self) -> &UsbfsSyctlreg {
        &self.usbfs_syctlreg
    }
    #[doc = "0x04 - desc SDIOC_SYCTLREG"]
    #[inline(always)]
    pub const fn sdioc_syctlreg(&self) -> &SdiocSyctlreg {
        &self.sdioc_syctlreg
    }
}
#[doc = "USBFS_SYCTLREG (rw) register accessor: desc USBFS_SYCTLREG\n\nYou can [`read`](crate::Reg::read) this register and get [`usbfs_syctlreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbfs_syctlreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbfs_syctlreg`] module"]
#[doc(alias = "USBFS_SYCTLREG")]
pub type UsbfsSyctlreg = crate::Reg<usbfs_syctlreg::UsbfsSyctlregSpec>;
#[doc = "desc USBFS_SYCTLREG"]
pub mod usbfs_syctlreg;
#[doc = "SDIOC_SYCTLREG (rw) register accessor: desc SDIOC_SYCTLREG\n\nYou can [`read`](crate::Reg::read) this register and get [`sdioc_syctlreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdioc_syctlreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdioc_syctlreg`] module"]
#[doc(alias = "SDIOC_SYCTLREG")]
pub type SdiocSyctlreg = crate::Reg<sdioc_syctlreg::SdiocSyctlregSpec>;
#[doc = "desc SDIOC_SYCTLREG"]
pub mod sdioc_syctlreg;
