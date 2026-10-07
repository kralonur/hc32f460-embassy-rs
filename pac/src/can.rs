#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rbuf: Rbuf,
    _reserved1: [u8; 0x4c],
    tbuf: Tbuf,
    _reserved2: [u8; 0x4c],
    cfg_stat: CfgStat,
    tcmd: Tcmd,
    tctrl: Tctrl,
    rctrl: Rctrl,
    rtie: Rtie,
    rtif: Rtif,
    errint: Errint,
    limit: Limit,
    sbt: Sbt,
    _reserved11: [u8; 0x04],
    ealcap: Ealcap,
    _reserved12: [u8; 0x01],
    recnt: Recnt,
    tecnt: Tecnt,
    acfctrl: Acfctrl,
    _reserved15: [u8; 0x01],
    acfen: Acfen,
    _reserved16: [u8; 0x01],
    acf: Acf,
    _reserved17: [u8; 0x02],
    tbslot: Tbslot,
    ttcfg: Ttcfg,
    ref_msg: RefMsg,
    trg_cfg: TrgCfg,
    tt_trig: TtTrig,
    tt_wtrig: TtWtrig,
}
impl RegisterBlock {
    #[doc = "0x00 - desc RBUF"]
    #[inline(always)]
    pub const fn rbuf(&self) -> &Rbuf {
        &self.rbuf
    }
    #[doc = "0x50 - desc TBUF"]
    #[inline(always)]
    pub const fn tbuf(&self) -> &Tbuf {
        &self.tbuf
    }
    #[doc = "0xa0 - desc CFG_STAT"]
    #[inline(always)]
    pub const fn cfg_stat(&self) -> &CfgStat {
        &self.cfg_stat
    }
    #[doc = "0xa1 - desc TCMD"]
    #[inline(always)]
    pub const fn tcmd(&self) -> &Tcmd {
        &self.tcmd
    }
    #[doc = "0xa2 - desc TCTRL"]
    #[inline(always)]
    pub const fn tctrl(&self) -> &Tctrl {
        &self.tctrl
    }
    #[doc = "0xa3 - desc RCTRL"]
    #[inline(always)]
    pub const fn rctrl(&self) -> &Rctrl {
        &self.rctrl
    }
    #[doc = "0xa4 - desc RTIE"]
    #[inline(always)]
    pub const fn rtie(&self) -> &Rtie {
        &self.rtie
    }
    #[doc = "0xa5 - desc RTIF"]
    #[inline(always)]
    pub const fn rtif(&self) -> &Rtif {
        &self.rtif
    }
    #[doc = "0xa6 - desc ERRINT"]
    #[inline(always)]
    pub const fn errint(&self) -> &Errint {
        &self.errint
    }
    #[doc = "0xa7 - desc LIMIT"]
    #[inline(always)]
    pub const fn limit(&self) -> &Limit {
        &self.limit
    }
    #[doc = "0xa8 - desc SBT"]
    #[inline(always)]
    pub const fn sbt(&self) -> &Sbt {
        &self.sbt
    }
    #[doc = "0xb0 - desc EALCAP"]
    #[inline(always)]
    pub const fn ealcap(&self) -> &Ealcap {
        &self.ealcap
    }
    #[doc = "0xb2 - desc RECNT"]
    #[inline(always)]
    pub const fn recnt(&self) -> &Recnt {
        &self.recnt
    }
    #[doc = "0xb3 - desc TECNT"]
    #[inline(always)]
    pub const fn tecnt(&self) -> &Tecnt {
        &self.tecnt
    }
    #[doc = "0xb4 - desc ACFCTRL"]
    #[inline(always)]
    pub const fn acfctrl(&self) -> &Acfctrl {
        &self.acfctrl
    }
    #[doc = "0xb6 - desc ACFEN"]
    #[inline(always)]
    pub const fn acfen(&self) -> &Acfen {
        &self.acfen
    }
    #[doc = "0xb8 - desc ACF"]
    #[inline(always)]
    pub const fn acf(&self) -> &Acf {
        &self.acf
    }
    #[doc = "0xbe - desc TBSLOT"]
    #[inline(always)]
    pub const fn tbslot(&self) -> &Tbslot {
        &self.tbslot
    }
    #[doc = "0xbf - desc TTCFG"]
    #[inline(always)]
    pub const fn ttcfg(&self) -> &Ttcfg {
        &self.ttcfg
    }
    #[doc = "0xc0 - desc REF_MSG"]
    #[inline(always)]
    pub const fn ref_msg(&self) -> &RefMsg {
        &self.ref_msg
    }
    #[doc = "0xc4 - desc TRG_CFG"]
    #[inline(always)]
    pub const fn trg_cfg(&self) -> &TrgCfg {
        &self.trg_cfg
    }
    #[doc = "0xc6 - desc TT_TRIG"]
    #[inline(always)]
    pub const fn tt_trig(&self) -> &TtTrig {
        &self.tt_trig
    }
    #[doc = "0xc8 - desc TT_WTRIG"]
    #[inline(always)]
    pub const fn tt_wtrig(&self) -> &TtWtrig {
        &self.tt_wtrig
    }
}
#[doc = "RBUF (r) register accessor: desc RBUF\n\nYou can [`read`](crate::Reg::read) this register and get [`rbuf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbuf`] module"]
#[doc(alias = "RBUF")]
pub type Rbuf = crate::Reg<rbuf::RbufSpec>;
#[doc = "desc RBUF"]
pub mod rbuf;
#[doc = "TBUF (rw) register accessor: desc TBUF\n\nYou can [`read`](crate::Reg::read) this register and get [`tbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbuf`] module"]
#[doc(alias = "TBUF")]
pub type Tbuf = crate::Reg<tbuf::TbufSpec>;
#[doc = "desc TBUF"]
pub mod tbuf;
#[doc = "CFG_STAT (rw) register accessor: desc CFG_STAT\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_stat`] module"]
#[doc(alias = "CFG_STAT")]
pub type CfgStat = crate::Reg<cfg_stat::CfgStatSpec>;
#[doc = "desc CFG_STAT"]
pub mod cfg_stat;
#[doc = "TCMD (rw) register accessor: desc TCMD\n\nYou can [`read`](crate::Reg::read) this register and get [`tcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcmd`] module"]
#[doc(alias = "TCMD")]
pub type Tcmd = crate::Reg<tcmd::TcmdSpec>;
#[doc = "desc TCMD"]
pub mod tcmd;
#[doc = "TCTRL (rw) register accessor: desc TCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`tctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tctrl`] module"]
#[doc(alias = "TCTRL")]
pub type Tctrl = crate::Reg<tctrl::TctrlSpec>;
#[doc = "desc TCTRL"]
pub mod tctrl;
#[doc = "RCTRL (rw) register accessor: desc RCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`rctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rctrl`] module"]
#[doc(alias = "RCTRL")]
pub type Rctrl = crate::Reg<rctrl::RctrlSpec>;
#[doc = "desc RCTRL"]
pub mod rctrl;
#[doc = "RTIE (rw) register accessor: desc RTIE\n\nYou can [`read`](crate::Reg::read) this register and get [`rtie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtie`] module"]
#[doc(alias = "RTIE")]
pub type Rtie = crate::Reg<rtie::RtieSpec>;
#[doc = "desc RTIE"]
pub mod rtie;
#[doc = "RTIF (rw) register accessor: desc RTIF\n\nYou can [`read`](crate::Reg::read) this register and get [`rtif::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtif::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtif`] module"]
#[doc(alias = "RTIF")]
pub type Rtif = crate::Reg<rtif::RtifSpec>;
#[doc = "desc RTIF"]
pub mod rtif;
#[doc = "ERRINT (rw) register accessor: desc ERRINT\n\nYou can [`read`](crate::Reg::read) this register and get [`errint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errint`] module"]
#[doc(alias = "ERRINT")]
pub type Errint = crate::Reg<errint::ErrintSpec>;
#[doc = "desc ERRINT"]
pub mod errint;
#[doc = "LIMIT (rw) register accessor: desc LIMIT\n\nYou can [`read`](crate::Reg::read) this register and get [`limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@limit`] module"]
#[doc(alias = "LIMIT")]
pub type Limit = crate::Reg<limit::LimitSpec>;
#[doc = "desc LIMIT"]
pub mod limit;
#[doc = "SBT (rw) register accessor: desc SBT\n\nYou can [`read`](crate::Reg::read) this register and get [`sbt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbt`] module"]
#[doc(alias = "SBT")]
pub type Sbt = crate::Reg<sbt::SbtSpec>;
#[doc = "desc SBT"]
pub mod sbt;
#[doc = "EALCAP (r) register accessor: desc EALCAP\n\nYou can [`read`](crate::Reg::read) this register and get [`ealcap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ealcap`] module"]
#[doc(alias = "EALCAP")]
pub type Ealcap = crate::Reg<ealcap::EalcapSpec>;
#[doc = "desc EALCAP"]
pub mod ealcap;
#[doc = "RECNT (rw) register accessor: desc RECNT\n\nYou can [`read`](crate::Reg::read) this register and get [`recnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`recnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@recnt`] module"]
#[doc(alias = "RECNT")]
pub type Recnt = crate::Reg<recnt::RecntSpec>;
#[doc = "desc RECNT"]
pub mod recnt;
#[doc = "TECNT (rw) register accessor: desc TECNT\n\nYou can [`read`](crate::Reg::read) this register and get [`tecnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tecnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tecnt`] module"]
#[doc(alias = "TECNT")]
pub type Tecnt = crate::Reg<tecnt::TecntSpec>;
#[doc = "desc TECNT"]
pub mod tecnt;
#[doc = "ACFCTRL (rw) register accessor: desc ACFCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`acfctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acfctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acfctrl`] module"]
#[doc(alias = "ACFCTRL")]
pub type Acfctrl = crate::Reg<acfctrl::AcfctrlSpec>;
#[doc = "desc ACFCTRL"]
pub mod acfctrl;
#[doc = "ACFEN (rw) register accessor: desc ACFEN\n\nYou can [`read`](crate::Reg::read) this register and get [`acfen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acfen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acfen`] module"]
#[doc(alias = "ACFEN")]
pub type Acfen = crate::Reg<acfen::AcfenSpec>;
#[doc = "desc ACFEN"]
pub mod acfen;
#[doc = "ACF (rw) register accessor: desc ACF\n\nYou can [`read`](crate::Reg::read) this register and get [`acf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acf`] module"]
#[doc(alias = "ACF")]
pub type Acf = crate::Reg<acf::AcfSpec>;
#[doc = "desc ACF"]
pub mod acf;
#[doc = "TBSLOT (rw) register accessor: desc TBSLOT\n\nYou can [`read`](crate::Reg::read) this register and get [`tbslot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbslot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbslot`] module"]
#[doc(alias = "TBSLOT")]
pub type Tbslot = crate::Reg<tbslot::TbslotSpec>;
#[doc = "desc TBSLOT"]
pub mod tbslot;
#[doc = "TTCFG (rw) register accessor: desc TTCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`ttcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttcfg`] module"]
#[doc(alias = "TTCFG")]
pub type Ttcfg = crate::Reg<ttcfg::TtcfgSpec>;
#[doc = "desc TTCFG"]
pub mod ttcfg;
#[doc = "REF_MSG (rw) register accessor: desc REF_MSG\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_msg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_msg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_msg`] module"]
#[doc(alias = "REF_MSG")]
pub type RefMsg = crate::Reg<ref_msg::RefMsgSpec>;
#[doc = "desc REF_MSG"]
pub mod ref_msg;
#[doc = "TRG_CFG (rw) register accessor: desc TRG_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`trg_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trg_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trg_cfg`] module"]
#[doc(alias = "TRG_CFG")]
pub type TrgCfg = crate::Reg<trg_cfg::TrgCfgSpec>;
#[doc = "desc TRG_CFG"]
pub mod trg_cfg;
#[doc = "TT_TRIG (rw) register accessor: desc TT_TRIG\n\nYou can [`read`](crate::Reg::read) this register and get [`tt_trig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tt_trig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tt_trig`] module"]
#[doc(alias = "TT_TRIG")]
pub type TtTrig = crate::Reg<tt_trig::TtTrigSpec>;
#[doc = "desc TT_TRIG"]
pub mod tt_trig;
#[doc = "TT_WTRIG (rw) register accessor: desc TT_WTRIG\n\nYou can [`read`](crate::Reg::read) this register and get [`tt_wtrig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tt_wtrig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tt_wtrig`] module"]
#[doc(alias = "TT_WTRIG")]
pub type TtWtrig = crate::Reg<tt_wtrig::TtWtrigSpec>;
#[doc = "desc TT_WTRIG"]
pub mod tt_wtrig;
