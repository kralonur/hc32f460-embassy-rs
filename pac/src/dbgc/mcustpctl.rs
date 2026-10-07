#[doc = "Register `MCUSTPCTL` reader"]
pub type R = crate::R<McustpctlSpec>;
#[doc = "Register `MCUSTPCTL` writer"]
pub type W = crate::W<McustpctlSpec>;
#[doc = "Field `SWDTSTP` reader - desc SWDTSTP"]
pub type SwdtstpR = crate::BitReader;
#[doc = "Field `SWDTSTP` writer - desc SWDTSTP"]
pub type SwdtstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTSTP` reader - desc WDTSTP"]
pub type WdtstpR = crate::BitReader;
#[doc = "Field `WDTSTP` writer - desc WDTSTP"]
pub type WdtstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSTP` reader - desc RTCSTP"]
pub type RtcstpR = crate::BitReader;
#[doc = "Field `RTCSTP` writer - desc RTCSTP"]
pub type RtcstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR01STP` reader - desc TMR01STP"]
pub type Tmr01stpR = crate::BitReader;
#[doc = "Field `TMR01STP` writer - desc TMR01STP"]
pub type Tmr01stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR02STP` reader - desc TMR02STP"]
pub type Tmr02stpR = crate::BitReader;
#[doc = "Field `TMR02STP` writer - desc TMR02STP"]
pub type Tmr02stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR41STP` reader - desc TMR41STP"]
pub type Tmr41stpR = crate::BitReader;
#[doc = "Field `TMR41STP` writer - desc TMR41STP"]
pub type Tmr41stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR42STP` reader - desc TMR42STP"]
pub type Tmr42stpR = crate::BitReader;
#[doc = "Field `TMR42STP` writer - desc TMR42STP"]
pub type Tmr42stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR43STP` reader - desc TMR43STP"]
pub type Tmr43stpR = crate::BitReader;
#[doc = "Field `TMR43STP` writer - desc TMR43STP"]
pub type Tmr43stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM61STP` reader - desc TM61STP"]
pub type Tm61stpR = crate::BitReader;
#[doc = "Field `TM61STP` writer - desc TM61STP"]
pub type Tm61stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM62STP` reader - desc TM62STP"]
pub type Tm62stpR = crate::BitReader;
#[doc = "Field `TM62STP` writer - desc TM62STP"]
pub type Tm62stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR63STP` reader - desc TMR63STP"]
pub type Tmr63stpR = crate::BitReader;
#[doc = "Field `TMR63STP` writer - desc TMR63STP"]
pub type Tmr63stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRA1STP` reader - desc TMRA1STP"]
pub type Tmra1stpR = crate::BitReader;
#[doc = "Field `TMRA1STP` writer - desc TMRA1STP"]
pub type Tmra1stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRA2STP` reader - desc TMRA2STP"]
pub type Tmra2stpR = crate::BitReader;
#[doc = "Field `TMRA2STP` writer - desc TMRA2STP"]
pub type Tmra2stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRA3STP` reader - desc TMRA3STP"]
pub type Tmra3stpR = crate::BitReader;
#[doc = "Field `TMRA3STP` writer - desc TMRA3STP"]
pub type Tmra3stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRA4STP` reader - desc TMRA4STP"]
pub type Tmra4stpR = crate::BitReader;
#[doc = "Field `TMRA4STP` writer - desc TMRA4STP"]
pub type Tmra4stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRA5STP` reader - desc TMRA5STP"]
pub type Tmra5stpR = crate::BitReader;
#[doc = "Field `TMRA5STP` writer - desc TMRA5STP"]
pub type Tmra5stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRA6STP` reader - desc TMRA6STP"]
pub type Tmra6stpR = crate::BitReader;
#[doc = "Field `TMRA6STP` writer - desc TMRA6STP"]
pub type Tmra6stpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SWDTSTP"]
    #[inline(always)]
    pub fn swdtstp(&self) -> SwdtstpR {
        SwdtstpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc WDTSTP"]
    #[inline(always)]
    pub fn wdtstp(&self) -> WdtstpR {
        WdtstpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc RTCSTP"]
    #[inline(always)]
    pub fn rtcstp(&self) -> RtcstpR {
        RtcstpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 14 - desc TMR01STP"]
    #[inline(always)]
    pub fn tmr01stp(&self) -> Tmr01stpR {
        Tmr01stpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc TMR02STP"]
    #[inline(always)]
    pub fn tmr02stp(&self) -> Tmr02stpR {
        Tmr02stpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - desc TMR41STP"]
    #[inline(always)]
    pub fn tmr41stp(&self) -> Tmr41stpR {
        Tmr41stpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc TMR42STP"]
    #[inline(always)]
    pub fn tmr42stp(&self) -> Tmr42stpR {
        Tmr42stpR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc TMR43STP"]
    #[inline(always)]
    pub fn tmr43stp(&self) -> Tmr43stpR {
        Tmr43stpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc TM61STP"]
    #[inline(always)]
    pub fn tm61stp(&self) -> Tm61stpR {
        Tm61stpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc TM62STP"]
    #[inline(always)]
    pub fn tm62stp(&self) -> Tm62stpR {
        Tm62stpR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc TMR63STP"]
    #[inline(always)]
    pub fn tmr63stp(&self) -> Tmr63stpR {
        Tmr63stpR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc TMRA1STP"]
    #[inline(always)]
    pub fn tmra1stp(&self) -> Tmra1stpR {
        Tmra1stpR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc TMRA2STP"]
    #[inline(always)]
    pub fn tmra2stp(&self) -> Tmra2stpR {
        Tmra2stpR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc TMRA3STP"]
    #[inline(always)]
    pub fn tmra3stp(&self) -> Tmra3stpR {
        Tmra3stpR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc TMRA4STP"]
    #[inline(always)]
    pub fn tmra4stp(&self) -> Tmra4stpR {
        Tmra4stpR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc TMRA5STP"]
    #[inline(always)]
    pub fn tmra5stp(&self) -> Tmra5stpR {
        Tmra5stpR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc TMRA6STP"]
    #[inline(always)]
    pub fn tmra6stp(&self) -> Tmra6stpR {
        Tmra6stpR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SWDTSTP"]
    #[inline(always)]
    pub fn swdtstp(&mut self) -> SwdtstpW<'_, McustpctlSpec> {
        SwdtstpW::new(self, 0)
    }
    #[doc = "Bit 1 - desc WDTSTP"]
    #[inline(always)]
    pub fn wdtstp(&mut self) -> WdtstpW<'_, McustpctlSpec> {
        WdtstpW::new(self, 1)
    }
    #[doc = "Bit 2 - desc RTCSTP"]
    #[inline(always)]
    pub fn rtcstp(&mut self) -> RtcstpW<'_, McustpctlSpec> {
        RtcstpW::new(self, 2)
    }
    #[doc = "Bit 14 - desc TMR01STP"]
    #[inline(always)]
    pub fn tmr01stp(&mut self) -> Tmr01stpW<'_, McustpctlSpec> {
        Tmr01stpW::new(self, 14)
    }
    #[doc = "Bit 15 - desc TMR02STP"]
    #[inline(always)]
    pub fn tmr02stp(&mut self) -> Tmr02stpW<'_, McustpctlSpec> {
        Tmr02stpW::new(self, 15)
    }
    #[doc = "Bit 20 - desc TMR41STP"]
    #[inline(always)]
    pub fn tmr41stp(&mut self) -> Tmr41stpW<'_, McustpctlSpec> {
        Tmr41stpW::new(self, 20)
    }
    #[doc = "Bit 21 - desc TMR42STP"]
    #[inline(always)]
    pub fn tmr42stp(&mut self) -> Tmr42stpW<'_, McustpctlSpec> {
        Tmr42stpW::new(self, 21)
    }
    #[doc = "Bit 22 - desc TMR43STP"]
    #[inline(always)]
    pub fn tmr43stp(&mut self) -> Tmr43stpW<'_, McustpctlSpec> {
        Tmr43stpW::new(self, 22)
    }
    #[doc = "Bit 23 - desc TM61STP"]
    #[inline(always)]
    pub fn tm61stp(&mut self) -> Tm61stpW<'_, McustpctlSpec> {
        Tm61stpW::new(self, 23)
    }
    #[doc = "Bit 24 - desc TM62STP"]
    #[inline(always)]
    pub fn tm62stp(&mut self) -> Tm62stpW<'_, McustpctlSpec> {
        Tm62stpW::new(self, 24)
    }
    #[doc = "Bit 25 - desc TMR63STP"]
    #[inline(always)]
    pub fn tmr63stp(&mut self) -> Tmr63stpW<'_, McustpctlSpec> {
        Tmr63stpW::new(self, 25)
    }
    #[doc = "Bit 26 - desc TMRA1STP"]
    #[inline(always)]
    pub fn tmra1stp(&mut self) -> Tmra1stpW<'_, McustpctlSpec> {
        Tmra1stpW::new(self, 26)
    }
    #[doc = "Bit 27 - desc TMRA2STP"]
    #[inline(always)]
    pub fn tmra2stp(&mut self) -> Tmra2stpW<'_, McustpctlSpec> {
        Tmra2stpW::new(self, 27)
    }
    #[doc = "Bit 28 - desc TMRA3STP"]
    #[inline(always)]
    pub fn tmra3stp(&mut self) -> Tmra3stpW<'_, McustpctlSpec> {
        Tmra3stpW::new(self, 28)
    }
    #[doc = "Bit 29 - desc TMRA4STP"]
    #[inline(always)]
    pub fn tmra4stp(&mut self) -> Tmra4stpW<'_, McustpctlSpec> {
        Tmra4stpW::new(self, 29)
    }
    #[doc = "Bit 30 - desc TMRA5STP"]
    #[inline(always)]
    pub fn tmra5stp(&mut self) -> Tmra5stpW<'_, McustpctlSpec> {
        Tmra5stpW::new(self, 30)
    }
    #[doc = "Bit 31 - desc TMRA6STP"]
    #[inline(always)]
    pub fn tmra6stp(&mut self) -> Tmra6stpW<'_, McustpctlSpec> {
        Tmra6stpW::new(self, 31)
    }
}
#[doc = "desc MCUSTPCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`mcustpctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcustpctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McustpctlSpec;
impl crate::RegisterSpec for McustpctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcustpctl::R`](R) reader structure"]
impl crate::Readable for McustpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`mcustpctl::W`](W) writer structure"]
impl crate::Writable for McustpctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCUSTPCTL to value 0x03"]
impl crate::Resettable for McustpctlSpec {
    const RESET_VALUE: u32 = 0x03;
}
