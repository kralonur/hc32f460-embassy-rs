#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    reslt: Reslt,
    _reserved2: [u8; 0x04],
    flg: Flg,
    _reserved3: [u8; 0x70],
    dat0: Dat0,
    dat1: Dat1,
    dat2: Dat2,
    dat3: Dat3,
    dat4: Dat4,
    dat5: Dat5,
    dat6: Dat6,
    dat7: Dat7,
    dat8: Dat8,
    dat9: Dat9,
    dat10: Dat10,
    dat11: Dat11,
    dat12: Dat12,
    dat13: Dat13,
    dat14: Dat14,
    dat15: Dat15,
    dat16: Dat16,
    dat17: Dat17,
    dat18: Dat18,
    dat19: Dat19,
    dat20: Dat20,
    dat21: Dat21,
    dat22: Dat22,
    dat23: Dat23,
    dat24: Dat24,
    dat25: Dat25,
    dat26: Dat26,
    dat27: Dat27,
    dat28: Dat28,
    dat29: Dat29,
    dat30: Dat30,
    dat31: Dat31,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - desc RESLT"]
    #[inline(always)]
    pub const fn reslt(&self) -> &Reslt {
        &self.reslt
    }
    #[doc = "0x0c - desc FLG"]
    #[inline(always)]
    pub const fn flg(&self) -> &Flg {
        &self.flg
    }
    #[doc = "0x80 - desc DAT0"]
    #[inline(always)]
    pub const fn dat0(&self) -> &Dat0 {
        &self.dat0
    }
    #[doc = "0x84 - desc DAT1"]
    #[inline(always)]
    pub const fn dat1(&self) -> &Dat1 {
        &self.dat1
    }
    #[doc = "0x88 - desc DAT2"]
    #[inline(always)]
    pub const fn dat2(&self) -> &Dat2 {
        &self.dat2
    }
    #[doc = "0x8c - desc DAT3"]
    #[inline(always)]
    pub const fn dat3(&self) -> &Dat3 {
        &self.dat3
    }
    #[doc = "0x90 - desc DAT4"]
    #[inline(always)]
    pub const fn dat4(&self) -> &Dat4 {
        &self.dat4
    }
    #[doc = "0x94 - desc DAT5"]
    #[inline(always)]
    pub const fn dat5(&self) -> &Dat5 {
        &self.dat5
    }
    #[doc = "0x98 - desc DAT6"]
    #[inline(always)]
    pub const fn dat6(&self) -> &Dat6 {
        &self.dat6
    }
    #[doc = "0x9c - desc DAT7"]
    #[inline(always)]
    pub const fn dat7(&self) -> &Dat7 {
        &self.dat7
    }
    #[doc = "0xa0 - desc DAT8"]
    #[inline(always)]
    pub const fn dat8(&self) -> &Dat8 {
        &self.dat8
    }
    #[doc = "0xa4 - desc DAT9"]
    #[inline(always)]
    pub const fn dat9(&self) -> &Dat9 {
        &self.dat9
    }
    #[doc = "0xa8 - desc DAT10"]
    #[inline(always)]
    pub const fn dat10(&self) -> &Dat10 {
        &self.dat10
    }
    #[doc = "0xac - desc DAT11"]
    #[inline(always)]
    pub const fn dat11(&self) -> &Dat11 {
        &self.dat11
    }
    #[doc = "0xb0 - desc DAT12"]
    #[inline(always)]
    pub const fn dat12(&self) -> &Dat12 {
        &self.dat12
    }
    #[doc = "0xb4 - desc DAT13"]
    #[inline(always)]
    pub const fn dat13(&self) -> &Dat13 {
        &self.dat13
    }
    #[doc = "0xb8 - desc DAT14"]
    #[inline(always)]
    pub const fn dat14(&self) -> &Dat14 {
        &self.dat14
    }
    #[doc = "0xbc - desc DAT15"]
    #[inline(always)]
    pub const fn dat15(&self) -> &Dat15 {
        &self.dat15
    }
    #[doc = "0xc0 - desc DAT16"]
    #[inline(always)]
    pub const fn dat16(&self) -> &Dat16 {
        &self.dat16
    }
    #[doc = "0xc4 - desc DAT17"]
    #[inline(always)]
    pub const fn dat17(&self) -> &Dat17 {
        &self.dat17
    }
    #[doc = "0xc8 - desc DAT18"]
    #[inline(always)]
    pub const fn dat18(&self) -> &Dat18 {
        &self.dat18
    }
    #[doc = "0xcc - desc DAT19"]
    #[inline(always)]
    pub const fn dat19(&self) -> &Dat19 {
        &self.dat19
    }
    #[doc = "0xd0 - desc DAT20"]
    #[inline(always)]
    pub const fn dat20(&self) -> &Dat20 {
        &self.dat20
    }
    #[doc = "0xd4 - desc DAT21"]
    #[inline(always)]
    pub const fn dat21(&self) -> &Dat21 {
        &self.dat21
    }
    #[doc = "0xd8 - desc DAT22"]
    #[inline(always)]
    pub const fn dat22(&self) -> &Dat22 {
        &self.dat22
    }
    #[doc = "0xdc - desc DAT23"]
    #[inline(always)]
    pub const fn dat23(&self) -> &Dat23 {
        &self.dat23
    }
    #[doc = "0xe0 - desc DAT24"]
    #[inline(always)]
    pub const fn dat24(&self) -> &Dat24 {
        &self.dat24
    }
    #[doc = "0xe4 - desc DAT25"]
    #[inline(always)]
    pub const fn dat25(&self) -> &Dat25 {
        &self.dat25
    }
    #[doc = "0xe8 - desc DAT26"]
    #[inline(always)]
    pub const fn dat26(&self) -> &Dat26 {
        &self.dat26
    }
    #[doc = "0xec - desc DAT27"]
    #[inline(always)]
    pub const fn dat27(&self) -> &Dat27 {
        &self.dat27
    }
    #[doc = "0xf0 - desc DAT28"]
    #[inline(always)]
    pub const fn dat28(&self) -> &Dat28 {
        &self.dat28
    }
    #[doc = "0xf4 - desc DAT29"]
    #[inline(always)]
    pub const fn dat29(&self) -> &Dat29 {
        &self.dat29
    }
    #[doc = "0xf8 - desc DAT30"]
    #[inline(always)]
    pub const fn dat30(&self) -> &Dat30 {
        &self.dat30
    }
    #[doc = "0xfc - desc DAT31"]
    #[inline(always)]
    pub const fn dat31(&self) -> &Dat31 {
        &self.dat31
    }
}
#[doc = "CR (rw) register accessor: desc CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "RESLT (rw) register accessor: desc RESLT\n\nYou can [`read`](crate::Reg::read) this register and get [`reslt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reslt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reslt`] module"]
#[doc(alias = "RESLT")]
pub type Reslt = crate::Reg<reslt::ResltSpec>;
#[doc = "desc RESLT"]
pub mod reslt;
#[doc = "FLG (r) register accessor: desc FLG\n\nYou can [`read`](crate::Reg::read) this register and get [`flg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flg`] module"]
#[doc(alias = "FLG")]
pub type Flg = crate::Reg<flg::FlgSpec>;
#[doc = "desc FLG"]
pub mod flg;
#[doc = "DAT0 (r) register accessor: desc DAT0\n\nYou can [`read`](crate::Reg::read) this register and get [`dat0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat0`] module"]
#[doc(alias = "DAT0")]
pub type Dat0 = crate::Reg<dat0::Dat0Spec>;
#[doc = "desc DAT0"]
pub mod dat0;
#[doc = "DAT1 (r) register accessor: desc DAT1\n\nYou can [`read`](crate::Reg::read) this register and get [`dat1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat1`] module"]
#[doc(alias = "DAT1")]
pub type Dat1 = crate::Reg<dat1::Dat1Spec>;
#[doc = "desc DAT1"]
pub mod dat1;
#[doc = "DAT2 (r) register accessor: desc DAT2\n\nYou can [`read`](crate::Reg::read) this register and get [`dat2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat2`] module"]
#[doc(alias = "DAT2")]
pub type Dat2 = crate::Reg<dat2::Dat2Spec>;
#[doc = "desc DAT2"]
pub mod dat2;
#[doc = "DAT3 (r) register accessor: desc DAT3\n\nYou can [`read`](crate::Reg::read) this register and get [`dat3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat3`] module"]
#[doc(alias = "DAT3")]
pub type Dat3 = crate::Reg<dat3::Dat3Spec>;
#[doc = "desc DAT3"]
pub mod dat3;
#[doc = "DAT4 (r) register accessor: desc DAT4\n\nYou can [`read`](crate::Reg::read) this register and get [`dat4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat4`] module"]
#[doc(alias = "DAT4")]
pub type Dat4 = crate::Reg<dat4::Dat4Spec>;
#[doc = "desc DAT4"]
pub mod dat4;
#[doc = "DAT5 (r) register accessor: desc DAT5\n\nYou can [`read`](crate::Reg::read) this register and get [`dat5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat5`] module"]
#[doc(alias = "DAT5")]
pub type Dat5 = crate::Reg<dat5::Dat5Spec>;
#[doc = "desc DAT5"]
pub mod dat5;
#[doc = "DAT6 (r) register accessor: desc DAT6\n\nYou can [`read`](crate::Reg::read) this register and get [`dat6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat6`] module"]
#[doc(alias = "DAT6")]
pub type Dat6 = crate::Reg<dat6::Dat6Spec>;
#[doc = "desc DAT6"]
pub mod dat6;
#[doc = "DAT7 (r) register accessor: desc DAT7\n\nYou can [`read`](crate::Reg::read) this register and get [`dat7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat7`] module"]
#[doc(alias = "DAT7")]
pub type Dat7 = crate::Reg<dat7::Dat7Spec>;
#[doc = "desc DAT7"]
pub mod dat7;
#[doc = "DAT8 (r) register accessor: desc DAT8\n\nYou can [`read`](crate::Reg::read) this register and get [`dat8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat8`] module"]
#[doc(alias = "DAT8")]
pub type Dat8 = crate::Reg<dat8::Dat8Spec>;
#[doc = "desc DAT8"]
pub mod dat8;
#[doc = "DAT9 (r) register accessor: desc DAT9\n\nYou can [`read`](crate::Reg::read) this register and get [`dat9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat9`] module"]
#[doc(alias = "DAT9")]
pub type Dat9 = crate::Reg<dat9::Dat9Spec>;
#[doc = "desc DAT9"]
pub mod dat9;
#[doc = "DAT10 (r) register accessor: desc DAT10\n\nYou can [`read`](crate::Reg::read) this register and get [`dat10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat10`] module"]
#[doc(alias = "DAT10")]
pub type Dat10 = crate::Reg<dat10::Dat10Spec>;
#[doc = "desc DAT10"]
pub mod dat10;
#[doc = "DAT11 (r) register accessor: desc DAT11\n\nYou can [`read`](crate::Reg::read) this register and get [`dat11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat11`] module"]
#[doc(alias = "DAT11")]
pub type Dat11 = crate::Reg<dat11::Dat11Spec>;
#[doc = "desc DAT11"]
pub mod dat11;
#[doc = "DAT12 (r) register accessor: desc DAT12\n\nYou can [`read`](crate::Reg::read) this register and get [`dat12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat12`] module"]
#[doc(alias = "DAT12")]
pub type Dat12 = crate::Reg<dat12::Dat12Spec>;
#[doc = "desc DAT12"]
pub mod dat12;
#[doc = "DAT13 (r) register accessor: desc DAT13\n\nYou can [`read`](crate::Reg::read) this register and get [`dat13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat13`] module"]
#[doc(alias = "DAT13")]
pub type Dat13 = crate::Reg<dat13::Dat13Spec>;
#[doc = "desc DAT13"]
pub mod dat13;
#[doc = "DAT14 (r) register accessor: desc DAT14\n\nYou can [`read`](crate::Reg::read) this register and get [`dat14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat14`] module"]
#[doc(alias = "DAT14")]
pub type Dat14 = crate::Reg<dat14::Dat14Spec>;
#[doc = "desc DAT14"]
pub mod dat14;
#[doc = "DAT15 (r) register accessor: desc DAT15\n\nYou can [`read`](crate::Reg::read) this register and get [`dat15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat15`] module"]
#[doc(alias = "DAT15")]
pub type Dat15 = crate::Reg<dat15::Dat15Spec>;
#[doc = "desc DAT15"]
pub mod dat15;
#[doc = "DAT16 (r) register accessor: desc DAT16\n\nYou can [`read`](crate::Reg::read) this register and get [`dat16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat16`] module"]
#[doc(alias = "DAT16")]
pub type Dat16 = crate::Reg<dat16::Dat16Spec>;
#[doc = "desc DAT16"]
pub mod dat16;
#[doc = "DAT17 (r) register accessor: desc DAT17\n\nYou can [`read`](crate::Reg::read) this register and get [`dat17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat17`] module"]
#[doc(alias = "DAT17")]
pub type Dat17 = crate::Reg<dat17::Dat17Spec>;
#[doc = "desc DAT17"]
pub mod dat17;
#[doc = "DAT18 (r) register accessor: desc DAT18\n\nYou can [`read`](crate::Reg::read) this register and get [`dat18::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat18`] module"]
#[doc(alias = "DAT18")]
pub type Dat18 = crate::Reg<dat18::Dat18Spec>;
#[doc = "desc DAT18"]
pub mod dat18;
#[doc = "DAT19 (r) register accessor: desc DAT19\n\nYou can [`read`](crate::Reg::read) this register and get [`dat19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat19`] module"]
#[doc(alias = "DAT19")]
pub type Dat19 = crate::Reg<dat19::Dat19Spec>;
#[doc = "desc DAT19"]
pub mod dat19;
#[doc = "DAT20 (r) register accessor: desc DAT20\n\nYou can [`read`](crate::Reg::read) this register and get [`dat20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat20`] module"]
#[doc(alias = "DAT20")]
pub type Dat20 = crate::Reg<dat20::Dat20Spec>;
#[doc = "desc DAT20"]
pub mod dat20;
#[doc = "DAT21 (r) register accessor: desc DAT21\n\nYou can [`read`](crate::Reg::read) this register and get [`dat21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat21`] module"]
#[doc(alias = "DAT21")]
pub type Dat21 = crate::Reg<dat21::Dat21Spec>;
#[doc = "desc DAT21"]
pub mod dat21;
#[doc = "DAT22 (r) register accessor: desc DAT22\n\nYou can [`read`](crate::Reg::read) this register and get [`dat22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat22`] module"]
#[doc(alias = "DAT22")]
pub type Dat22 = crate::Reg<dat22::Dat22Spec>;
#[doc = "desc DAT22"]
pub mod dat22;
#[doc = "DAT23 (r) register accessor: desc DAT23\n\nYou can [`read`](crate::Reg::read) this register and get [`dat23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat23`] module"]
#[doc(alias = "DAT23")]
pub type Dat23 = crate::Reg<dat23::Dat23Spec>;
#[doc = "desc DAT23"]
pub mod dat23;
#[doc = "DAT24 (r) register accessor: desc DAT24\n\nYou can [`read`](crate::Reg::read) this register and get [`dat24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat24`] module"]
#[doc(alias = "DAT24")]
pub type Dat24 = crate::Reg<dat24::Dat24Spec>;
#[doc = "desc DAT24"]
pub mod dat24;
#[doc = "DAT25 (r) register accessor: desc DAT25\n\nYou can [`read`](crate::Reg::read) this register and get [`dat25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat25`] module"]
#[doc(alias = "DAT25")]
pub type Dat25 = crate::Reg<dat25::Dat25Spec>;
#[doc = "desc DAT25"]
pub mod dat25;
#[doc = "DAT26 (r) register accessor: desc DAT26\n\nYou can [`read`](crate::Reg::read) this register and get [`dat26::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat26`] module"]
#[doc(alias = "DAT26")]
pub type Dat26 = crate::Reg<dat26::Dat26Spec>;
#[doc = "desc DAT26"]
pub mod dat26;
#[doc = "DAT27 (r) register accessor: desc DAT27\n\nYou can [`read`](crate::Reg::read) this register and get [`dat27::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat27`] module"]
#[doc(alias = "DAT27")]
pub type Dat27 = crate::Reg<dat27::Dat27Spec>;
#[doc = "desc DAT27"]
pub mod dat27;
#[doc = "DAT28 (r) register accessor: desc DAT28\n\nYou can [`read`](crate::Reg::read) this register and get [`dat28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat28`] module"]
#[doc(alias = "DAT28")]
pub type Dat28 = crate::Reg<dat28::Dat28Spec>;
#[doc = "desc DAT28"]
pub mod dat28;
#[doc = "DAT29 (r) register accessor: desc DAT29\n\nYou can [`read`](crate::Reg::read) this register and get [`dat29::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat29`] module"]
#[doc(alias = "DAT29")]
pub type Dat29 = crate::Reg<dat29::Dat29Spec>;
#[doc = "desc DAT29"]
pub mod dat29;
#[doc = "DAT30 (r) register accessor: desc DAT30\n\nYou can [`read`](crate::Reg::read) this register and get [`dat30::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat30`] module"]
#[doc(alias = "DAT30")]
pub type Dat30 = crate::Reg<dat30::Dat30Spec>;
#[doc = "desc DAT30"]
pub mod dat30;
#[doc = "DAT31 (r) register accessor: desc DAT31\n\nYou can [`read`](crate::Reg::read) this register and get [`dat31::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat31`] module"]
#[doc(alias = "DAT31")]
pub type Dat31 = crate::Reg<dat31::Dat31Spec>;
#[doc = "desc DAT31"]
pub mod dat31;
