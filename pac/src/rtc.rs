#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr0: Cr0,
    _reserved1: [u8; 0x03],
    cr1: Cr1,
    _reserved2: [u8; 0x03],
    cr2: Cr2,
    _reserved3: [u8; 0x03],
    cr3: Cr3,
    _reserved4: [u8; 0x03],
    sec: Sec,
    _reserved5: [u8; 0x03],
    min: Min,
    _reserved6: [u8; 0x03],
    hour: Hour,
    _reserved7: [u8; 0x03],
    week: Week,
    _reserved8: [u8; 0x03],
    day: Day,
    _reserved9: [u8; 0x03],
    mon: Mon,
    _reserved10: [u8; 0x03],
    year: Year,
    _reserved11: [u8; 0x03],
    almmin: Almmin,
    _reserved12: [u8; 0x03],
    almhour: Almhour,
    _reserved13: [u8; 0x03],
    almweek: Almweek,
    _reserved14: [u8; 0x03],
    errcrh: Errcrh,
    _reserved15: [u8; 0x03],
    errcrl: Errcrl,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CR0"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0x04 - desc CR1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x08 - desc CR2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x0c - desc CR3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &Cr3 {
        &self.cr3
    }
    #[doc = "0x10 - desc SEC"]
    #[inline(always)]
    pub const fn sec(&self) -> &Sec {
        &self.sec
    }
    #[doc = "0x14 - desc MIN"]
    #[inline(always)]
    pub const fn min(&self) -> &Min {
        &self.min
    }
    #[doc = "0x18 - desc HOUR"]
    #[inline(always)]
    pub const fn hour(&self) -> &Hour {
        &self.hour
    }
    #[doc = "0x1c - desc WEEK"]
    #[inline(always)]
    pub const fn week(&self) -> &Week {
        &self.week
    }
    #[doc = "0x20 - desc DAY"]
    #[inline(always)]
    pub const fn day(&self) -> &Day {
        &self.day
    }
    #[doc = "0x24 - desc MON"]
    #[inline(always)]
    pub const fn mon(&self) -> &Mon {
        &self.mon
    }
    #[doc = "0x28 - desc YEAR"]
    #[inline(always)]
    pub const fn year(&self) -> &Year {
        &self.year
    }
    #[doc = "0x2c - desc ALMMIN"]
    #[inline(always)]
    pub const fn almmin(&self) -> &Almmin {
        &self.almmin
    }
    #[doc = "0x30 - desc ALMHOUR"]
    #[inline(always)]
    pub const fn almhour(&self) -> &Almhour {
        &self.almhour
    }
    #[doc = "0x34 - desc ALMWEEK"]
    #[inline(always)]
    pub const fn almweek(&self) -> &Almweek {
        &self.almweek
    }
    #[doc = "0x38 - desc ERRCRH"]
    #[inline(always)]
    pub const fn errcrh(&self) -> &Errcrh {
        &self.errcrh
    }
    #[doc = "0x3c - desc ERRCRL"]
    #[inline(always)]
    pub const fn errcrl(&self) -> &Errcrl {
        &self.errcrl
    }
}
#[doc = "CR0 (rw) register accessor: desc CR0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`] module"]
#[doc(alias = "CR0")]
pub type Cr0 = crate::Reg<cr0::Cr0Spec>;
#[doc = "desc CR0"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: desc CR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "desc CR1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: desc CR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "desc CR2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: desc CR3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`] module"]
#[doc(alias = "CR3")]
pub type Cr3 = crate::Reg<cr3::Cr3Spec>;
#[doc = "desc CR3"]
pub mod cr3;
#[doc = "SEC (rw) register accessor: desc SEC\n\nYou can [`read`](crate::Reg::read) this register and get [`sec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec`] module"]
#[doc(alias = "SEC")]
pub type Sec = crate::Reg<sec::SecSpec>;
#[doc = "desc SEC"]
pub mod sec;
#[doc = "MIN (rw) register accessor: desc MIN\n\nYou can [`read`](crate::Reg::read) this register and get [`min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@min`] module"]
#[doc(alias = "MIN")]
pub type Min = crate::Reg<min::MinSpec>;
#[doc = "desc MIN"]
pub mod min;
#[doc = "HOUR (rw) register accessor: desc HOUR\n\nYou can [`read`](crate::Reg::read) this register and get [`hour::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hour::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hour`] module"]
#[doc(alias = "HOUR")]
pub type Hour = crate::Reg<hour::HourSpec>;
#[doc = "desc HOUR"]
pub mod hour;
#[doc = "WEEK (rw) register accessor: desc WEEK\n\nYou can [`read`](crate::Reg::read) this register and get [`week::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`week::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@week`] module"]
#[doc(alias = "WEEK")]
pub type Week = crate::Reg<week::WeekSpec>;
#[doc = "desc WEEK"]
pub mod week;
#[doc = "DAY (rw) register accessor: desc DAY\n\nYou can [`read`](crate::Reg::read) this register and get [`day::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`day::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@day`] module"]
#[doc(alias = "DAY")]
pub type Day = crate::Reg<day::DaySpec>;
#[doc = "desc DAY"]
pub mod day;
#[doc = "MON (rw) register accessor: desc MON\n\nYou can [`read`](crate::Reg::read) this register and get [`mon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mon`] module"]
#[doc(alias = "MON")]
pub type Mon = crate::Reg<mon::MonSpec>;
#[doc = "desc MON"]
pub mod mon;
#[doc = "YEAR (rw) register accessor: desc YEAR\n\nYou can [`read`](crate::Reg::read) this register and get [`year::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`year::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@year`] module"]
#[doc(alias = "YEAR")]
pub type Year = crate::Reg<year::YearSpec>;
#[doc = "desc YEAR"]
pub mod year;
#[doc = "ALMMIN (rw) register accessor: desc ALMMIN\n\nYou can [`read`](crate::Reg::read) this register and get [`almmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@almmin`] module"]
#[doc(alias = "ALMMIN")]
pub type Almmin = crate::Reg<almmin::AlmminSpec>;
#[doc = "desc ALMMIN"]
pub mod almmin;
#[doc = "ALMHOUR (rw) register accessor: desc ALMHOUR\n\nYou can [`read`](crate::Reg::read) this register and get [`almhour::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almhour::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@almhour`] module"]
#[doc(alias = "ALMHOUR")]
pub type Almhour = crate::Reg<almhour::AlmhourSpec>;
#[doc = "desc ALMHOUR"]
pub mod almhour;
#[doc = "ALMWEEK (rw) register accessor: desc ALMWEEK\n\nYou can [`read`](crate::Reg::read) this register and get [`almweek::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almweek::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@almweek`] module"]
#[doc(alias = "ALMWEEK")]
pub type Almweek = crate::Reg<almweek::AlmweekSpec>;
#[doc = "desc ALMWEEK"]
pub mod almweek;
#[doc = "ERRCRH (rw) register accessor: desc ERRCRH\n\nYou can [`read`](crate::Reg::read) this register and get [`errcrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errcrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errcrh`] module"]
#[doc(alias = "ERRCRH")]
pub type Errcrh = crate::Reg<errcrh::ErrcrhSpec>;
#[doc = "desc ERRCRH"]
pub mod errcrh;
#[doc = "ERRCRL (rw) register accessor: desc ERRCRL\n\nYou can [`read`](crate::Reg::read) this register and get [`errcrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errcrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errcrl`] module"]
#[doc(alias = "ERRCRL")]
pub type Errcrl = crate::Reg<errcrl::ErrcrlSpec>;
#[doc = "desc ERRCRL"]
pub mod errcrl;
