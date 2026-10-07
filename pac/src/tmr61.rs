#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cnter: Cnter,
    perar: Perar,
    perbr: Perbr,
    percr: Percr,
    gcmar: Gcmar,
    gcmbr: Gcmbr,
    gcmcr: Gcmcr,
    gcmdr: Gcmdr,
    gcmer: Gcmer,
    gcmfr: Gcmfr,
    scmar: Scmar,
    scmbr: Scmbr,
    scmcr: Scmcr,
    scmdr: Scmdr,
    scmer: Scmer,
    scmfr: Scmfr,
    dtuar: Dtuar,
    dtdar: Dtdar,
    dtubr: Dtubr,
    dtdbr: Dtdbr,
    gconr: Gconr,
    iconr: Iconr,
    pconr: Pconr,
    bconr: Bconr,
    dconr: Dconr,
    _reserved25: [u8; 0x04],
    fconr: Fconr,
    vperr: Vperr,
    stflr: Stflr,
    hstar: Hstar,
    hstpr: Hstpr,
    hclrr: Hclrr,
    hcpar: Hcpar,
    hcpbr: Hcpbr,
    hcupr: Hcupr,
    hcdor: Hcdor,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CNTER"]
    #[inline(always)]
    pub const fn cnter(&self) -> &Cnter {
        &self.cnter
    }
    #[doc = "0x04 - desc PERAR"]
    #[inline(always)]
    pub const fn perar(&self) -> &Perar {
        &self.perar
    }
    #[doc = "0x08 - desc PERBR"]
    #[inline(always)]
    pub const fn perbr(&self) -> &Perbr {
        &self.perbr
    }
    #[doc = "0x0c - desc PERCR"]
    #[inline(always)]
    pub const fn percr(&self) -> &Percr {
        &self.percr
    }
    #[doc = "0x10 - desc GCMAR"]
    #[inline(always)]
    pub const fn gcmar(&self) -> &Gcmar {
        &self.gcmar
    }
    #[doc = "0x14 - desc GCMBR"]
    #[inline(always)]
    pub const fn gcmbr(&self) -> &Gcmbr {
        &self.gcmbr
    }
    #[doc = "0x18 - desc GCMCR"]
    #[inline(always)]
    pub const fn gcmcr(&self) -> &Gcmcr {
        &self.gcmcr
    }
    #[doc = "0x1c - desc GCMDR"]
    #[inline(always)]
    pub const fn gcmdr(&self) -> &Gcmdr {
        &self.gcmdr
    }
    #[doc = "0x20 - desc GCMER"]
    #[inline(always)]
    pub const fn gcmer(&self) -> &Gcmer {
        &self.gcmer
    }
    #[doc = "0x24 - desc GCMFR"]
    #[inline(always)]
    pub const fn gcmfr(&self) -> &Gcmfr {
        &self.gcmfr
    }
    #[doc = "0x28 - desc SCMAR"]
    #[inline(always)]
    pub const fn scmar(&self) -> &Scmar {
        &self.scmar
    }
    #[doc = "0x2c - desc SCMBR"]
    #[inline(always)]
    pub const fn scmbr(&self) -> &Scmbr {
        &self.scmbr
    }
    #[doc = "0x30 - desc SCMCR"]
    #[inline(always)]
    pub const fn scmcr(&self) -> &Scmcr {
        &self.scmcr
    }
    #[doc = "0x34 - desc SCMDR"]
    #[inline(always)]
    pub const fn scmdr(&self) -> &Scmdr {
        &self.scmdr
    }
    #[doc = "0x38 - desc SCMER"]
    #[inline(always)]
    pub const fn scmer(&self) -> &Scmer {
        &self.scmer
    }
    #[doc = "0x3c - desc SCMFR"]
    #[inline(always)]
    pub const fn scmfr(&self) -> &Scmfr {
        &self.scmfr
    }
    #[doc = "0x40 - desc DTUAR"]
    #[inline(always)]
    pub const fn dtuar(&self) -> &Dtuar {
        &self.dtuar
    }
    #[doc = "0x44 - desc DTDAR"]
    #[inline(always)]
    pub const fn dtdar(&self) -> &Dtdar {
        &self.dtdar
    }
    #[doc = "0x48 - desc DTUBR"]
    #[inline(always)]
    pub const fn dtubr(&self) -> &Dtubr {
        &self.dtubr
    }
    #[doc = "0x4c - desc DTDBR"]
    #[inline(always)]
    pub const fn dtdbr(&self) -> &Dtdbr {
        &self.dtdbr
    }
    #[doc = "0x50 - desc GCONR"]
    #[inline(always)]
    pub const fn gconr(&self) -> &Gconr {
        &self.gconr
    }
    #[doc = "0x54 - desc ICONR"]
    #[inline(always)]
    pub const fn iconr(&self) -> &Iconr {
        &self.iconr
    }
    #[doc = "0x58 - desc PCONR"]
    #[inline(always)]
    pub const fn pconr(&self) -> &Pconr {
        &self.pconr
    }
    #[doc = "0x5c - desc BCONR"]
    #[inline(always)]
    pub const fn bconr(&self) -> &Bconr {
        &self.bconr
    }
    #[doc = "0x60 - desc DCONR"]
    #[inline(always)]
    pub const fn dconr(&self) -> &Dconr {
        &self.dconr
    }
    #[doc = "0x68 - desc FCONR"]
    #[inline(always)]
    pub const fn fconr(&self) -> &Fconr {
        &self.fconr
    }
    #[doc = "0x6c - desc VPERR"]
    #[inline(always)]
    pub const fn vperr(&self) -> &Vperr {
        &self.vperr
    }
    #[doc = "0x70 - desc STFLR"]
    #[inline(always)]
    pub const fn stflr(&self) -> &Stflr {
        &self.stflr
    }
    #[doc = "0x74 - desc HSTAR"]
    #[inline(always)]
    pub const fn hstar(&self) -> &Hstar {
        &self.hstar
    }
    #[doc = "0x78 - desc HSTPR"]
    #[inline(always)]
    pub const fn hstpr(&self) -> &Hstpr {
        &self.hstpr
    }
    #[doc = "0x7c - desc HCLRR"]
    #[inline(always)]
    pub const fn hclrr(&self) -> &Hclrr {
        &self.hclrr
    }
    #[doc = "0x80 - desc HCPAR"]
    #[inline(always)]
    pub const fn hcpar(&self) -> &Hcpar {
        &self.hcpar
    }
    #[doc = "0x84 - desc HCPBR"]
    #[inline(always)]
    pub const fn hcpbr(&self) -> &Hcpbr {
        &self.hcpbr
    }
    #[doc = "0x88 - desc HCUPR"]
    #[inline(always)]
    pub const fn hcupr(&self) -> &Hcupr {
        &self.hcupr
    }
    #[doc = "0x8c - desc HCDOR"]
    #[inline(always)]
    pub const fn hcdor(&self) -> &Hcdor {
        &self.hcdor
    }
}
#[doc = "CNTER (rw) register accessor: desc CNTER\n\nYou can [`read`](crate::Reg::read) this register and get [`cnter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnter`] module"]
#[doc(alias = "CNTER")]
pub type Cnter = crate::Reg<cnter::CnterSpec>;
#[doc = "desc CNTER"]
pub mod cnter;
#[doc = "PERAR (rw) register accessor: desc PERAR\n\nYou can [`read`](crate::Reg::read) this register and get [`perar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perar`] module"]
#[doc(alias = "PERAR")]
pub type Perar = crate::Reg<perar::PerarSpec>;
#[doc = "desc PERAR"]
pub mod perar;
#[doc = "PERBR (rw) register accessor: desc PERBR\n\nYou can [`read`](crate::Reg::read) this register and get [`perbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perbr`] module"]
#[doc(alias = "PERBR")]
pub type Perbr = crate::Reg<perbr::PerbrSpec>;
#[doc = "desc PERBR"]
pub mod perbr;
#[doc = "PERCR (rw) register accessor: desc PERCR\n\nYou can [`read`](crate::Reg::read) this register and get [`percr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`percr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@percr`] module"]
#[doc(alias = "PERCR")]
pub type Percr = crate::Reg<percr::PercrSpec>;
#[doc = "desc PERCR"]
pub mod percr;
#[doc = "GCMAR (rw) register accessor: desc GCMAR\n\nYou can [`read`](crate::Reg::read) this register and get [`gcmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcmar`] module"]
#[doc(alias = "GCMAR")]
pub type Gcmar = crate::Reg<gcmar::GcmarSpec>;
#[doc = "desc GCMAR"]
pub mod gcmar;
#[doc = "GCMBR (rw) register accessor: desc GCMBR\n\nYou can [`read`](crate::Reg::read) this register and get [`gcmbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcmbr`] module"]
#[doc(alias = "GCMBR")]
pub type Gcmbr = crate::Reg<gcmbr::GcmbrSpec>;
#[doc = "desc GCMBR"]
pub mod gcmbr;
#[doc = "GCMCR (rw) register accessor: desc GCMCR\n\nYou can [`read`](crate::Reg::read) this register and get [`gcmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcmcr`] module"]
#[doc(alias = "GCMCR")]
pub type Gcmcr = crate::Reg<gcmcr::GcmcrSpec>;
#[doc = "desc GCMCR"]
pub mod gcmcr;
#[doc = "GCMDR (rw) register accessor: desc GCMDR\n\nYou can [`read`](crate::Reg::read) this register and get [`gcmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcmdr`] module"]
#[doc(alias = "GCMDR")]
pub type Gcmdr = crate::Reg<gcmdr::GcmdrSpec>;
#[doc = "desc GCMDR"]
pub mod gcmdr;
#[doc = "GCMER (rw) register accessor: desc GCMER\n\nYou can [`read`](crate::Reg::read) this register and get [`gcmer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcmer`] module"]
#[doc(alias = "GCMER")]
pub type Gcmer = crate::Reg<gcmer::GcmerSpec>;
#[doc = "desc GCMER"]
pub mod gcmer;
#[doc = "GCMFR (rw) register accessor: desc GCMFR\n\nYou can [`read`](crate::Reg::read) this register and get [`gcmfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcmfr`] module"]
#[doc(alias = "GCMFR")]
pub type Gcmfr = crate::Reg<gcmfr::GcmfrSpec>;
#[doc = "desc GCMFR"]
pub mod gcmfr;
#[doc = "SCMAR (rw) register accessor: desc SCMAR\n\nYou can [`read`](crate::Reg::read) this register and get [`scmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmar`] module"]
#[doc(alias = "SCMAR")]
pub type Scmar = crate::Reg<scmar::ScmarSpec>;
#[doc = "desc SCMAR"]
pub mod scmar;
#[doc = "SCMBR (rw) register accessor: desc SCMBR\n\nYou can [`read`](crate::Reg::read) this register and get [`scmbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmbr`] module"]
#[doc(alias = "SCMBR")]
pub type Scmbr = crate::Reg<scmbr::ScmbrSpec>;
#[doc = "desc SCMBR"]
pub mod scmbr;
#[doc = "SCMCR (rw) register accessor: desc SCMCR\n\nYou can [`read`](crate::Reg::read) this register and get [`scmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmcr`] module"]
#[doc(alias = "SCMCR")]
pub type Scmcr = crate::Reg<scmcr::ScmcrSpec>;
#[doc = "desc SCMCR"]
pub mod scmcr;
#[doc = "SCMDR (rw) register accessor: desc SCMDR\n\nYou can [`read`](crate::Reg::read) this register and get [`scmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmdr`] module"]
#[doc(alias = "SCMDR")]
pub type Scmdr = crate::Reg<scmdr::ScmdrSpec>;
#[doc = "desc SCMDR"]
pub mod scmdr;
#[doc = "SCMER (rw) register accessor: desc SCMER\n\nYou can [`read`](crate::Reg::read) this register and get [`scmer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmer`] module"]
#[doc(alias = "SCMER")]
pub type Scmer = crate::Reg<scmer::ScmerSpec>;
#[doc = "desc SCMER"]
pub mod scmer;
#[doc = "SCMFR (rw) register accessor: desc SCMFR\n\nYou can [`read`](crate::Reg::read) this register and get [`scmfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmfr`] module"]
#[doc(alias = "SCMFR")]
pub type Scmfr = crate::Reg<scmfr::ScmfrSpec>;
#[doc = "desc SCMFR"]
pub mod scmfr;
#[doc = "DTUAR (rw) register accessor: desc DTUAR\n\nYou can [`read`](crate::Reg::read) this register and get [`dtuar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtuar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtuar`] module"]
#[doc(alias = "DTUAR")]
pub type Dtuar = crate::Reg<dtuar::DtuarSpec>;
#[doc = "desc DTUAR"]
pub mod dtuar;
#[doc = "DTDAR (rw) register accessor: desc DTDAR\n\nYou can [`read`](crate::Reg::read) this register and get [`dtdar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtdar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtdar`] module"]
#[doc(alias = "DTDAR")]
pub type Dtdar = crate::Reg<dtdar::DtdarSpec>;
#[doc = "desc DTDAR"]
pub mod dtdar;
#[doc = "DTUBR (rw) register accessor: desc DTUBR\n\nYou can [`read`](crate::Reg::read) this register and get [`dtubr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtubr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtubr`] module"]
#[doc(alias = "DTUBR")]
pub type Dtubr = crate::Reg<dtubr::DtubrSpec>;
#[doc = "desc DTUBR"]
pub mod dtubr;
#[doc = "DTDBR (rw) register accessor: desc DTDBR\n\nYou can [`read`](crate::Reg::read) this register and get [`dtdbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtdbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtdbr`] module"]
#[doc(alias = "DTDBR")]
pub type Dtdbr = crate::Reg<dtdbr::DtdbrSpec>;
#[doc = "desc DTDBR"]
pub mod dtdbr;
#[doc = "GCONR (rw) register accessor: desc GCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`gconr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gconr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gconr`] module"]
#[doc(alias = "GCONR")]
pub type Gconr = crate::Reg<gconr::GconrSpec>;
#[doc = "desc GCONR"]
pub mod gconr;
#[doc = "ICONR (rw) register accessor: desc ICONR\n\nYou can [`read`](crate::Reg::read) this register and get [`iconr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iconr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iconr`] module"]
#[doc(alias = "ICONR")]
pub type Iconr = crate::Reg<iconr::IconrSpec>;
#[doc = "desc ICONR"]
pub mod iconr;
#[doc = "PCONR (rw) register accessor: desc PCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`pconr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconr`] module"]
#[doc(alias = "PCONR")]
pub type Pconr = crate::Reg<pconr::PconrSpec>;
#[doc = "desc PCONR"]
pub mod pconr;
#[doc = "BCONR (rw) register accessor: desc BCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`bconr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bconr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bconr`] module"]
#[doc(alias = "BCONR")]
pub type Bconr = crate::Reg<bconr::BconrSpec>;
#[doc = "desc BCONR"]
pub mod bconr;
#[doc = "DCONR (rw) register accessor: desc DCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`dconr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dconr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dconr`] module"]
#[doc(alias = "DCONR")]
pub type Dconr = crate::Reg<dconr::DconrSpec>;
#[doc = "desc DCONR"]
pub mod dconr;
#[doc = "FCONR (rw) register accessor: desc FCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`fconr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fconr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fconr`] module"]
#[doc(alias = "FCONR")]
pub type Fconr = crate::Reg<fconr::FconrSpec>;
#[doc = "desc FCONR"]
pub mod fconr;
#[doc = "VPERR (rw) register accessor: desc VPERR\n\nYou can [`read`](crate::Reg::read) this register and get [`vperr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vperr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vperr`] module"]
#[doc(alias = "VPERR")]
pub type Vperr = crate::Reg<vperr::VperrSpec>;
#[doc = "desc VPERR"]
pub mod vperr;
#[doc = "STFLR (rw) register accessor: desc STFLR\n\nYou can [`read`](crate::Reg::read) this register and get [`stflr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stflr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stflr`] module"]
#[doc(alias = "STFLR")]
pub type Stflr = crate::Reg<stflr::StflrSpec>;
#[doc = "desc STFLR"]
pub mod stflr;
#[doc = "HSTAR (rw) register accessor: desc HSTAR\n\nYou can [`read`](crate::Reg::read) this register and get [`hstar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstar`] module"]
#[doc(alias = "HSTAR")]
pub type Hstar = crate::Reg<hstar::HstarSpec>;
#[doc = "desc HSTAR"]
pub mod hstar;
#[doc = "HSTPR (rw) register accessor: desc HSTPR\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstpr`] module"]
#[doc(alias = "HSTPR")]
pub type Hstpr = crate::Reg<hstpr::HstprSpec>;
#[doc = "desc HSTPR"]
pub mod hstpr;
#[doc = "HCLRR (rw) register accessor: desc HCLRR\n\nYou can [`read`](crate::Reg::read) this register and get [`hclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hclrr`] module"]
#[doc(alias = "HCLRR")]
pub type Hclrr = crate::Reg<hclrr::HclrrSpec>;
#[doc = "desc HCLRR"]
pub mod hclrr;
#[doc = "HCPAR (rw) register accessor: desc HCPAR\n\nYou can [`read`](crate::Reg::read) this register and get [`hcpar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcpar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcpar`] module"]
#[doc(alias = "HCPAR")]
pub type Hcpar = crate::Reg<hcpar::HcparSpec>;
#[doc = "desc HCPAR"]
pub mod hcpar;
#[doc = "HCPBR (rw) register accessor: desc HCPBR\n\nYou can [`read`](crate::Reg::read) this register and get [`hcpbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcpbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcpbr`] module"]
#[doc(alias = "HCPBR")]
pub type Hcpbr = crate::Reg<hcpbr::HcpbrSpec>;
#[doc = "desc HCPBR"]
pub mod hcpbr;
#[doc = "HCUPR (rw) register accessor: desc HCUPR\n\nYou can [`read`](crate::Reg::read) this register and get [`hcupr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcupr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcupr`] module"]
#[doc(alias = "HCUPR")]
pub type Hcupr = crate::Reg<hcupr::HcuprSpec>;
#[doc = "desc HCUPR"]
pub mod hcupr;
#[doc = "HCDOR (rw) register accessor: desc HCDOR\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdor`] module"]
#[doc(alias = "HCDOR")]
pub type Hcdor = crate::Reg<hcdor::HcdorSpec>;
#[doc = "desc HCDOR"]
pub mod hcdor;
