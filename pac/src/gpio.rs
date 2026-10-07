#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pidra: Pidra,
    _reserved1: [u8; 0x02],
    podra: Podra,
    poera: Poera,
    posra: Posra,
    porra: Porra,
    potra: Potra,
    _reserved6: [u8; 0x02],
    pidrb: Pidrb,
    _reserved7: [u8; 0x02],
    podrb: Podrb,
    poerb: Poerb,
    posrb: Posrb,
    porrb: Porrb,
    potrb: Potrb,
    _reserved12: [u8; 0x02],
    pidrc: Pidrc,
    _reserved13: [u8; 0x02],
    podrc: Podrc,
    poerc: Poerc,
    posrc: Posrc,
    porrc: Porrc,
    potrc: Potrc,
    _reserved18: [u8; 0x22],
    pidrh: Pidrh,
    _reserved19: [u8; 0x02],
    podrh: Podrh,
    poerh: Poerh,
    posrh: Posrh,
    porrh: Porrh,
    potrh: Potrh,
    _reserved24: [u8; 0x0396],
    pspcr: Pspcr,
    _reserved25: [u8; 0x02],
    pccr: Pccr,
    pinaer: Pinaer,
    pwpr: Pwpr,
    _reserved28: [u8; 0x02],
    pcra0: Pcra0,
    pfsra0: Pfsra0,
    pcra1: Pcra1,
    pfsra1: Pfsra1,
    pcra2: Pcra2,
    pfsra2: Pfsra2,
    pcra3: Pcra3,
    pfsra3: Pfsra3,
    pcra4: Pcra4,
    pfsra4: Pfsra4,
    pcra5: Pcra5,
    pfsra5: Pfsra5,
    pcra6: Pcra6,
    pfsra6: Pfsra6,
    pcra7: Pcra7,
    pfsra7: Pfsra7,
    pcra8: Pcra8,
    pfsra8: Pfsra8,
    pcra9: Pcra9,
    pfsra9: Pfsra9,
    pcra10: Pcra10,
    pfsra10: Pfsra10,
    pcra11: Pcra11,
    pfsra11: Pfsra11,
    pcra12: Pcra12,
    pfsra12: Pfsra12,
    pcra13: Pcra13,
    pfsra13: Pfsra13,
    pcra14: Pcra14,
    pfsra14: Pfsra14,
    pcra15: Pcra15,
    pfsra15: Pfsra15,
    pcrb0: Pcrb0,
    pfsrb0: Pfsrb0,
    pcrb1: Pcrb1,
    pfsrb1: Pfsrb1,
    pcrb2: Pcrb2,
    pfsrb2: Pfsrb2,
    pcrb3: Pcrb3,
    pfsrb3: Pfsrb3,
    pcrb4: Pcrb4,
    pfsrb4: Pfsrb4,
    pcrb5: Pcrb5,
    pfsrb5: Pfsrb5,
    pcrb6: Pcrb6,
    pfsrb6: Pfsrb6,
    pcrb7: Pcrb7,
    pfsrb7: Pfsrb7,
    pcrb8: Pcrb8,
    pfsrb8: Pfsrb8,
    pcrb9: Pcrb9,
    pfsrb9: Pfsrb9,
    pcrb10: Pcrb10,
    pfsrb10: Pfsrb10,
    pcrb11: Pcrb11,
    pfsrb11: Pfsrb11,
    pcrb12: Pcrb12,
    pfsrb12: Pfsrb12,
    pcrb13: Pcrb13,
    pfsrb13: Pfsrb13,
    pcrb14: Pcrb14,
    pfsrb14: Pfsrb14,
    pcrb15: Pcrb15,
    pfsrb15: Pfsrb15,
    _reserved92: [u8; 0x34],
    pcrc13: Pcrc13,
    pfsrc13: Pfsrc13,
    pcrc14: Pcrc14,
    pfsrc14: Pfsrc14,
    pcrc15: Pcrc15,
    pfsrc15: Pfsrc15,
    _reserved98: [u8; 0x80],
    pcrh0: Pcrh0,
    pfsrh0: Pfsrh0,
    pcrh1: Pcrh1,
    pfsrh1: Pfsrh1,
    pcrh2: Pcrh2,
    pfsrh2: Pfsrh2,
}
impl RegisterBlock {
    #[doc = "0x00 - desc PIDRA"]
    #[inline(always)]
    pub const fn pidra(&self) -> &Pidra {
        &self.pidra
    }
    #[doc = "0x04 - desc PODRA"]
    #[inline(always)]
    pub const fn podra(&self) -> &Podra {
        &self.podra
    }
    #[doc = "0x06 - desc POERA"]
    #[inline(always)]
    pub const fn poera(&self) -> &Poera {
        &self.poera
    }
    #[doc = "0x08 - desc POSRA"]
    #[inline(always)]
    pub const fn posra(&self) -> &Posra {
        &self.posra
    }
    #[doc = "0x0a - desc PORRA"]
    #[inline(always)]
    pub const fn porra(&self) -> &Porra {
        &self.porra
    }
    #[doc = "0x0c - desc POTRA"]
    #[inline(always)]
    pub const fn potra(&self) -> &Potra {
        &self.potra
    }
    #[doc = "0x10 - desc PIDRB"]
    #[inline(always)]
    pub const fn pidrb(&self) -> &Pidrb {
        &self.pidrb
    }
    #[doc = "0x14 - desc PODRB"]
    #[inline(always)]
    pub const fn podrb(&self) -> &Podrb {
        &self.podrb
    }
    #[doc = "0x16 - desc POERB"]
    #[inline(always)]
    pub const fn poerb(&self) -> &Poerb {
        &self.poerb
    }
    #[doc = "0x18 - desc POSRB"]
    #[inline(always)]
    pub const fn posrb(&self) -> &Posrb {
        &self.posrb
    }
    #[doc = "0x1a - desc PORRB"]
    #[inline(always)]
    pub const fn porrb(&self) -> &Porrb {
        &self.porrb
    }
    #[doc = "0x1c - desc POTRB"]
    #[inline(always)]
    pub const fn potrb(&self) -> &Potrb {
        &self.potrb
    }
    #[doc = "0x20 - desc PIDRC"]
    #[inline(always)]
    pub const fn pidrc(&self) -> &Pidrc {
        &self.pidrc
    }
    #[doc = "0x24 - desc PODRC"]
    #[inline(always)]
    pub const fn podrc(&self) -> &Podrc {
        &self.podrc
    }
    #[doc = "0x26 - desc POERC"]
    #[inline(always)]
    pub const fn poerc(&self) -> &Poerc {
        &self.poerc
    }
    #[doc = "0x28 - desc POSRC"]
    #[inline(always)]
    pub const fn posrc(&self) -> &Posrc {
        &self.posrc
    }
    #[doc = "0x2a - desc PORRC"]
    #[inline(always)]
    pub const fn porrc(&self) -> &Porrc {
        &self.porrc
    }
    #[doc = "0x2c - desc POTRC"]
    #[inline(always)]
    pub const fn potrc(&self) -> &Potrc {
        &self.potrc
    }
    #[doc = "0x50 - desc PIDRH"]
    #[inline(always)]
    pub const fn pidrh(&self) -> &Pidrh {
        &self.pidrh
    }
    #[doc = "0x54 - desc PODRH"]
    #[inline(always)]
    pub const fn podrh(&self) -> &Podrh {
        &self.podrh
    }
    #[doc = "0x56 - desc POERH"]
    #[inline(always)]
    pub const fn poerh(&self) -> &Poerh {
        &self.poerh
    }
    #[doc = "0x58 - desc POSRH"]
    #[inline(always)]
    pub const fn posrh(&self) -> &Posrh {
        &self.posrh
    }
    #[doc = "0x5a - desc PORRH"]
    #[inline(always)]
    pub const fn porrh(&self) -> &Porrh {
        &self.porrh
    }
    #[doc = "0x5c - desc POTRH"]
    #[inline(always)]
    pub const fn potrh(&self) -> &Potrh {
        &self.potrh
    }
    #[doc = "0x3f4 - desc PSPCR"]
    #[inline(always)]
    pub const fn pspcr(&self) -> &Pspcr {
        &self.pspcr
    }
    #[doc = "0x3f8 - desc PCCR"]
    #[inline(always)]
    pub const fn pccr(&self) -> &Pccr {
        &self.pccr
    }
    #[doc = "0x3fa - desc PINAER"]
    #[inline(always)]
    pub const fn pinaer(&self) -> &Pinaer {
        &self.pinaer
    }
    #[doc = "0x3fc - desc PWPR"]
    #[inline(always)]
    pub const fn pwpr(&self) -> &Pwpr {
        &self.pwpr
    }
    #[doc = "0x400 - desc PCRA0"]
    #[inline(always)]
    pub const fn pcra0(&self) -> &Pcra0 {
        &self.pcra0
    }
    #[doc = "0x402 - desc PFSRA0"]
    #[inline(always)]
    pub const fn pfsra0(&self) -> &Pfsra0 {
        &self.pfsra0
    }
    #[doc = "0x404 - desc PCRA1"]
    #[inline(always)]
    pub const fn pcra1(&self) -> &Pcra1 {
        &self.pcra1
    }
    #[doc = "0x406 - desc PFSRA1"]
    #[inline(always)]
    pub const fn pfsra1(&self) -> &Pfsra1 {
        &self.pfsra1
    }
    #[doc = "0x408 - desc PCRA2"]
    #[inline(always)]
    pub const fn pcra2(&self) -> &Pcra2 {
        &self.pcra2
    }
    #[doc = "0x40a - desc PFSRA2"]
    #[inline(always)]
    pub const fn pfsra2(&self) -> &Pfsra2 {
        &self.pfsra2
    }
    #[doc = "0x40c - desc PCRA3"]
    #[inline(always)]
    pub const fn pcra3(&self) -> &Pcra3 {
        &self.pcra3
    }
    #[doc = "0x40e - desc PFSRA3"]
    #[inline(always)]
    pub const fn pfsra3(&self) -> &Pfsra3 {
        &self.pfsra3
    }
    #[doc = "0x410 - desc PCRA4"]
    #[inline(always)]
    pub const fn pcra4(&self) -> &Pcra4 {
        &self.pcra4
    }
    #[doc = "0x412 - desc PFSRA4"]
    #[inline(always)]
    pub const fn pfsra4(&self) -> &Pfsra4 {
        &self.pfsra4
    }
    #[doc = "0x414 - desc PCRA5"]
    #[inline(always)]
    pub const fn pcra5(&self) -> &Pcra5 {
        &self.pcra5
    }
    #[doc = "0x416 - desc PFSRA5"]
    #[inline(always)]
    pub const fn pfsra5(&self) -> &Pfsra5 {
        &self.pfsra5
    }
    #[doc = "0x418 - desc PCRA6"]
    #[inline(always)]
    pub const fn pcra6(&self) -> &Pcra6 {
        &self.pcra6
    }
    #[doc = "0x41a - desc PFSRA6"]
    #[inline(always)]
    pub const fn pfsra6(&self) -> &Pfsra6 {
        &self.pfsra6
    }
    #[doc = "0x41c - desc PCRA7"]
    #[inline(always)]
    pub const fn pcra7(&self) -> &Pcra7 {
        &self.pcra7
    }
    #[doc = "0x41e - desc PFSRA7"]
    #[inline(always)]
    pub const fn pfsra7(&self) -> &Pfsra7 {
        &self.pfsra7
    }
    #[doc = "0x420 - desc PCRA8"]
    #[inline(always)]
    pub const fn pcra8(&self) -> &Pcra8 {
        &self.pcra8
    }
    #[doc = "0x422 - desc PFSRA8"]
    #[inline(always)]
    pub const fn pfsra8(&self) -> &Pfsra8 {
        &self.pfsra8
    }
    #[doc = "0x424 - desc PCRA9"]
    #[inline(always)]
    pub const fn pcra9(&self) -> &Pcra9 {
        &self.pcra9
    }
    #[doc = "0x426 - desc PFSRA9"]
    #[inline(always)]
    pub const fn pfsra9(&self) -> &Pfsra9 {
        &self.pfsra9
    }
    #[doc = "0x428 - desc PCRA10"]
    #[inline(always)]
    pub const fn pcra10(&self) -> &Pcra10 {
        &self.pcra10
    }
    #[doc = "0x42a - desc PFSRA10"]
    #[inline(always)]
    pub const fn pfsra10(&self) -> &Pfsra10 {
        &self.pfsra10
    }
    #[doc = "0x42c - desc PCRA11"]
    #[inline(always)]
    pub const fn pcra11(&self) -> &Pcra11 {
        &self.pcra11
    }
    #[doc = "0x42e - desc PFSRA11"]
    #[inline(always)]
    pub const fn pfsra11(&self) -> &Pfsra11 {
        &self.pfsra11
    }
    #[doc = "0x430 - desc PCRA12"]
    #[inline(always)]
    pub const fn pcra12(&self) -> &Pcra12 {
        &self.pcra12
    }
    #[doc = "0x432 - desc PFSRA12"]
    #[inline(always)]
    pub const fn pfsra12(&self) -> &Pfsra12 {
        &self.pfsra12
    }
    #[doc = "0x434 - desc PCRA13"]
    #[inline(always)]
    pub const fn pcra13(&self) -> &Pcra13 {
        &self.pcra13
    }
    #[doc = "0x436 - desc PFSRA13"]
    #[inline(always)]
    pub const fn pfsra13(&self) -> &Pfsra13 {
        &self.pfsra13
    }
    #[doc = "0x438 - desc PCRA14"]
    #[inline(always)]
    pub const fn pcra14(&self) -> &Pcra14 {
        &self.pcra14
    }
    #[doc = "0x43a - desc PFSRA14"]
    #[inline(always)]
    pub const fn pfsra14(&self) -> &Pfsra14 {
        &self.pfsra14
    }
    #[doc = "0x43c - desc PCRA15"]
    #[inline(always)]
    pub const fn pcra15(&self) -> &Pcra15 {
        &self.pcra15
    }
    #[doc = "0x43e - desc PFSRA15"]
    #[inline(always)]
    pub const fn pfsra15(&self) -> &Pfsra15 {
        &self.pfsra15
    }
    #[doc = "0x440 - desc PCRB0"]
    #[inline(always)]
    pub const fn pcrb0(&self) -> &Pcrb0 {
        &self.pcrb0
    }
    #[doc = "0x442 - desc PFSRB0"]
    #[inline(always)]
    pub const fn pfsrb0(&self) -> &Pfsrb0 {
        &self.pfsrb0
    }
    #[doc = "0x444 - desc PCRB1"]
    #[inline(always)]
    pub const fn pcrb1(&self) -> &Pcrb1 {
        &self.pcrb1
    }
    #[doc = "0x446 - desc PFSRB1"]
    #[inline(always)]
    pub const fn pfsrb1(&self) -> &Pfsrb1 {
        &self.pfsrb1
    }
    #[doc = "0x448 - desc PCRB2"]
    #[inline(always)]
    pub const fn pcrb2(&self) -> &Pcrb2 {
        &self.pcrb2
    }
    #[doc = "0x44a - desc PFSRB2"]
    #[inline(always)]
    pub const fn pfsrb2(&self) -> &Pfsrb2 {
        &self.pfsrb2
    }
    #[doc = "0x44c - desc PCRB3"]
    #[inline(always)]
    pub const fn pcrb3(&self) -> &Pcrb3 {
        &self.pcrb3
    }
    #[doc = "0x44e - desc PFSRB3"]
    #[inline(always)]
    pub const fn pfsrb3(&self) -> &Pfsrb3 {
        &self.pfsrb3
    }
    #[doc = "0x450 - desc PCRB4"]
    #[inline(always)]
    pub const fn pcrb4(&self) -> &Pcrb4 {
        &self.pcrb4
    }
    #[doc = "0x452 - desc PFSRB4"]
    #[inline(always)]
    pub const fn pfsrb4(&self) -> &Pfsrb4 {
        &self.pfsrb4
    }
    #[doc = "0x454 - desc PCRB5"]
    #[inline(always)]
    pub const fn pcrb5(&self) -> &Pcrb5 {
        &self.pcrb5
    }
    #[doc = "0x456 - desc PFSRB5"]
    #[inline(always)]
    pub const fn pfsrb5(&self) -> &Pfsrb5 {
        &self.pfsrb5
    }
    #[doc = "0x458 - desc PCRB6"]
    #[inline(always)]
    pub const fn pcrb6(&self) -> &Pcrb6 {
        &self.pcrb6
    }
    #[doc = "0x45a - desc PFSRB6"]
    #[inline(always)]
    pub const fn pfsrb6(&self) -> &Pfsrb6 {
        &self.pfsrb6
    }
    #[doc = "0x45c - desc PCRB7"]
    #[inline(always)]
    pub const fn pcrb7(&self) -> &Pcrb7 {
        &self.pcrb7
    }
    #[doc = "0x45e - desc PFSRB7"]
    #[inline(always)]
    pub const fn pfsrb7(&self) -> &Pfsrb7 {
        &self.pfsrb7
    }
    #[doc = "0x460 - desc PCRB8"]
    #[inline(always)]
    pub const fn pcrb8(&self) -> &Pcrb8 {
        &self.pcrb8
    }
    #[doc = "0x462 - desc PFSRB8"]
    #[inline(always)]
    pub const fn pfsrb8(&self) -> &Pfsrb8 {
        &self.pfsrb8
    }
    #[doc = "0x464 - desc PCRB9"]
    #[inline(always)]
    pub const fn pcrb9(&self) -> &Pcrb9 {
        &self.pcrb9
    }
    #[doc = "0x466 - desc PFSRB9"]
    #[inline(always)]
    pub const fn pfsrb9(&self) -> &Pfsrb9 {
        &self.pfsrb9
    }
    #[doc = "0x468 - desc PCRB10"]
    #[inline(always)]
    pub const fn pcrb10(&self) -> &Pcrb10 {
        &self.pcrb10
    }
    #[doc = "0x46a - desc PFSRB10"]
    #[inline(always)]
    pub const fn pfsrb10(&self) -> &Pfsrb10 {
        &self.pfsrb10
    }
    #[doc = "0x46c - desc PCRB11"]
    #[inline(always)]
    pub const fn pcrb11(&self) -> &Pcrb11 {
        &self.pcrb11
    }
    #[doc = "0x46e - desc PFSRB11"]
    #[inline(always)]
    pub const fn pfsrb11(&self) -> &Pfsrb11 {
        &self.pfsrb11
    }
    #[doc = "0x470 - desc PCRB12"]
    #[inline(always)]
    pub const fn pcrb12(&self) -> &Pcrb12 {
        &self.pcrb12
    }
    #[doc = "0x472 - desc PFSRB12"]
    #[inline(always)]
    pub const fn pfsrb12(&self) -> &Pfsrb12 {
        &self.pfsrb12
    }
    #[doc = "0x474 - desc PCRB13"]
    #[inline(always)]
    pub const fn pcrb13(&self) -> &Pcrb13 {
        &self.pcrb13
    }
    #[doc = "0x476 - desc PFSRB13"]
    #[inline(always)]
    pub const fn pfsrb13(&self) -> &Pfsrb13 {
        &self.pfsrb13
    }
    #[doc = "0x478 - desc PCRB14"]
    #[inline(always)]
    pub const fn pcrb14(&self) -> &Pcrb14 {
        &self.pcrb14
    }
    #[doc = "0x47a - desc PFSRB14"]
    #[inline(always)]
    pub const fn pfsrb14(&self) -> &Pfsrb14 {
        &self.pfsrb14
    }
    #[doc = "0x47c - desc PCRB15"]
    #[inline(always)]
    pub const fn pcrb15(&self) -> &Pcrb15 {
        &self.pcrb15
    }
    #[doc = "0x47e - desc PFSRB15"]
    #[inline(always)]
    pub const fn pfsrb15(&self) -> &Pfsrb15 {
        &self.pfsrb15
    }
    #[doc = "0x4b4 - desc PCRC13"]
    #[inline(always)]
    pub const fn pcrc13(&self) -> &Pcrc13 {
        &self.pcrc13
    }
    #[doc = "0x4b6 - desc PFSRC13"]
    #[inline(always)]
    pub const fn pfsrc13(&self) -> &Pfsrc13 {
        &self.pfsrc13
    }
    #[doc = "0x4b8 - desc PCRC14"]
    #[inline(always)]
    pub const fn pcrc14(&self) -> &Pcrc14 {
        &self.pcrc14
    }
    #[doc = "0x4ba - desc PFSRC14"]
    #[inline(always)]
    pub const fn pfsrc14(&self) -> &Pfsrc14 {
        &self.pfsrc14
    }
    #[doc = "0x4bc - desc PCRC15"]
    #[inline(always)]
    pub const fn pcrc15(&self) -> &Pcrc15 {
        &self.pcrc15
    }
    #[doc = "0x4be - desc PFSRC15"]
    #[inline(always)]
    pub const fn pfsrc15(&self) -> &Pfsrc15 {
        &self.pfsrc15
    }
    #[doc = "0x540 - desc PCRH0"]
    #[inline(always)]
    pub const fn pcrh0(&self) -> &Pcrh0 {
        &self.pcrh0
    }
    #[doc = "0x542 - desc PFSRH0"]
    #[inline(always)]
    pub const fn pfsrh0(&self) -> &Pfsrh0 {
        &self.pfsrh0
    }
    #[doc = "0x544 - desc PCRH1"]
    #[inline(always)]
    pub const fn pcrh1(&self) -> &Pcrh1 {
        &self.pcrh1
    }
    #[doc = "0x546 - desc PFSRH1"]
    #[inline(always)]
    pub const fn pfsrh1(&self) -> &Pfsrh1 {
        &self.pfsrh1
    }
    #[doc = "0x548 - desc PCRH2"]
    #[inline(always)]
    pub const fn pcrh2(&self) -> &Pcrh2 {
        &self.pcrh2
    }
    #[doc = "0x54a - desc PFSRH2"]
    #[inline(always)]
    pub const fn pfsrh2(&self) -> &Pfsrh2 {
        &self.pfsrh2
    }
}
#[doc = "PIDRA (r) register accessor: desc PIDRA\n\nYou can [`read`](crate::Reg::read) this register and get [`pidra::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidra`] module"]
#[doc(alias = "PIDRA")]
pub type Pidra = crate::Reg<pidra::PidraSpec>;
#[doc = "desc PIDRA"]
pub mod pidra;
#[doc = "PODRA (rw) register accessor: desc PODRA\n\nYou can [`read`](crate::Reg::read) this register and get [`podra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`podra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@podra`] module"]
#[doc(alias = "PODRA")]
pub type Podra = crate::Reg<podra::PodraSpec>;
#[doc = "desc PODRA"]
pub mod podra;
#[doc = "POERA (rw) register accessor: desc POERA\n\nYou can [`read`](crate::Reg::read) this register and get [`poera::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poera::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@poera`] module"]
#[doc(alias = "POERA")]
pub type Poera = crate::Reg<poera::PoeraSpec>;
#[doc = "desc POERA"]
pub mod poera;
#[doc = "POSRA (rw) register accessor: desc POSRA\n\nYou can [`read`](crate::Reg::read) this register and get [`posra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`posra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@posra`] module"]
#[doc(alias = "POSRA")]
pub type Posra = crate::Reg<posra::PosraSpec>;
#[doc = "desc POSRA"]
pub mod posra;
#[doc = "PORRA (rw) register accessor: desc PORRA\n\nYou can [`read`](crate::Reg::read) this register and get [`porra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porra`] module"]
#[doc(alias = "PORRA")]
pub type Porra = crate::Reg<porra::PorraSpec>;
#[doc = "desc PORRA"]
pub mod porra;
#[doc = "POTRA (rw) register accessor: desc POTRA\n\nYou can [`read`](crate::Reg::read) this register and get [`potra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`potra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@potra`] module"]
#[doc(alias = "POTRA")]
pub type Potra = crate::Reg<potra::PotraSpec>;
#[doc = "desc POTRA"]
pub mod potra;
#[doc = "PIDRB (r) register accessor: desc PIDRB\n\nYou can [`read`](crate::Reg::read) this register and get [`pidrb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidrb`] module"]
#[doc(alias = "PIDRB")]
pub type Pidrb = crate::Reg<pidrb::PidrbSpec>;
#[doc = "desc PIDRB"]
pub mod pidrb;
#[doc = "PODRB (rw) register accessor: desc PODRB\n\nYou can [`read`](crate::Reg::read) this register and get [`podrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`podrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@podrb`] module"]
#[doc(alias = "PODRB")]
pub type Podrb = crate::Reg<podrb::PodrbSpec>;
#[doc = "desc PODRB"]
pub mod podrb;
#[doc = "POERB (rw) register accessor: desc POERB\n\nYou can [`read`](crate::Reg::read) this register and get [`poerb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poerb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@poerb`] module"]
#[doc(alias = "POERB")]
pub type Poerb = crate::Reg<poerb::PoerbSpec>;
#[doc = "desc POERB"]
pub mod poerb;
#[doc = "POSRB (rw) register accessor: desc POSRB\n\nYou can [`read`](crate::Reg::read) this register and get [`posrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`posrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@posrb`] module"]
#[doc(alias = "POSRB")]
pub type Posrb = crate::Reg<posrb::PosrbSpec>;
#[doc = "desc POSRB"]
pub mod posrb;
#[doc = "PORRB (rw) register accessor: desc PORRB\n\nYou can [`read`](crate::Reg::read) this register and get [`porrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porrb`] module"]
#[doc(alias = "PORRB")]
pub type Porrb = crate::Reg<porrb::PorrbSpec>;
#[doc = "desc PORRB"]
pub mod porrb;
#[doc = "POTRB (rw) register accessor: desc POTRB\n\nYou can [`read`](crate::Reg::read) this register and get [`potrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`potrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@potrb`] module"]
#[doc(alias = "POTRB")]
pub type Potrb = crate::Reg<potrb::PotrbSpec>;
#[doc = "desc POTRB"]
pub mod potrb;
#[doc = "PIDRC (r) register accessor: desc PIDRC\n\nYou can [`read`](crate::Reg::read) this register and get [`pidrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidrc`] module"]
#[doc(alias = "PIDRC")]
pub type Pidrc = crate::Reg<pidrc::PidrcSpec>;
#[doc = "desc PIDRC"]
pub mod pidrc;
#[doc = "PODRC (rw) register accessor: desc PODRC\n\nYou can [`read`](crate::Reg::read) this register and get [`podrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`podrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@podrc`] module"]
#[doc(alias = "PODRC")]
pub type Podrc = crate::Reg<podrc::PodrcSpec>;
#[doc = "desc PODRC"]
pub mod podrc;
#[doc = "POERC (rw) register accessor: desc POERC\n\nYou can [`read`](crate::Reg::read) this register and get [`poerc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poerc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@poerc`] module"]
#[doc(alias = "POERC")]
pub type Poerc = crate::Reg<poerc::PoercSpec>;
#[doc = "desc POERC"]
pub mod poerc;
#[doc = "POSRC (rw) register accessor: desc POSRC\n\nYou can [`read`](crate::Reg::read) this register and get [`posrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`posrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@posrc`] module"]
#[doc(alias = "POSRC")]
pub type Posrc = crate::Reg<posrc::PosrcSpec>;
#[doc = "desc POSRC"]
pub mod posrc;
#[doc = "PORRC (rw) register accessor: desc PORRC\n\nYou can [`read`](crate::Reg::read) this register and get [`porrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porrc`] module"]
#[doc(alias = "PORRC")]
pub type Porrc = crate::Reg<porrc::PorrcSpec>;
#[doc = "desc PORRC"]
pub mod porrc;
#[doc = "POTRC (rw) register accessor: desc POTRC\n\nYou can [`read`](crate::Reg::read) this register and get [`potrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`potrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@potrc`] module"]
#[doc(alias = "POTRC")]
pub type Potrc = crate::Reg<potrc::PotrcSpec>;
#[doc = "desc POTRC"]
pub mod potrc;
#[doc = "PIDRH (r) register accessor: desc PIDRH\n\nYou can [`read`](crate::Reg::read) this register and get [`pidrh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidrh`] module"]
#[doc(alias = "PIDRH")]
pub type Pidrh = crate::Reg<pidrh::PidrhSpec>;
#[doc = "desc PIDRH"]
pub mod pidrh;
#[doc = "PODRH (rw) register accessor: desc PODRH\n\nYou can [`read`](crate::Reg::read) this register and get [`podrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`podrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@podrh`] module"]
#[doc(alias = "PODRH")]
pub type Podrh = crate::Reg<podrh::PodrhSpec>;
#[doc = "desc PODRH"]
pub mod podrh;
#[doc = "POERH (rw) register accessor: desc POERH\n\nYou can [`read`](crate::Reg::read) this register and get [`poerh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poerh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@poerh`] module"]
#[doc(alias = "POERH")]
pub type Poerh = crate::Reg<poerh::PoerhSpec>;
#[doc = "desc POERH"]
pub mod poerh;
#[doc = "POSRH (rw) register accessor: desc POSRH\n\nYou can [`read`](crate::Reg::read) this register and get [`posrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`posrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@posrh`] module"]
#[doc(alias = "POSRH")]
pub type Posrh = crate::Reg<posrh::PosrhSpec>;
#[doc = "desc POSRH"]
pub mod posrh;
#[doc = "PORRH (rw) register accessor: desc PORRH\n\nYou can [`read`](crate::Reg::read) this register and get [`porrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porrh`] module"]
#[doc(alias = "PORRH")]
pub type Porrh = crate::Reg<porrh::PorrhSpec>;
#[doc = "desc PORRH"]
pub mod porrh;
#[doc = "POTRH (rw) register accessor: desc POTRH\n\nYou can [`read`](crate::Reg::read) this register and get [`potrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`potrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@potrh`] module"]
#[doc(alias = "POTRH")]
pub type Potrh = crate::Reg<potrh::PotrhSpec>;
#[doc = "desc POTRH"]
pub mod potrh;
#[doc = "PSPCR (rw) register accessor: desc PSPCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pspcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pspcr`] module"]
#[doc(alias = "PSPCR")]
pub type Pspcr = crate::Reg<pspcr::PspcrSpec>;
#[doc = "desc PSPCR"]
pub mod pspcr;
#[doc = "PCCR (rw) register accessor: desc PCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pccr`] module"]
#[doc(alias = "PCCR")]
pub type Pccr = crate::Reg<pccr::PccrSpec>;
#[doc = "desc PCCR"]
pub mod pccr;
#[doc = "PINAER (rw) register accessor: desc PINAER\n\nYou can [`read`](crate::Reg::read) this register and get [`pinaer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinaer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinaer`] module"]
#[doc(alias = "PINAER")]
pub type Pinaer = crate::Reg<pinaer::PinaerSpec>;
#[doc = "desc PINAER"]
pub mod pinaer;
#[doc = "PWPR (rw) register accessor: desc PWPR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwpr`] module"]
#[doc(alias = "PWPR")]
pub type Pwpr = crate::Reg<pwpr::PwprSpec>;
#[doc = "desc PWPR"]
pub mod pwpr;
#[doc = "PCRA0 (rw) register accessor: desc PCRA0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra0`] module"]
#[doc(alias = "PCRA0")]
pub type Pcra0 = crate::Reg<pcra0::Pcra0Spec>;
#[doc = "desc PCRA0"]
pub mod pcra0;
#[doc = "PFSRA0 (rw) register accessor: desc PFSRA0\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra0`] module"]
#[doc(alias = "PFSRA0")]
pub type Pfsra0 = crate::Reg<pfsra0::Pfsra0Spec>;
#[doc = "desc PFSRA0"]
pub mod pfsra0;
#[doc = "PCRA1 (rw) register accessor: desc PCRA1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra1`] module"]
#[doc(alias = "PCRA1")]
pub type Pcra1 = crate::Reg<pcra1::Pcra1Spec>;
#[doc = "desc PCRA1"]
pub mod pcra1;
#[doc = "PFSRA1 (rw) register accessor: desc PFSRA1\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra1`] module"]
#[doc(alias = "PFSRA1")]
pub type Pfsra1 = crate::Reg<pfsra1::Pfsra1Spec>;
#[doc = "desc PFSRA1"]
pub mod pfsra1;
#[doc = "PCRA2 (rw) register accessor: desc PCRA2\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra2`] module"]
#[doc(alias = "PCRA2")]
pub type Pcra2 = crate::Reg<pcra2::Pcra2Spec>;
#[doc = "desc PCRA2"]
pub mod pcra2;
#[doc = "PFSRA2 (rw) register accessor: desc PFSRA2\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra2`] module"]
#[doc(alias = "PFSRA2")]
pub type Pfsra2 = crate::Reg<pfsra2::Pfsra2Spec>;
#[doc = "desc PFSRA2"]
pub mod pfsra2;
#[doc = "PCRA3 (rw) register accessor: desc PCRA3\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra3`] module"]
#[doc(alias = "PCRA3")]
pub type Pcra3 = crate::Reg<pcra3::Pcra3Spec>;
#[doc = "desc PCRA3"]
pub mod pcra3;
#[doc = "PFSRA3 (rw) register accessor: desc PFSRA3\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra3`] module"]
#[doc(alias = "PFSRA3")]
pub type Pfsra3 = crate::Reg<pfsra3::Pfsra3Spec>;
#[doc = "desc PFSRA3"]
pub mod pfsra3;
#[doc = "PCRA4 (rw) register accessor: desc PCRA4\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra4`] module"]
#[doc(alias = "PCRA4")]
pub type Pcra4 = crate::Reg<pcra4::Pcra4Spec>;
#[doc = "desc PCRA4"]
pub mod pcra4;
#[doc = "PFSRA4 (rw) register accessor: desc PFSRA4\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra4`] module"]
#[doc(alias = "PFSRA4")]
pub type Pfsra4 = crate::Reg<pfsra4::Pfsra4Spec>;
#[doc = "desc PFSRA4"]
pub mod pfsra4;
#[doc = "PCRA5 (rw) register accessor: desc PCRA5\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra5`] module"]
#[doc(alias = "PCRA5")]
pub type Pcra5 = crate::Reg<pcra5::Pcra5Spec>;
#[doc = "desc PCRA5"]
pub mod pcra5;
#[doc = "PFSRA5 (rw) register accessor: desc PFSRA5\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra5`] module"]
#[doc(alias = "PFSRA5")]
pub type Pfsra5 = crate::Reg<pfsra5::Pfsra5Spec>;
#[doc = "desc PFSRA5"]
pub mod pfsra5;
#[doc = "PCRA6 (rw) register accessor: desc PCRA6\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra6`] module"]
#[doc(alias = "PCRA6")]
pub type Pcra6 = crate::Reg<pcra6::Pcra6Spec>;
#[doc = "desc PCRA6"]
pub mod pcra6;
#[doc = "PFSRA6 (rw) register accessor: desc PFSRA6\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra6`] module"]
#[doc(alias = "PFSRA6")]
pub type Pfsra6 = crate::Reg<pfsra6::Pfsra6Spec>;
#[doc = "desc PFSRA6"]
pub mod pfsra6;
#[doc = "PCRA7 (rw) register accessor: desc PCRA7\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra7`] module"]
#[doc(alias = "PCRA7")]
pub type Pcra7 = crate::Reg<pcra7::Pcra7Spec>;
#[doc = "desc PCRA7"]
pub mod pcra7;
#[doc = "PFSRA7 (rw) register accessor: desc PFSRA7\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra7`] module"]
#[doc(alias = "PFSRA7")]
pub type Pfsra7 = crate::Reg<pfsra7::Pfsra7Spec>;
#[doc = "desc PFSRA7"]
pub mod pfsra7;
#[doc = "PCRA8 (rw) register accessor: desc PCRA8\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra8`] module"]
#[doc(alias = "PCRA8")]
pub type Pcra8 = crate::Reg<pcra8::Pcra8Spec>;
#[doc = "desc PCRA8"]
pub mod pcra8;
#[doc = "PFSRA8 (rw) register accessor: desc PFSRA8\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra8`] module"]
#[doc(alias = "PFSRA8")]
pub type Pfsra8 = crate::Reg<pfsra8::Pfsra8Spec>;
#[doc = "desc PFSRA8"]
pub mod pfsra8;
#[doc = "PCRA9 (rw) register accessor: desc PCRA9\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra9`] module"]
#[doc(alias = "PCRA9")]
pub type Pcra9 = crate::Reg<pcra9::Pcra9Spec>;
#[doc = "desc PCRA9"]
pub mod pcra9;
#[doc = "PFSRA9 (rw) register accessor: desc PFSRA9\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra9`] module"]
#[doc(alias = "PFSRA9")]
pub type Pfsra9 = crate::Reg<pfsra9::Pfsra9Spec>;
#[doc = "desc PFSRA9"]
pub mod pfsra9;
#[doc = "PCRA10 (rw) register accessor: desc PCRA10\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra10`] module"]
#[doc(alias = "PCRA10")]
pub type Pcra10 = crate::Reg<pcra10::Pcra10Spec>;
#[doc = "desc PCRA10"]
pub mod pcra10;
#[doc = "PFSRA10 (rw) register accessor: desc PFSRA10\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra10`] module"]
#[doc(alias = "PFSRA10")]
pub type Pfsra10 = crate::Reg<pfsra10::Pfsra10Spec>;
#[doc = "desc PFSRA10"]
pub mod pfsra10;
#[doc = "PCRA11 (rw) register accessor: desc PCRA11\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra11`] module"]
#[doc(alias = "PCRA11")]
pub type Pcra11 = crate::Reg<pcra11::Pcra11Spec>;
#[doc = "desc PCRA11"]
pub mod pcra11;
#[doc = "PFSRA11 (rw) register accessor: desc PFSRA11\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra11`] module"]
#[doc(alias = "PFSRA11")]
pub type Pfsra11 = crate::Reg<pfsra11::Pfsra11Spec>;
#[doc = "desc PFSRA11"]
pub mod pfsra11;
#[doc = "PCRA12 (rw) register accessor: desc PCRA12\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra12`] module"]
#[doc(alias = "PCRA12")]
pub type Pcra12 = crate::Reg<pcra12::Pcra12Spec>;
#[doc = "desc PCRA12"]
pub mod pcra12;
#[doc = "PFSRA12 (rw) register accessor: desc PFSRA12\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra12`] module"]
#[doc(alias = "PFSRA12")]
pub type Pfsra12 = crate::Reg<pfsra12::Pfsra12Spec>;
#[doc = "desc PFSRA12"]
pub mod pfsra12;
#[doc = "PCRA13 (rw) register accessor: desc PCRA13\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra13`] module"]
#[doc(alias = "PCRA13")]
pub type Pcra13 = crate::Reg<pcra13::Pcra13Spec>;
#[doc = "desc PCRA13"]
pub mod pcra13;
#[doc = "PFSRA13 (rw) register accessor: desc PFSRA13\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra13`] module"]
#[doc(alias = "PFSRA13")]
pub type Pfsra13 = crate::Reg<pfsra13::Pfsra13Spec>;
#[doc = "desc PFSRA13"]
pub mod pfsra13;
#[doc = "PCRA14 (rw) register accessor: desc PCRA14\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra14`] module"]
#[doc(alias = "PCRA14")]
pub type Pcra14 = crate::Reg<pcra14::Pcra14Spec>;
#[doc = "desc PCRA14"]
pub mod pcra14;
#[doc = "PFSRA14 (rw) register accessor: desc PFSRA14\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra14`] module"]
#[doc(alias = "PFSRA14")]
pub type Pfsra14 = crate::Reg<pfsra14::Pfsra14Spec>;
#[doc = "desc PFSRA14"]
pub mod pfsra14;
#[doc = "PCRA15 (rw) register accessor: desc PCRA15\n\nYou can [`read`](crate::Reg::read) this register and get [`pcra15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcra15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcra15`] module"]
#[doc(alias = "PCRA15")]
pub type Pcra15 = crate::Reg<pcra15::Pcra15Spec>;
#[doc = "desc PCRA15"]
pub mod pcra15;
#[doc = "PFSRA15 (rw) register accessor: desc PFSRA15\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsra15`] module"]
#[doc(alias = "PFSRA15")]
pub type Pfsra15 = crate::Reg<pfsra15::Pfsra15Spec>;
#[doc = "desc PFSRA15"]
pub mod pfsra15;
#[doc = "PCRB0 (rw) register accessor: desc PCRB0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb0`] module"]
#[doc(alias = "PCRB0")]
pub type Pcrb0 = crate::Reg<pcrb0::Pcrb0Spec>;
#[doc = "desc PCRB0"]
pub mod pcrb0;
#[doc = "PFSRB0 (rw) register accessor: desc PFSRB0\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb0`] module"]
#[doc(alias = "PFSRB0")]
pub type Pfsrb0 = crate::Reg<pfsrb0::Pfsrb0Spec>;
#[doc = "desc PFSRB0"]
pub mod pfsrb0;
#[doc = "PCRB1 (rw) register accessor: desc PCRB1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb1`] module"]
#[doc(alias = "PCRB1")]
pub type Pcrb1 = crate::Reg<pcrb1::Pcrb1Spec>;
#[doc = "desc PCRB1"]
pub mod pcrb1;
#[doc = "PFSRB1 (rw) register accessor: desc PFSRB1\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb1`] module"]
#[doc(alias = "PFSRB1")]
pub type Pfsrb1 = crate::Reg<pfsrb1::Pfsrb1Spec>;
#[doc = "desc PFSRB1"]
pub mod pfsrb1;
#[doc = "PCRB2 (rw) register accessor: desc PCRB2\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb2`] module"]
#[doc(alias = "PCRB2")]
pub type Pcrb2 = crate::Reg<pcrb2::Pcrb2Spec>;
#[doc = "desc PCRB2"]
pub mod pcrb2;
#[doc = "PFSRB2 (rw) register accessor: desc PFSRB2\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb2`] module"]
#[doc(alias = "PFSRB2")]
pub type Pfsrb2 = crate::Reg<pfsrb2::Pfsrb2Spec>;
#[doc = "desc PFSRB2"]
pub mod pfsrb2;
#[doc = "PCRB3 (rw) register accessor: desc PCRB3\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb3`] module"]
#[doc(alias = "PCRB3")]
pub type Pcrb3 = crate::Reg<pcrb3::Pcrb3Spec>;
#[doc = "desc PCRB3"]
pub mod pcrb3;
#[doc = "PFSRB3 (rw) register accessor: desc PFSRB3\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb3`] module"]
#[doc(alias = "PFSRB3")]
pub type Pfsrb3 = crate::Reg<pfsrb3::Pfsrb3Spec>;
#[doc = "desc PFSRB3"]
pub mod pfsrb3;
#[doc = "PCRB4 (rw) register accessor: desc PCRB4\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb4`] module"]
#[doc(alias = "PCRB4")]
pub type Pcrb4 = crate::Reg<pcrb4::Pcrb4Spec>;
#[doc = "desc PCRB4"]
pub mod pcrb4;
#[doc = "PFSRB4 (rw) register accessor: desc PFSRB4\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb4`] module"]
#[doc(alias = "PFSRB4")]
pub type Pfsrb4 = crate::Reg<pfsrb4::Pfsrb4Spec>;
#[doc = "desc PFSRB4"]
pub mod pfsrb4;
#[doc = "PCRB5 (rw) register accessor: desc PCRB5\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb5`] module"]
#[doc(alias = "PCRB5")]
pub type Pcrb5 = crate::Reg<pcrb5::Pcrb5Spec>;
#[doc = "desc PCRB5"]
pub mod pcrb5;
#[doc = "PFSRB5 (rw) register accessor: desc PFSRB5\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb5`] module"]
#[doc(alias = "PFSRB5")]
pub type Pfsrb5 = crate::Reg<pfsrb5::Pfsrb5Spec>;
#[doc = "desc PFSRB5"]
pub mod pfsrb5;
#[doc = "PCRB6 (rw) register accessor: desc PCRB6\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb6`] module"]
#[doc(alias = "PCRB6")]
pub type Pcrb6 = crate::Reg<pcrb6::Pcrb6Spec>;
#[doc = "desc PCRB6"]
pub mod pcrb6;
#[doc = "PFSRB6 (rw) register accessor: desc PFSRB6\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb6`] module"]
#[doc(alias = "PFSRB6")]
pub type Pfsrb6 = crate::Reg<pfsrb6::Pfsrb6Spec>;
#[doc = "desc PFSRB6"]
pub mod pfsrb6;
#[doc = "PCRB7 (rw) register accessor: desc PCRB7\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb7`] module"]
#[doc(alias = "PCRB7")]
pub type Pcrb7 = crate::Reg<pcrb7::Pcrb7Spec>;
#[doc = "desc PCRB7"]
pub mod pcrb7;
#[doc = "PFSRB7 (rw) register accessor: desc PFSRB7\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb7`] module"]
#[doc(alias = "PFSRB7")]
pub type Pfsrb7 = crate::Reg<pfsrb7::Pfsrb7Spec>;
#[doc = "desc PFSRB7"]
pub mod pfsrb7;
#[doc = "PCRB8 (rw) register accessor: desc PCRB8\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb8`] module"]
#[doc(alias = "PCRB8")]
pub type Pcrb8 = crate::Reg<pcrb8::Pcrb8Spec>;
#[doc = "desc PCRB8"]
pub mod pcrb8;
#[doc = "PFSRB8 (rw) register accessor: desc PFSRB8\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb8`] module"]
#[doc(alias = "PFSRB8")]
pub type Pfsrb8 = crate::Reg<pfsrb8::Pfsrb8Spec>;
#[doc = "desc PFSRB8"]
pub mod pfsrb8;
#[doc = "PCRB9 (rw) register accessor: desc PCRB9\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb9`] module"]
#[doc(alias = "PCRB9")]
pub type Pcrb9 = crate::Reg<pcrb9::Pcrb9Spec>;
#[doc = "desc PCRB9"]
pub mod pcrb9;
#[doc = "PFSRB9 (rw) register accessor: desc PFSRB9\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb9`] module"]
#[doc(alias = "PFSRB9")]
pub type Pfsrb9 = crate::Reg<pfsrb9::Pfsrb9Spec>;
#[doc = "desc PFSRB9"]
pub mod pfsrb9;
#[doc = "PCRB10 (rw) register accessor: desc PCRB10\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb10`] module"]
#[doc(alias = "PCRB10")]
pub type Pcrb10 = crate::Reg<pcrb10::Pcrb10Spec>;
#[doc = "desc PCRB10"]
pub mod pcrb10;
#[doc = "PFSRB10 (rw) register accessor: desc PFSRB10\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb10`] module"]
#[doc(alias = "PFSRB10")]
pub type Pfsrb10 = crate::Reg<pfsrb10::Pfsrb10Spec>;
#[doc = "desc PFSRB10"]
pub mod pfsrb10;
#[doc = "PCRB11 (rw) register accessor: desc PCRB11\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb11`] module"]
#[doc(alias = "PCRB11")]
pub type Pcrb11 = crate::Reg<pcrb11::Pcrb11Spec>;
#[doc = "desc PCRB11"]
pub mod pcrb11;
#[doc = "PFSRB11 (rw) register accessor: desc PFSRB11\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb11`] module"]
#[doc(alias = "PFSRB11")]
pub type Pfsrb11 = crate::Reg<pfsrb11::Pfsrb11Spec>;
#[doc = "desc PFSRB11"]
pub mod pfsrb11;
#[doc = "PCRB12 (rw) register accessor: desc PCRB12\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb12`] module"]
#[doc(alias = "PCRB12")]
pub type Pcrb12 = crate::Reg<pcrb12::Pcrb12Spec>;
#[doc = "desc PCRB12"]
pub mod pcrb12;
#[doc = "PFSRB12 (rw) register accessor: desc PFSRB12\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb12`] module"]
#[doc(alias = "PFSRB12")]
pub type Pfsrb12 = crate::Reg<pfsrb12::Pfsrb12Spec>;
#[doc = "desc PFSRB12"]
pub mod pfsrb12;
#[doc = "PCRB13 (rw) register accessor: desc PCRB13\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb13`] module"]
#[doc(alias = "PCRB13")]
pub type Pcrb13 = crate::Reg<pcrb13::Pcrb13Spec>;
#[doc = "desc PCRB13"]
pub mod pcrb13;
#[doc = "PFSRB13 (rw) register accessor: desc PFSRB13\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb13`] module"]
#[doc(alias = "PFSRB13")]
pub type Pfsrb13 = crate::Reg<pfsrb13::Pfsrb13Spec>;
#[doc = "desc PFSRB13"]
pub mod pfsrb13;
#[doc = "PCRB14 (rw) register accessor: desc PCRB14\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb14`] module"]
#[doc(alias = "PCRB14")]
pub type Pcrb14 = crate::Reg<pcrb14::Pcrb14Spec>;
#[doc = "desc PCRB14"]
pub mod pcrb14;
#[doc = "PFSRB14 (rw) register accessor: desc PFSRB14\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb14`] module"]
#[doc(alias = "PFSRB14")]
pub type Pfsrb14 = crate::Reg<pfsrb14::Pfsrb14Spec>;
#[doc = "desc PFSRB14"]
pub mod pfsrb14;
#[doc = "PCRB15 (rw) register accessor: desc PCRB15\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrb15`] module"]
#[doc(alias = "PCRB15")]
pub type Pcrb15 = crate::Reg<pcrb15::Pcrb15Spec>;
#[doc = "desc PCRB15"]
pub mod pcrb15;
#[doc = "PFSRB15 (rw) register accessor: desc PFSRB15\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrb15`] module"]
#[doc(alias = "PFSRB15")]
pub type Pfsrb15 = crate::Reg<pfsrb15::Pfsrb15Spec>;
#[doc = "desc PFSRB15"]
pub mod pfsrb15;
#[doc = "PCRC13 (rw) register accessor: desc PCRC13\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrc13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrc13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrc13`] module"]
#[doc(alias = "PCRC13")]
pub type Pcrc13 = crate::Reg<pcrc13::Pcrc13Spec>;
#[doc = "desc PCRC13"]
pub mod pcrc13;
#[doc = "PFSRC13 (rw) register accessor: desc PFSRC13\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrc13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrc13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrc13`] module"]
#[doc(alias = "PFSRC13")]
pub type Pfsrc13 = crate::Reg<pfsrc13::Pfsrc13Spec>;
#[doc = "desc PFSRC13"]
pub mod pfsrc13;
#[doc = "PCRC14 (rw) register accessor: desc PCRC14\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrc14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrc14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrc14`] module"]
#[doc(alias = "PCRC14")]
pub type Pcrc14 = crate::Reg<pcrc14::Pcrc14Spec>;
#[doc = "desc PCRC14"]
pub mod pcrc14;
#[doc = "PFSRC14 (rw) register accessor: desc PFSRC14\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrc14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrc14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrc14`] module"]
#[doc(alias = "PFSRC14")]
pub type Pfsrc14 = crate::Reg<pfsrc14::Pfsrc14Spec>;
#[doc = "desc PFSRC14"]
pub mod pfsrc14;
#[doc = "PCRC15 (rw) register accessor: desc PCRC15\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrc15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrc15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrc15`] module"]
#[doc(alias = "PCRC15")]
pub type Pcrc15 = crate::Reg<pcrc15::Pcrc15Spec>;
#[doc = "desc PCRC15"]
pub mod pcrc15;
#[doc = "PFSRC15 (rw) register accessor: desc PFSRC15\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrc15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrc15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrc15`] module"]
#[doc(alias = "PFSRC15")]
pub type Pfsrc15 = crate::Reg<pfsrc15::Pfsrc15Spec>;
#[doc = "desc PFSRC15"]
pub mod pfsrc15;
#[doc = "PCRH0 (rw) register accessor: desc PCRH0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrh0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrh0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrh0`] module"]
#[doc(alias = "PCRH0")]
pub type Pcrh0 = crate::Reg<pcrh0::Pcrh0Spec>;
#[doc = "desc PCRH0"]
pub mod pcrh0;
#[doc = "PFSRH0 (rw) register accessor: desc PFSRH0\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrh0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrh0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrh0`] module"]
#[doc(alias = "PFSRH0")]
pub type Pfsrh0 = crate::Reg<pfsrh0::Pfsrh0Spec>;
#[doc = "desc PFSRH0"]
pub mod pfsrh0;
#[doc = "PCRH1 (rw) register accessor: desc PCRH1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrh1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrh1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrh1`] module"]
#[doc(alias = "PCRH1")]
pub type Pcrh1 = crate::Reg<pcrh1::Pcrh1Spec>;
#[doc = "desc PCRH1"]
pub mod pcrh1;
#[doc = "PFSRH1 (rw) register accessor: desc PFSRH1\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrh1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrh1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrh1`] module"]
#[doc(alias = "PFSRH1")]
pub type Pfsrh1 = crate::Reg<pfsrh1::Pfsrh1Spec>;
#[doc = "desc PFSRH1"]
pub mod pfsrh1;
#[doc = "PCRH2 (rw) register accessor: desc PCRH2\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrh2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrh2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrh2`] module"]
#[doc(alias = "PCRH2")]
pub type Pcrh2 = crate::Reg<pcrh2::Pcrh2Spec>;
#[doc = "desc PCRH2"]
pub mod pcrh2;
#[doc = "PFSRH2 (rw) register accessor: desc PFSRH2\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrh2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrh2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrh2`] module"]
#[doc(alias = "PFSRH2")]
pub type Pfsrh2 = crate::Reg<pfsrh2::Pfsrh2Spec>;
#[doc = "desc PFSRH2"]
pub mod pfsrh2;
