#[doc = "Register `NMIENR` reader"]
pub type R = crate::R<NmienrSpec>;
#[doc = "Register `NMIENR` writer"]
pub type W = crate::W<NmienrSpec>;
#[doc = "Field `NMIENR` reader - desc NMIENR"]
pub type NmienrR = crate::BitReader;
#[doc = "Field `NMIENR` writer - desc NMIENR"]
pub type NmienrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDTENR` reader - desc SWDTENR"]
pub type SwdtenrR = crate::BitReader;
#[doc = "Field `SWDTENR` writer - desc SWDTENR"]
pub type SwdtenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD1ENR` reader - desc PVD1ENR"]
pub type Pvd1enrR = crate::BitReader;
#[doc = "Field `PVD1ENR` writer - desc PVD1ENR"]
pub type Pvd1enrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2ENR` reader - desc PVD2ENR"]
pub type Pvd2enrR = crate::BitReader;
#[doc = "Field `PVD2ENR` writer - desc PVD2ENR"]
pub type Pvd2enrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALSTPENR` reader - desc XTALSTPENR"]
pub type XtalstpenrR = crate::BitReader;
#[doc = "Field `XTALSTPENR` writer - desc XTALSTPENR"]
pub type XtalstpenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPENR` reader - desc REPENR"]
pub type RepenrR = crate::BitReader;
#[doc = "Field `REPENR` writer - desc REPENR"]
pub type RepenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECCENR` reader - desc RECCENR"]
pub type ReccenrR = crate::BitReader;
#[doc = "Field `RECCENR` writer - desc RECCENR"]
pub type ReccenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSMENR` reader - desc BUSMENR"]
pub type BusmenrR = crate::BitReader;
#[doc = "Field `BUSMENR` writer - desc BUSMENR"]
pub type BusmenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTENR` reader - desc WDTENR"]
pub type WdtenrR = crate::BitReader;
#[doc = "Field `WDTENR` writer - desc WDTENR"]
pub type WdtenrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc NMIENR"]
    #[inline(always)]
    pub fn nmienr(&self) -> NmienrR {
        NmienrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SWDTENR"]
    #[inline(always)]
    pub fn swdtenr(&self) -> SwdtenrR {
        SwdtenrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PVD1ENR"]
    #[inline(always)]
    pub fn pvd1enr(&self) -> Pvd1enrR {
        Pvd1enrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PVD2ENR"]
    #[inline(always)]
    pub fn pvd2enr(&self) -> Pvd2enrR {
        Pvd2enrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - desc XTALSTPENR"]
    #[inline(always)]
    pub fn xtalstpenr(&self) -> XtalstpenrR {
        XtalstpenrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - desc REPENR"]
    #[inline(always)]
    pub fn repenr(&self) -> RepenrR {
        RepenrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc RECCENR"]
    #[inline(always)]
    pub fn reccenr(&self) -> ReccenrR {
        ReccenrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BUSMENR"]
    #[inline(always)]
    pub fn busmenr(&self) -> BusmenrR {
        BusmenrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc WDTENR"]
    #[inline(always)]
    pub fn wdtenr(&self) -> WdtenrR {
        WdtenrR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc NMIENR"]
    #[inline(always)]
    pub fn nmienr(&mut self) -> NmienrW<'_, NmienrSpec> {
        NmienrW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SWDTENR"]
    #[inline(always)]
    pub fn swdtenr(&mut self) -> SwdtenrW<'_, NmienrSpec> {
        SwdtenrW::new(self, 1)
    }
    #[doc = "Bit 2 - desc PVD1ENR"]
    #[inline(always)]
    pub fn pvd1enr(&mut self) -> Pvd1enrW<'_, NmienrSpec> {
        Pvd1enrW::new(self, 2)
    }
    #[doc = "Bit 3 - desc PVD2ENR"]
    #[inline(always)]
    pub fn pvd2enr(&mut self) -> Pvd2enrW<'_, NmienrSpec> {
        Pvd2enrW::new(self, 3)
    }
    #[doc = "Bit 5 - desc XTALSTPENR"]
    #[inline(always)]
    pub fn xtalstpenr(&mut self) -> XtalstpenrW<'_, NmienrSpec> {
        XtalstpenrW::new(self, 5)
    }
    #[doc = "Bit 8 - desc REPENR"]
    #[inline(always)]
    pub fn repenr(&mut self) -> RepenrW<'_, NmienrSpec> {
        RepenrW::new(self, 8)
    }
    #[doc = "Bit 9 - desc RECCENR"]
    #[inline(always)]
    pub fn reccenr(&mut self) -> ReccenrW<'_, NmienrSpec> {
        ReccenrW::new(self, 9)
    }
    #[doc = "Bit 10 - desc BUSMENR"]
    #[inline(always)]
    pub fn busmenr(&mut self) -> BusmenrW<'_, NmienrSpec> {
        BusmenrW::new(self, 10)
    }
    #[doc = "Bit 11 - desc WDTENR"]
    #[inline(always)]
    pub fn wdtenr(&mut self) -> WdtenrW<'_, NmienrSpec> {
        WdtenrW::new(self, 11)
    }
}
#[doc = "desc NMIENR\n\nYou can [`read`](crate::Reg::read) this register and get [`nmienr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmienr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmienrSpec;
impl crate::RegisterSpec for NmienrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmienr::R`](R) reader structure"]
impl crate::Readable for NmienrSpec {}
#[doc = "`write(|w| ..)` method takes [`nmienr::W`](W) writer structure"]
impl crate::Writable for NmienrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NMIENR to value 0"]
impl crate::Resettable for NmienrSpec {}
