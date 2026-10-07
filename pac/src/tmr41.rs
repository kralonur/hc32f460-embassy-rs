#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x02],
    occruh: Occruh,
    _reserved1: [u8; 0x02],
    occrul: Occrul,
    _reserved2: [u8; 0x02],
    occrvh: Occrvh,
    _reserved3: [u8; 0x02],
    occrvl: Occrvl,
    _reserved4: [u8; 0x02],
    occrwh: Occrwh,
    _reserved5: [u8; 0x02],
    occrwl: Occrwl,
    ocsru: Ocsru,
    oceru: Oceru,
    ocsrv: Ocsrv,
    ocerv: Ocerv,
    ocsrw: Ocsrw,
    ocerw: Ocerw,
    ocmrhuh: Ocmrhuh,
    _reserved13: [u8; 0x02],
    ocmrlul: Ocmrlul,
    ocmrhvh: Ocmrhvh,
    _reserved15: [u8; 0x02],
    ocmrlvl: Ocmrlvl,
    ocmrhwh: Ocmrhwh,
    _reserved17: [u8; 0x02],
    ocmrlwl: Ocmrlwl,
    _reserved18: [u8; 0x06],
    cpsr: Cpsr,
    _reserved19: [u8; 0x02],
    cntr: Cntr,
    ccsr: Ccsr,
    cvpr: Cvpr,
    _reserved22: [u8; 0x36],
    pfsru: Pfsru,
    pdaru: Pdaru,
    pdbru: Pdbru,
    _reserved25: [u8; 0x02],
    pfsrv: Pfsrv,
    pdarv: Pdarv,
    pdbrv: Pdbrv,
    _reserved28: [u8; 0x02],
    pfsrw: Pfsrw,
    pdarw: Pdarw,
    pdbrw: Pdbrw,
    pocru: Pocru,
    _reserved32: [u8; 0x02],
    pocrv: Pocrv,
    _reserved33: [u8; 0x02],
    pocrw: Pocrw,
    _reserved34: [u8; 0x02],
    rcsr: Rcsr,
    _reserved35: [u8; 0x0c],
    sccruh: Sccruh,
    _reserved36: [u8; 0x02],
    sccrul: Sccrul,
    _reserved37: [u8; 0x02],
    sccrvh: Sccrvh,
    _reserved38: [u8; 0x02],
    sccrvl: Sccrvl,
    _reserved39: [u8; 0x02],
    sccrwh: Sccrwh,
    _reserved40: [u8; 0x02],
    sccrwl: Sccrwl,
    scsruh: Scsruh,
    scmruh: Scmruh,
    scsrul: Scsrul,
    scmrul: Scmrul,
    scsrvh: Scsrvh,
    scmrvh: Scmrvh,
    scsrvl: Scsrvl,
    scmrvl: Scmrvl,
    scsrwh: Scsrwh,
    scmrwh: Scmrwh,
    scsrwl: Scsrwl,
    scmrwl: Scmrwl,
    _reserved53: [u8; 0x10],
    ecsr: Ecsr,
}
impl RegisterBlock {
    #[doc = "0x02 - desc OCCRUH"]
    #[inline(always)]
    pub const fn occruh(&self) -> &Occruh {
        &self.occruh
    }
    #[doc = "0x06 - desc OCCRUL"]
    #[inline(always)]
    pub const fn occrul(&self) -> &Occrul {
        &self.occrul
    }
    #[doc = "0x0a - desc OCCRVH"]
    #[inline(always)]
    pub const fn occrvh(&self) -> &Occrvh {
        &self.occrvh
    }
    #[doc = "0x0e - desc OCCRVL"]
    #[inline(always)]
    pub const fn occrvl(&self) -> &Occrvl {
        &self.occrvl
    }
    #[doc = "0x12 - desc OCCRWH"]
    #[inline(always)]
    pub const fn occrwh(&self) -> &Occrwh {
        &self.occrwh
    }
    #[doc = "0x16 - desc OCCRWL"]
    #[inline(always)]
    pub const fn occrwl(&self) -> &Occrwl {
        &self.occrwl
    }
    #[doc = "0x18 - desc OCSRU"]
    #[inline(always)]
    pub const fn ocsru(&self) -> &Ocsru {
        &self.ocsru
    }
    #[doc = "0x1a - desc OCERU"]
    #[inline(always)]
    pub const fn oceru(&self) -> &Oceru {
        &self.oceru
    }
    #[doc = "0x1c - desc OCSRV"]
    #[inline(always)]
    pub const fn ocsrv(&self) -> &Ocsrv {
        &self.ocsrv
    }
    #[doc = "0x1e - desc OCERV"]
    #[inline(always)]
    pub const fn ocerv(&self) -> &Ocerv {
        &self.ocerv
    }
    #[doc = "0x20 - desc OCSRW"]
    #[inline(always)]
    pub const fn ocsrw(&self) -> &Ocsrw {
        &self.ocsrw
    }
    #[doc = "0x22 - desc OCERW"]
    #[inline(always)]
    pub const fn ocerw(&self) -> &Ocerw {
        &self.ocerw
    }
    #[doc = "0x24 - desc OCMRHUH"]
    #[inline(always)]
    pub const fn ocmrhuh(&self) -> &Ocmrhuh {
        &self.ocmrhuh
    }
    #[doc = "0x28 - desc OCMRLUL"]
    #[inline(always)]
    pub const fn ocmrlul(&self) -> &Ocmrlul {
        &self.ocmrlul
    }
    #[doc = "0x2c - desc OCMRHVH"]
    #[inline(always)]
    pub const fn ocmrhvh(&self) -> &Ocmrhvh {
        &self.ocmrhvh
    }
    #[doc = "0x30 - desc OCMRLVL"]
    #[inline(always)]
    pub const fn ocmrlvl(&self) -> &Ocmrlvl {
        &self.ocmrlvl
    }
    #[doc = "0x34 - desc OCMRHWH"]
    #[inline(always)]
    pub const fn ocmrhwh(&self) -> &Ocmrhwh {
        &self.ocmrhwh
    }
    #[doc = "0x38 - desc OCMRLWL"]
    #[inline(always)]
    pub const fn ocmrlwl(&self) -> &Ocmrlwl {
        &self.ocmrlwl
    }
    #[doc = "0x42 - desc CPSR"]
    #[inline(always)]
    pub const fn cpsr(&self) -> &Cpsr {
        &self.cpsr
    }
    #[doc = "0x46 - desc CNTR"]
    #[inline(always)]
    pub const fn cntr(&self) -> &Cntr {
        &self.cntr
    }
    #[doc = "0x48 - desc CCSR"]
    #[inline(always)]
    pub const fn ccsr(&self) -> &Ccsr {
        &self.ccsr
    }
    #[doc = "0x4a - desc CVPR"]
    #[inline(always)]
    pub const fn cvpr(&self) -> &Cvpr {
        &self.cvpr
    }
    #[doc = "0x82 - desc PFSRU"]
    #[inline(always)]
    pub const fn pfsru(&self) -> &Pfsru {
        &self.pfsru
    }
    #[doc = "0x84 - desc PDARU"]
    #[inline(always)]
    pub const fn pdaru(&self) -> &Pdaru {
        &self.pdaru
    }
    #[doc = "0x86 - desc PDBRU"]
    #[inline(always)]
    pub const fn pdbru(&self) -> &Pdbru {
        &self.pdbru
    }
    #[doc = "0x8a - desc PFSRV"]
    #[inline(always)]
    pub const fn pfsrv(&self) -> &Pfsrv {
        &self.pfsrv
    }
    #[doc = "0x8c - desc PDARV"]
    #[inline(always)]
    pub const fn pdarv(&self) -> &Pdarv {
        &self.pdarv
    }
    #[doc = "0x8e - desc PDBRV"]
    #[inline(always)]
    pub const fn pdbrv(&self) -> &Pdbrv {
        &self.pdbrv
    }
    #[doc = "0x92 - desc PFSRW"]
    #[inline(always)]
    pub const fn pfsrw(&self) -> &Pfsrw {
        &self.pfsrw
    }
    #[doc = "0x94 - desc PDARW"]
    #[inline(always)]
    pub const fn pdarw(&self) -> &Pdarw {
        &self.pdarw
    }
    #[doc = "0x96 - desc PDBRW"]
    #[inline(always)]
    pub const fn pdbrw(&self) -> &Pdbrw {
        &self.pdbrw
    }
    #[doc = "0x98 - desc POCRU"]
    #[inline(always)]
    pub const fn pocru(&self) -> &Pocru {
        &self.pocru
    }
    #[doc = "0x9c - desc POCRV"]
    #[inline(always)]
    pub const fn pocrv(&self) -> &Pocrv {
        &self.pocrv
    }
    #[doc = "0xa0 - desc POCRW"]
    #[inline(always)]
    pub const fn pocrw(&self) -> &Pocrw {
        &self.pocrw
    }
    #[doc = "0xa4 - desc RCSR"]
    #[inline(always)]
    pub const fn rcsr(&self) -> &Rcsr {
        &self.rcsr
    }
    #[doc = "0xb2 - desc SCCRUH"]
    #[inline(always)]
    pub const fn sccruh(&self) -> &Sccruh {
        &self.sccruh
    }
    #[doc = "0xb6 - desc SCCRUL"]
    #[inline(always)]
    pub const fn sccrul(&self) -> &Sccrul {
        &self.sccrul
    }
    #[doc = "0xba - desc SCCRVH"]
    #[inline(always)]
    pub const fn sccrvh(&self) -> &Sccrvh {
        &self.sccrvh
    }
    #[doc = "0xbe - desc SCCRVL"]
    #[inline(always)]
    pub const fn sccrvl(&self) -> &Sccrvl {
        &self.sccrvl
    }
    #[doc = "0xc2 - desc SCCRWH"]
    #[inline(always)]
    pub const fn sccrwh(&self) -> &Sccrwh {
        &self.sccrwh
    }
    #[doc = "0xc6 - desc SCCRWL"]
    #[inline(always)]
    pub const fn sccrwl(&self) -> &Sccrwl {
        &self.sccrwl
    }
    #[doc = "0xc8 - desc SCSRUH"]
    #[inline(always)]
    pub const fn scsruh(&self) -> &Scsruh {
        &self.scsruh
    }
    #[doc = "0xca - desc SCMRUH"]
    #[inline(always)]
    pub const fn scmruh(&self) -> &Scmruh {
        &self.scmruh
    }
    #[doc = "0xcc - desc SCSRUL"]
    #[inline(always)]
    pub const fn scsrul(&self) -> &Scsrul {
        &self.scsrul
    }
    #[doc = "0xce - desc SCMRUL"]
    #[inline(always)]
    pub const fn scmrul(&self) -> &Scmrul {
        &self.scmrul
    }
    #[doc = "0xd0 - desc SCSRVH"]
    #[inline(always)]
    pub const fn scsrvh(&self) -> &Scsrvh {
        &self.scsrvh
    }
    #[doc = "0xd2 - desc SCMRVH"]
    #[inline(always)]
    pub const fn scmrvh(&self) -> &Scmrvh {
        &self.scmrvh
    }
    #[doc = "0xd4 - desc SCSRVL"]
    #[inline(always)]
    pub const fn scsrvl(&self) -> &Scsrvl {
        &self.scsrvl
    }
    #[doc = "0xd6 - desc SCMRVL"]
    #[inline(always)]
    pub const fn scmrvl(&self) -> &Scmrvl {
        &self.scmrvl
    }
    #[doc = "0xd8 - desc SCSRWH"]
    #[inline(always)]
    pub const fn scsrwh(&self) -> &Scsrwh {
        &self.scsrwh
    }
    #[doc = "0xda - desc SCMRWH"]
    #[inline(always)]
    pub const fn scmrwh(&self) -> &Scmrwh {
        &self.scmrwh
    }
    #[doc = "0xdc - desc SCSRWL"]
    #[inline(always)]
    pub const fn scsrwl(&self) -> &Scsrwl {
        &self.scsrwl
    }
    #[doc = "0xde - desc SCMRWL"]
    #[inline(always)]
    pub const fn scmrwl(&self) -> &Scmrwl {
        &self.scmrwl
    }
    #[doc = "0xf0 - desc ECSR"]
    #[inline(always)]
    pub const fn ecsr(&self) -> &Ecsr {
        &self.ecsr
    }
}
#[doc = "OCCRUH (rw) register accessor: desc OCCRUH\n\nYou can [`read`](crate::Reg::read) this register and get [`occruh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occruh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occruh`] module"]
#[doc(alias = "OCCRUH")]
pub type Occruh = crate::Reg<occruh::OccruhSpec>;
#[doc = "desc OCCRUH"]
pub mod occruh;
#[doc = "OCCRUL (rw) register accessor: desc OCCRUL\n\nYou can [`read`](crate::Reg::read) this register and get [`occrul::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrul::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occrul`] module"]
#[doc(alias = "OCCRUL")]
pub type Occrul = crate::Reg<occrul::OccrulSpec>;
#[doc = "desc OCCRUL"]
pub mod occrul;
#[doc = "OCCRVH (rw) register accessor: desc OCCRVH\n\nYou can [`read`](crate::Reg::read) this register and get [`occrvh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrvh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occrvh`] module"]
#[doc(alias = "OCCRVH")]
pub type Occrvh = crate::Reg<occrvh::OccrvhSpec>;
#[doc = "desc OCCRVH"]
pub mod occrvh;
#[doc = "OCCRVL (rw) register accessor: desc OCCRVL\n\nYou can [`read`](crate::Reg::read) this register and get [`occrvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occrvl`] module"]
#[doc(alias = "OCCRVL")]
pub type Occrvl = crate::Reg<occrvl::OccrvlSpec>;
#[doc = "desc OCCRVL"]
pub mod occrvl;
#[doc = "OCCRWH (rw) register accessor: desc OCCRWH\n\nYou can [`read`](crate::Reg::read) this register and get [`occrwh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrwh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occrwh`] module"]
#[doc(alias = "OCCRWH")]
pub type Occrwh = crate::Reg<occrwh::OccrwhSpec>;
#[doc = "desc OCCRWH"]
pub mod occrwh;
#[doc = "OCCRWL (rw) register accessor: desc OCCRWL\n\nYou can [`read`](crate::Reg::read) this register and get [`occrwl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrwl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occrwl`] module"]
#[doc(alias = "OCCRWL")]
pub type Occrwl = crate::Reg<occrwl::OccrwlSpec>;
#[doc = "desc OCCRWL"]
pub mod occrwl;
#[doc = "OCSRU (rw) register accessor: desc OCSRU\n\nYou can [`read`](crate::Reg::read) this register and get [`ocsru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocsru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocsru`] module"]
#[doc(alias = "OCSRU")]
pub type Ocsru = crate::Reg<ocsru::OcsruSpec>;
#[doc = "desc OCSRU"]
pub mod ocsru;
#[doc = "OCERU (rw) register accessor: desc OCERU\n\nYou can [`read`](crate::Reg::read) this register and get [`oceru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oceru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oceru`] module"]
#[doc(alias = "OCERU")]
pub type Oceru = crate::Reg<oceru::OceruSpec>;
#[doc = "desc OCERU"]
pub mod oceru;
#[doc = "OCSRV (rw) register accessor: desc OCSRV\n\nYou can [`read`](crate::Reg::read) this register and get [`ocsrv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocsrv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocsrv`] module"]
#[doc(alias = "OCSRV")]
pub type Ocsrv = crate::Reg<ocsrv::OcsrvSpec>;
#[doc = "desc OCSRV"]
pub mod ocsrv;
#[doc = "OCERV (rw) register accessor: desc OCERV\n\nYou can [`read`](crate::Reg::read) this register and get [`ocerv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocerv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocerv`] module"]
#[doc(alias = "OCERV")]
pub type Ocerv = crate::Reg<ocerv::OcervSpec>;
#[doc = "desc OCERV"]
pub mod ocerv;
#[doc = "OCSRW (rw) register accessor: desc OCSRW\n\nYou can [`read`](crate::Reg::read) this register and get [`ocsrw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocsrw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocsrw`] module"]
#[doc(alias = "OCSRW")]
pub type Ocsrw = crate::Reg<ocsrw::OcsrwSpec>;
#[doc = "desc OCSRW"]
pub mod ocsrw;
#[doc = "OCERW (rw) register accessor: desc OCERW\n\nYou can [`read`](crate::Reg::read) this register and get [`ocerw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocerw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocerw`] module"]
#[doc(alias = "OCERW")]
pub type Ocerw = crate::Reg<ocerw::OcerwSpec>;
#[doc = "desc OCERW"]
pub mod ocerw;
#[doc = "OCMRHUH (rw) register accessor: desc OCMRHUH\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmrhuh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmrhuh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocmrhuh`] module"]
#[doc(alias = "OCMRHUH")]
pub type Ocmrhuh = crate::Reg<ocmrhuh::OcmrhuhSpec>;
#[doc = "desc OCMRHUH"]
pub mod ocmrhuh;
#[doc = "OCMRLUL (rw) register accessor: desc OCMRLUL\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmrlul::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmrlul::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocmrlul`] module"]
#[doc(alias = "OCMRLUL")]
pub type Ocmrlul = crate::Reg<ocmrlul::OcmrlulSpec>;
#[doc = "desc OCMRLUL"]
pub mod ocmrlul;
#[doc = "OCMRHVH (rw) register accessor: desc OCMRHVH\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmrhvh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmrhvh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocmrhvh`] module"]
#[doc(alias = "OCMRHVH")]
pub type Ocmrhvh = crate::Reg<ocmrhvh::OcmrhvhSpec>;
#[doc = "desc OCMRHVH"]
pub mod ocmrhvh;
#[doc = "OCMRLVL (rw) register accessor: desc OCMRLVL\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmrlvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmrlvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocmrlvl`] module"]
#[doc(alias = "OCMRLVL")]
pub type Ocmrlvl = crate::Reg<ocmrlvl::OcmrlvlSpec>;
#[doc = "desc OCMRLVL"]
pub mod ocmrlvl;
#[doc = "OCMRHWH (rw) register accessor: desc OCMRHWH\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmrhwh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmrhwh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocmrhwh`] module"]
#[doc(alias = "OCMRHWH")]
pub type Ocmrhwh = crate::Reg<ocmrhwh::OcmrhwhSpec>;
#[doc = "desc OCMRHWH"]
pub mod ocmrhwh;
#[doc = "OCMRLWL (rw) register accessor: desc OCMRLWL\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmrlwl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmrlwl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocmrlwl`] module"]
#[doc(alias = "OCMRLWL")]
pub type Ocmrlwl = crate::Reg<ocmrlwl::OcmrlwlSpec>;
#[doc = "desc OCMRLWL"]
pub mod ocmrlwl;
#[doc = "CPSR (rw) register accessor: desc CPSR\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsr`] module"]
#[doc(alias = "CPSR")]
pub type Cpsr = crate::Reg<cpsr::CpsrSpec>;
#[doc = "desc CPSR"]
pub mod cpsr;
#[doc = "CNTR (rw) register accessor: desc CNTR\n\nYou can [`read`](crate::Reg::read) this register and get [`cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`] module"]
#[doc(alias = "CNTR")]
pub type Cntr = crate::Reg<cntr::CntrSpec>;
#[doc = "desc CNTR"]
pub mod cntr;
#[doc = "CCSR (rw) register accessor: desc CCSR\n\nYou can [`read`](crate::Reg::read) this register and get [`ccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccsr`] module"]
#[doc(alias = "CCSR")]
pub type Ccsr = crate::Reg<ccsr::CcsrSpec>;
#[doc = "desc CCSR"]
pub mod ccsr;
#[doc = "CVPR (rw) register accessor: desc CVPR\n\nYou can [`read`](crate::Reg::read) this register and get [`cvpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvpr`] module"]
#[doc(alias = "CVPR")]
pub type Cvpr = crate::Reg<cvpr::CvprSpec>;
#[doc = "desc CVPR"]
pub mod cvpr;
#[doc = "PFSRU (rw) register accessor: desc PFSRU\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsru`] module"]
#[doc(alias = "PFSRU")]
pub type Pfsru = crate::Reg<pfsru::PfsruSpec>;
#[doc = "desc PFSRU"]
pub mod pfsru;
#[doc = "PDARU (rw) register accessor: desc PDARU\n\nYou can [`read`](crate::Reg::read) this register and get [`pdaru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdaru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdaru`] module"]
#[doc(alias = "PDARU")]
pub type Pdaru = crate::Reg<pdaru::PdaruSpec>;
#[doc = "desc PDARU"]
pub mod pdaru;
#[doc = "PDBRU (rw) register accessor: desc PDBRU\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbru`] module"]
#[doc(alias = "PDBRU")]
pub type Pdbru = crate::Reg<pdbru::PdbruSpec>;
#[doc = "desc PDBRU"]
pub mod pdbru;
#[doc = "PFSRV (rw) register accessor: desc PFSRV\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrv`] module"]
#[doc(alias = "PFSRV")]
pub type Pfsrv = crate::Reg<pfsrv::PfsrvSpec>;
#[doc = "desc PFSRV"]
pub mod pfsrv;
#[doc = "PDARV (rw) register accessor: desc PDARV\n\nYou can [`read`](crate::Reg::read) this register and get [`pdarv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdarv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdarv`] module"]
#[doc(alias = "PDARV")]
pub type Pdarv = crate::Reg<pdarv::PdarvSpec>;
#[doc = "desc PDARV"]
pub mod pdarv;
#[doc = "PDBRV (rw) register accessor: desc PDBRV\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbrv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbrv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbrv`] module"]
#[doc(alias = "PDBRV")]
pub type Pdbrv = crate::Reg<pdbrv::PdbrvSpec>;
#[doc = "desc PDBRV"]
pub mod pdbrv;
#[doc = "PFSRW (rw) register accessor: desc PFSRW\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrw`] module"]
#[doc(alias = "PFSRW")]
pub type Pfsrw = crate::Reg<pfsrw::PfsrwSpec>;
#[doc = "desc PFSRW"]
pub mod pfsrw;
#[doc = "PDARW (rw) register accessor: desc PDARW\n\nYou can [`read`](crate::Reg::read) this register and get [`pdarw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdarw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdarw`] module"]
#[doc(alias = "PDARW")]
pub type Pdarw = crate::Reg<pdarw::PdarwSpec>;
#[doc = "desc PDARW"]
pub mod pdarw;
#[doc = "PDBRW (rw) register accessor: desc PDBRW\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbrw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbrw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbrw`] module"]
#[doc(alias = "PDBRW")]
pub type Pdbrw = crate::Reg<pdbrw::PdbrwSpec>;
#[doc = "desc PDBRW"]
pub mod pdbrw;
#[doc = "POCRU (rw) register accessor: desc POCRU\n\nYou can [`read`](crate::Reg::read) this register and get [`pocru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pocru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pocru`] module"]
#[doc(alias = "POCRU")]
pub type Pocru = crate::Reg<pocru::PocruSpec>;
#[doc = "desc POCRU"]
pub mod pocru;
#[doc = "POCRV (rw) register accessor: desc POCRV\n\nYou can [`read`](crate::Reg::read) this register and get [`pocrv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pocrv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pocrv`] module"]
#[doc(alias = "POCRV")]
pub type Pocrv = crate::Reg<pocrv::PocrvSpec>;
#[doc = "desc POCRV"]
pub mod pocrv;
#[doc = "POCRW (rw) register accessor: desc POCRW\n\nYou can [`read`](crate::Reg::read) this register and get [`pocrw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pocrw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pocrw`] module"]
#[doc(alias = "POCRW")]
pub type Pocrw = crate::Reg<pocrw::PocrwSpec>;
#[doc = "desc POCRW"]
pub mod pocrw;
#[doc = "RCSR (rw) register accessor: desc RCSR\n\nYou can [`read`](crate::Reg::read) this register and get [`rcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcsr`] module"]
#[doc(alias = "RCSR")]
pub type Rcsr = crate::Reg<rcsr::RcsrSpec>;
#[doc = "desc RCSR"]
pub mod rcsr;
#[doc = "SCCRUH (rw) register accessor: desc SCCRUH\n\nYou can [`read`](crate::Reg::read) this register and get [`sccruh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccruh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccruh`] module"]
#[doc(alias = "SCCRUH")]
pub type Sccruh = crate::Reg<sccruh::SccruhSpec>;
#[doc = "desc SCCRUH"]
pub mod sccruh;
#[doc = "SCCRUL (rw) register accessor: desc SCCRUL\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrul::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrul::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccrul`] module"]
#[doc(alias = "SCCRUL")]
pub type Sccrul = crate::Reg<sccrul::SccrulSpec>;
#[doc = "desc SCCRUL"]
pub mod sccrul;
#[doc = "SCCRVH (rw) register accessor: desc SCCRVH\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrvh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrvh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccrvh`] module"]
#[doc(alias = "SCCRVH")]
pub type Sccrvh = crate::Reg<sccrvh::SccrvhSpec>;
#[doc = "desc SCCRVH"]
pub mod sccrvh;
#[doc = "SCCRVL (rw) register accessor: desc SCCRVL\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccrvl`] module"]
#[doc(alias = "SCCRVL")]
pub type Sccrvl = crate::Reg<sccrvl::SccrvlSpec>;
#[doc = "desc SCCRVL"]
pub mod sccrvl;
#[doc = "SCCRWH (rw) register accessor: desc SCCRWH\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrwh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrwh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccrwh`] module"]
#[doc(alias = "SCCRWH")]
pub type Sccrwh = crate::Reg<sccrwh::SccrwhSpec>;
#[doc = "desc SCCRWH"]
pub mod sccrwh;
#[doc = "SCCRWL (rw) register accessor: desc SCCRWL\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrwl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrwl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccrwl`] module"]
#[doc(alias = "SCCRWL")]
pub type Sccrwl = crate::Reg<sccrwl::SccrwlSpec>;
#[doc = "desc SCCRWL"]
pub mod sccrwl;
#[doc = "SCSRUH (rw) register accessor: desc SCSRUH\n\nYou can [`read`](crate::Reg::read) this register and get [`scsruh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsruh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsruh`] module"]
#[doc(alias = "SCSRUH")]
pub type Scsruh = crate::Reg<scsruh::ScsruhSpec>;
#[doc = "desc SCSRUH"]
pub mod scsruh;
#[doc = "SCMRUH (rw) register accessor: desc SCMRUH\n\nYou can [`read`](crate::Reg::read) this register and get [`scmruh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmruh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmruh`] module"]
#[doc(alias = "SCMRUH")]
pub type Scmruh = crate::Reg<scmruh::ScmruhSpec>;
#[doc = "desc SCMRUH"]
pub mod scmruh;
#[doc = "SCSRUL (rw) register accessor: desc SCSRUL\n\nYou can [`read`](crate::Reg::read) this register and get [`scsrul::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsrul::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsrul`] module"]
#[doc(alias = "SCSRUL")]
pub type Scsrul = crate::Reg<scsrul::ScsrulSpec>;
#[doc = "desc SCSRUL"]
pub mod scsrul;
#[doc = "SCMRUL (rw) register accessor: desc SCMRUL\n\nYou can [`read`](crate::Reg::read) this register and get [`scmrul::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmrul::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmrul`] module"]
#[doc(alias = "SCMRUL")]
pub type Scmrul = crate::Reg<scmrul::ScmrulSpec>;
#[doc = "desc SCMRUL"]
pub mod scmrul;
#[doc = "SCSRVH (rw) register accessor: desc SCSRVH\n\nYou can [`read`](crate::Reg::read) this register and get [`scsrvh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsrvh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsrvh`] module"]
#[doc(alias = "SCSRVH")]
pub type Scsrvh = crate::Reg<scsrvh::ScsrvhSpec>;
#[doc = "desc SCSRVH"]
pub mod scsrvh;
#[doc = "SCMRVH (rw) register accessor: desc SCMRVH\n\nYou can [`read`](crate::Reg::read) this register and get [`scmrvh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmrvh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmrvh`] module"]
#[doc(alias = "SCMRVH")]
pub type Scmrvh = crate::Reg<scmrvh::ScmrvhSpec>;
#[doc = "desc SCMRVH"]
pub mod scmrvh;
#[doc = "SCSRVL (rw) register accessor: desc SCSRVL\n\nYou can [`read`](crate::Reg::read) this register and get [`scsrvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsrvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsrvl`] module"]
#[doc(alias = "SCSRVL")]
pub type Scsrvl = crate::Reg<scsrvl::ScsrvlSpec>;
#[doc = "desc SCSRVL"]
pub mod scsrvl;
#[doc = "SCMRVL (rw) register accessor: desc SCMRVL\n\nYou can [`read`](crate::Reg::read) this register and get [`scmrvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmrvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmrvl`] module"]
#[doc(alias = "SCMRVL")]
pub type Scmrvl = crate::Reg<scmrvl::ScmrvlSpec>;
#[doc = "desc SCMRVL"]
pub mod scmrvl;
#[doc = "SCSRWH (rw) register accessor: desc SCSRWH\n\nYou can [`read`](crate::Reg::read) this register and get [`scsrwh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsrwh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsrwh`] module"]
#[doc(alias = "SCSRWH")]
pub type Scsrwh = crate::Reg<scsrwh::ScsrwhSpec>;
#[doc = "desc SCSRWH"]
pub mod scsrwh;
#[doc = "SCMRWH (rw) register accessor: desc SCMRWH\n\nYou can [`read`](crate::Reg::read) this register and get [`scmrwh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmrwh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmrwh`] module"]
#[doc(alias = "SCMRWH")]
pub type Scmrwh = crate::Reg<scmrwh::ScmrwhSpec>;
#[doc = "desc SCMRWH"]
pub mod scmrwh;
#[doc = "SCSRWL (rw) register accessor: desc SCSRWL\n\nYou can [`read`](crate::Reg::read) this register and get [`scsrwl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsrwl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsrwl`] module"]
#[doc(alias = "SCSRWL")]
pub type Scsrwl = crate::Reg<scsrwl::ScsrwlSpec>;
#[doc = "desc SCSRWL"]
pub mod scsrwl;
#[doc = "SCMRWL (rw) register accessor: desc SCMRWL\n\nYou can [`read`](crate::Reg::read) this register and get [`scmrwl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmrwl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmrwl`] module"]
#[doc(alias = "SCMRWL")]
pub type Scmrwl = crate::Reg<scmrwl::ScmrwlSpec>;
#[doc = "desc SCMRWL"]
pub mod scmrwl;
#[doc = "ECSR (rw) register accessor: desc ECSR\n\nYou can [`read`](crate::Reg::read) this register and get [`ecsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecsr`] module"]
#[doc(alias = "ECSR")]
pub type Ecsr = crate::Reg<ecsr::EcsrSpec>;
#[doc = "desc ECSR"]
pub mod ecsr;
