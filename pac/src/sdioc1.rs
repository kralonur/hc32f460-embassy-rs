#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    blksize: Blksize,
    blkcnt: Blkcnt,
    arg0: Arg0,
    arg1: Arg1,
    transmode: Transmode,
    cmd: Cmd,
    resp0: Resp0,
    resp1: Resp1,
    resp2: Resp2,
    resp3: Resp3,
    resp4: Resp4,
    resp5: Resp5,
    resp6: Resp6,
    resp7: Resp7,
    buf0: Buf0,
    buf1: Buf1,
    pstat: Pstat,
    hostcon: Hostcon,
    pwrcon: Pwrcon,
    blkgpcon: Blkgpcon,
    _reserved20: [u8; 0x01],
    clkcon: Clkcon,
    toutcon: Toutcon,
    sftrst: Sftrst,
    norintst: Norintst,
    errintst: Errintst,
    norintsten: Norintsten,
    errintsten: Errintsten,
    norintsgen: Norintsgen,
    errintsgen: Errintsgen,
    atcerrst: Atcerrst,
    _reserved30: [u8; 0x12],
    fea: Fea,
    fee: Fee,
}
impl RegisterBlock {
    #[doc = "0x04 - desc BLKSIZE"]
    #[inline(always)]
    pub const fn blksize(&self) -> &Blksize {
        &self.blksize
    }
    #[doc = "0x06 - desc BLKCNT"]
    #[inline(always)]
    pub const fn blkcnt(&self) -> &Blkcnt {
        &self.blkcnt
    }
    #[doc = "0x08 - desc ARG0"]
    #[inline(always)]
    pub const fn arg0(&self) -> &Arg0 {
        &self.arg0
    }
    #[doc = "0x0a - desc ARG1"]
    #[inline(always)]
    pub const fn arg1(&self) -> &Arg1 {
        &self.arg1
    }
    #[doc = "0x0c - desc TRANSMODE"]
    #[inline(always)]
    pub const fn transmode(&self) -> &Transmode {
        &self.transmode
    }
    #[doc = "0x0e - desc CMD"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x10 - desc RESP0"]
    #[inline(always)]
    pub const fn resp0(&self) -> &Resp0 {
        &self.resp0
    }
    #[doc = "0x12 - desc RESP1"]
    #[inline(always)]
    pub const fn resp1(&self) -> &Resp1 {
        &self.resp1
    }
    #[doc = "0x14 - desc RESP2"]
    #[inline(always)]
    pub const fn resp2(&self) -> &Resp2 {
        &self.resp2
    }
    #[doc = "0x16 - desc RESP3"]
    #[inline(always)]
    pub const fn resp3(&self) -> &Resp3 {
        &self.resp3
    }
    #[doc = "0x18 - desc RESP4"]
    #[inline(always)]
    pub const fn resp4(&self) -> &Resp4 {
        &self.resp4
    }
    #[doc = "0x1a - desc RESP5"]
    #[inline(always)]
    pub const fn resp5(&self) -> &Resp5 {
        &self.resp5
    }
    #[doc = "0x1c - desc RESP6"]
    #[inline(always)]
    pub const fn resp6(&self) -> &Resp6 {
        &self.resp6
    }
    #[doc = "0x1e - desc RESP7"]
    #[inline(always)]
    pub const fn resp7(&self) -> &Resp7 {
        &self.resp7
    }
    #[doc = "0x20 - desc BUF0"]
    #[inline(always)]
    pub const fn buf0(&self) -> &Buf0 {
        &self.buf0
    }
    #[doc = "0x22 - desc BUF1"]
    #[inline(always)]
    pub const fn buf1(&self) -> &Buf1 {
        &self.buf1
    }
    #[doc = "0x24 - desc PSTAT"]
    #[inline(always)]
    pub const fn pstat(&self) -> &Pstat {
        &self.pstat
    }
    #[doc = "0x28 - desc HOSTCON"]
    #[inline(always)]
    pub const fn hostcon(&self) -> &Hostcon {
        &self.hostcon
    }
    #[doc = "0x29 - desc PWRCON"]
    #[inline(always)]
    pub const fn pwrcon(&self) -> &Pwrcon {
        &self.pwrcon
    }
    #[doc = "0x2a - desc BLKGPCON"]
    #[inline(always)]
    pub const fn blkgpcon(&self) -> &Blkgpcon {
        &self.blkgpcon
    }
    #[doc = "0x2c - desc CLKCON"]
    #[inline(always)]
    pub const fn clkcon(&self) -> &Clkcon {
        &self.clkcon
    }
    #[doc = "0x2e - desc TOUTCON"]
    #[inline(always)]
    pub const fn toutcon(&self) -> &Toutcon {
        &self.toutcon
    }
    #[doc = "0x2f - desc SFTRST"]
    #[inline(always)]
    pub const fn sftrst(&self) -> &Sftrst {
        &self.sftrst
    }
    #[doc = "0x30 - desc NORINTST"]
    #[inline(always)]
    pub const fn norintst(&self) -> &Norintst {
        &self.norintst
    }
    #[doc = "0x32 - desc ERRINTST"]
    #[inline(always)]
    pub const fn errintst(&self) -> &Errintst {
        &self.errintst
    }
    #[doc = "0x34 - desc NORINTSTEN"]
    #[inline(always)]
    pub const fn norintsten(&self) -> &Norintsten {
        &self.norintsten
    }
    #[doc = "0x36 - desc ERRINTSTEN"]
    #[inline(always)]
    pub const fn errintsten(&self) -> &Errintsten {
        &self.errintsten
    }
    #[doc = "0x38 - desc NORINTSGEN"]
    #[inline(always)]
    pub const fn norintsgen(&self) -> &Norintsgen {
        &self.norintsgen
    }
    #[doc = "0x3a - desc ERRINTSGEN"]
    #[inline(always)]
    pub const fn errintsgen(&self) -> &Errintsgen {
        &self.errintsgen
    }
    #[doc = "0x3c - desc ATCERRST"]
    #[inline(always)]
    pub const fn atcerrst(&self) -> &Atcerrst {
        &self.atcerrst
    }
    #[doc = "0x50 - desc FEA"]
    #[inline(always)]
    pub const fn fea(&self) -> &Fea {
        &self.fea
    }
    #[doc = "0x52 - desc FEE"]
    #[inline(always)]
    pub const fn fee(&self) -> &Fee {
        &self.fee
    }
}
#[doc = "BLKSIZE (rw) register accessor: desc BLKSIZE\n\nYou can [`read`](crate::Reg::read) this register and get [`blksize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blksize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blksize`] module"]
#[doc(alias = "BLKSIZE")]
pub type Blksize = crate::Reg<blksize::BlksizeSpec>;
#[doc = "desc BLKSIZE"]
pub mod blksize;
#[doc = "BLKCNT (rw) register accessor: desc BLKCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`blkcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blkcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blkcnt`] module"]
#[doc(alias = "BLKCNT")]
pub type Blkcnt = crate::Reg<blkcnt::BlkcntSpec>;
#[doc = "desc BLKCNT"]
pub mod blkcnt;
#[doc = "ARG0 (rw) register accessor: desc ARG0\n\nYou can [`read`](crate::Reg::read) this register and get [`arg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg0`] module"]
#[doc(alias = "ARG0")]
pub type Arg0 = crate::Reg<arg0::Arg0Spec>;
#[doc = "desc ARG0"]
pub mod arg0;
#[doc = "ARG1 (rw) register accessor: desc ARG1\n\nYou can [`read`](crate::Reg::read) this register and get [`arg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg1`] module"]
#[doc(alias = "ARG1")]
pub type Arg1 = crate::Reg<arg1::Arg1Spec>;
#[doc = "desc ARG1"]
pub mod arg1;
#[doc = "TRANSMODE (rw) register accessor: desc TRANSMODE\n\nYou can [`read`](crate::Reg::read) this register and get [`transmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`transmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmode`] module"]
#[doc(alias = "TRANSMODE")]
pub type Transmode = crate::Reg<transmode::TransmodeSpec>;
#[doc = "desc TRANSMODE"]
pub mod transmode;
#[doc = "CMD (rw) register accessor: desc CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "desc CMD"]
pub mod cmd;
#[doc = "RESP0 (r) register accessor: desc RESP0\n\nYou can [`read`](crate::Reg::read) this register and get [`resp0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp0`] module"]
#[doc(alias = "RESP0")]
pub type Resp0 = crate::Reg<resp0::Resp0Spec>;
#[doc = "desc RESP0"]
pub mod resp0;
#[doc = "RESP1 (r) register accessor: desc RESP1\n\nYou can [`read`](crate::Reg::read) this register and get [`resp1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp1`] module"]
#[doc(alias = "RESP1")]
pub type Resp1 = crate::Reg<resp1::Resp1Spec>;
#[doc = "desc RESP1"]
pub mod resp1;
#[doc = "RESP2 (r) register accessor: desc RESP2\n\nYou can [`read`](crate::Reg::read) this register and get [`resp2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp2`] module"]
#[doc(alias = "RESP2")]
pub type Resp2 = crate::Reg<resp2::Resp2Spec>;
#[doc = "desc RESP2"]
pub mod resp2;
#[doc = "RESP3 (r) register accessor: desc RESP3\n\nYou can [`read`](crate::Reg::read) this register and get [`resp3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp3`] module"]
#[doc(alias = "RESP3")]
pub type Resp3 = crate::Reg<resp3::Resp3Spec>;
#[doc = "desc RESP3"]
pub mod resp3;
#[doc = "RESP4 (r) register accessor: desc RESP4\n\nYou can [`read`](crate::Reg::read) this register and get [`resp4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp4`] module"]
#[doc(alias = "RESP4")]
pub type Resp4 = crate::Reg<resp4::Resp4Spec>;
#[doc = "desc RESP4"]
pub mod resp4;
#[doc = "RESP5 (r) register accessor: desc RESP5\n\nYou can [`read`](crate::Reg::read) this register and get [`resp5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp5`] module"]
#[doc(alias = "RESP5")]
pub type Resp5 = crate::Reg<resp5::Resp5Spec>;
#[doc = "desc RESP5"]
pub mod resp5;
#[doc = "RESP6 (r) register accessor: desc RESP6\n\nYou can [`read`](crate::Reg::read) this register and get [`resp6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp6`] module"]
#[doc(alias = "RESP6")]
pub type Resp6 = crate::Reg<resp6::Resp6Spec>;
#[doc = "desc RESP6"]
pub mod resp6;
#[doc = "RESP7 (r) register accessor: desc RESP7\n\nYou can [`read`](crate::Reg::read) this register and get [`resp7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp7`] module"]
#[doc(alias = "RESP7")]
pub type Resp7 = crate::Reg<resp7::Resp7Spec>;
#[doc = "desc RESP7"]
pub mod resp7;
#[doc = "BUF0 (rw) register accessor: desc BUF0\n\nYou can [`read`](crate::Reg::read) this register and get [`buf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0`] module"]
#[doc(alias = "BUF0")]
pub type Buf0 = crate::Reg<buf0::Buf0Spec>;
#[doc = "desc BUF0"]
pub mod buf0;
#[doc = "BUF1 (rw) register accessor: desc BUF1\n\nYou can [`read`](crate::Reg::read) this register and get [`buf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1`] module"]
#[doc(alias = "BUF1")]
pub type Buf1 = crate::Reg<buf1::Buf1Spec>;
#[doc = "desc BUF1"]
pub mod buf1;
#[doc = "PSTAT (r) register accessor: desc PSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`pstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pstat`] module"]
#[doc(alias = "PSTAT")]
pub type Pstat = crate::Reg<pstat::PstatSpec>;
#[doc = "desc PSTAT"]
pub mod pstat;
#[doc = "HOSTCON (rw) register accessor: desc HOSTCON\n\nYou can [`read`](crate::Reg::read) this register and get [`hostcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostcon`] module"]
#[doc(alias = "HOSTCON")]
pub type Hostcon = crate::Reg<hostcon::HostconSpec>;
#[doc = "desc HOSTCON"]
pub mod hostcon;
#[doc = "PWRCON (rw) register accessor: desc PWRCON\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcon`] module"]
#[doc(alias = "PWRCON")]
pub type Pwrcon = crate::Reg<pwrcon::PwrconSpec>;
#[doc = "desc PWRCON"]
pub mod pwrcon;
#[doc = "BLKGPCON (rw) register accessor: desc BLKGPCON\n\nYou can [`read`](crate::Reg::read) this register and get [`blkgpcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blkgpcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blkgpcon`] module"]
#[doc(alias = "BLKGPCON")]
pub type Blkgpcon = crate::Reg<blkgpcon::BlkgpconSpec>;
#[doc = "desc BLKGPCON"]
pub mod blkgpcon;
#[doc = "CLKCON (rw) register accessor: desc CLKCON\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcon`] module"]
#[doc(alias = "CLKCON")]
pub type Clkcon = crate::Reg<clkcon::ClkconSpec>;
#[doc = "desc CLKCON"]
pub mod clkcon;
#[doc = "TOUTCON (rw) register accessor: desc TOUTCON\n\nYou can [`read`](crate::Reg::read) this register and get [`toutcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`toutcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@toutcon`] module"]
#[doc(alias = "TOUTCON")]
pub type Toutcon = crate::Reg<toutcon::ToutconSpec>;
#[doc = "desc TOUTCON"]
pub mod toutcon;
#[doc = "SFTRST (rw) register accessor: desc SFTRST\n\nYou can [`read`](crate::Reg::read) this register and get [`sftrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sftrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sftrst`] module"]
#[doc(alias = "SFTRST")]
pub type Sftrst = crate::Reg<sftrst::SftrstSpec>;
#[doc = "desc SFTRST"]
pub mod sftrst;
#[doc = "NORINTST (rw) register accessor: desc NORINTST\n\nYou can [`read`](crate::Reg::read) this register and get [`norintst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`norintst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@norintst`] module"]
#[doc(alias = "NORINTST")]
pub type Norintst = crate::Reg<norintst::NorintstSpec>;
#[doc = "desc NORINTST"]
pub mod norintst;
#[doc = "ERRINTST (rw) register accessor: desc ERRINTST\n\nYou can [`read`](crate::Reg::read) this register and get [`errintst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errintst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errintst`] module"]
#[doc(alias = "ERRINTST")]
pub type Errintst = crate::Reg<errintst::ErrintstSpec>;
#[doc = "desc ERRINTST"]
pub mod errintst;
#[doc = "NORINTSTEN (rw) register accessor: desc NORINTSTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`norintsten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`norintsten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@norintsten`] module"]
#[doc(alias = "NORINTSTEN")]
pub type Norintsten = crate::Reg<norintsten::NorintstenSpec>;
#[doc = "desc NORINTSTEN"]
pub mod norintsten;
#[doc = "ERRINTSTEN (rw) register accessor: desc ERRINTSTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`errintsten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errintsten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errintsten`] module"]
#[doc(alias = "ERRINTSTEN")]
pub type Errintsten = crate::Reg<errintsten::ErrintstenSpec>;
#[doc = "desc ERRINTSTEN"]
pub mod errintsten;
#[doc = "NORINTSGEN (rw) register accessor: desc NORINTSGEN\n\nYou can [`read`](crate::Reg::read) this register and get [`norintsgen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`norintsgen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@norintsgen`] module"]
#[doc(alias = "NORINTSGEN")]
pub type Norintsgen = crate::Reg<norintsgen::NorintsgenSpec>;
#[doc = "desc NORINTSGEN"]
pub mod norintsgen;
#[doc = "ERRINTSGEN (rw) register accessor: desc ERRINTSGEN\n\nYou can [`read`](crate::Reg::read) this register and get [`errintsgen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errintsgen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errintsgen`] module"]
#[doc(alias = "ERRINTSGEN")]
pub type Errintsgen = crate::Reg<errintsgen::ErrintsgenSpec>;
#[doc = "desc ERRINTSGEN"]
pub mod errintsgen;
#[doc = "ATCERRST (r) register accessor: desc ATCERRST\n\nYou can [`read`](crate::Reg::read) this register and get [`atcerrst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atcerrst`] module"]
#[doc(alias = "ATCERRST")]
pub type Atcerrst = crate::Reg<atcerrst::AtcerrstSpec>;
#[doc = "desc ATCERRST"]
pub mod atcerrst;
#[doc = "FEA (w) register accessor: desc FEA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fea::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fea`] module"]
#[doc(alias = "FEA")]
pub type Fea = crate::Reg<fea::FeaSpec>;
#[doc = "desc FEA"]
pub mod fea;
#[doc = "FEE (w) register accessor: desc FEE\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fee::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fee`] module"]
#[doc(alias = "FEE")]
pub type Fee = crate::Reg<fee::FeeSpec>;
#[doc = "desc FEE"]
pub mod fee;
