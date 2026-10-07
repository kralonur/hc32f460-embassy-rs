#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    sr: Sr,
    er: Er,
    cfgr: Cfgr,
    txbuf: Txbuf,
    rxbuf: Rxbuf,
    pr: Pr,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CTRL"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - desc SR"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x08 - desc ER"]
    #[inline(always)]
    pub const fn er(&self) -> &Er {
        &self.er
    }
    #[doc = "0x0c - desc CFGR"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x10 - desc TXBUF"]
    #[inline(always)]
    pub const fn txbuf(&self) -> &Txbuf {
        &self.txbuf
    }
    #[doc = "0x14 - desc RXBUF"]
    #[inline(always)]
    pub const fn rxbuf(&self) -> &Rxbuf {
        &self.rxbuf
    }
    #[doc = "0x18 - desc PR"]
    #[inline(always)]
    pub const fn pr(&self) -> &Pr {
        &self.pr
    }
}
#[doc = "CTRL (rw) register accessor: desc CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "desc CTRL"]
pub mod ctrl;
#[doc = "SR (r) register accessor: desc SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "desc SR"]
pub mod sr;
#[doc = "ER (rw) register accessor: desc ER\n\nYou can [`read`](crate::Reg::read) this register and get [`er::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`er::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@er`] module"]
#[doc(alias = "ER")]
pub type Er = crate::Reg<er::ErSpec>;
#[doc = "desc ER"]
pub mod er;
#[doc = "CFGR (rw) register accessor: desc CFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "desc CFGR"]
pub mod cfgr;
#[doc = "TXBUF (w) register accessor: desc TXBUF\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbuf::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbuf`] module"]
#[doc(alias = "TXBUF")]
pub type Txbuf = crate::Reg<txbuf::TxbufSpec>;
#[doc = "desc TXBUF"]
pub mod txbuf;
#[doc = "RXBUF (r) register accessor: desc RXBUF\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbuf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbuf`] module"]
#[doc(alias = "RXBUF")]
pub type Rxbuf = crate::Reg<rxbuf::RxbufSpec>;
#[doc = "desc RXBUF"]
pub mod rxbuf;
#[doc = "PR (rw) register accessor: desc PR\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`] module"]
#[doc(alias = "PR")]
pub type Pr = crate::Reg<pr::PrSpec>;
#[doc = "desc PR"]
pub mod pr;
