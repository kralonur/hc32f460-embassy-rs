#[doc = "Register `NMICFR` reader"]
pub type R = crate::R<NmicfrSpec>;
#[doc = "Register `NMICFR` writer"]
pub type W = crate::W<NmicfrSpec>;
#[doc = "Field `NMICFR` reader - desc NMICFR"]
pub type NmicfrR = crate::BitReader;
#[doc = "Field `NMICFR` writer - desc NMICFR"]
pub type NmicfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDTCFR` reader - desc SWDTCFR"]
pub type SwdtcfrR = crate::BitReader;
#[doc = "Field `SWDTCFR` writer - desc SWDTCFR"]
pub type SwdtcfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD1CFR` reader - desc PVD1CFR"]
pub type Pvd1cfrR = crate::BitReader;
#[doc = "Field `PVD1CFR` writer - desc PVD1CFR"]
pub type Pvd1cfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2CFR` reader - desc PVD2CFR"]
pub type Pvd2cfrR = crate::BitReader;
#[doc = "Field `PVD2CFR` writer - desc PVD2CFR"]
pub type Pvd2cfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALSTPCFR` reader - desc XTALSTPCFR"]
pub type XtalstpcfrR = crate::BitReader;
#[doc = "Field `XTALSTPCFR` writer - desc XTALSTPCFR"]
pub type XtalstpcfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPCFR` reader - desc REPCFR"]
pub type RepcfrR = crate::BitReader;
#[doc = "Field `REPCFR` writer - desc REPCFR"]
pub type RepcfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECCCFR` reader - desc RECCCFR"]
pub type RecccfrR = crate::BitReader;
#[doc = "Field `RECCCFR` writer - desc RECCCFR"]
pub type RecccfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSMCFR` reader - desc BUSMCFR"]
pub type BusmcfrR = crate::BitReader;
#[doc = "Field `BUSMCFR` writer - desc BUSMCFR"]
pub type BusmcfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTCFR` reader - desc WDTCFR"]
pub type WdtcfrR = crate::BitReader;
#[doc = "Field `WDTCFR` writer - desc WDTCFR"]
pub type WdtcfrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc NMICFR"]
    #[inline(always)]
    pub fn nmicfr(&self) -> NmicfrR {
        NmicfrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SWDTCFR"]
    #[inline(always)]
    pub fn swdtcfr(&self) -> SwdtcfrR {
        SwdtcfrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PVD1CFR"]
    #[inline(always)]
    pub fn pvd1cfr(&self) -> Pvd1cfrR {
        Pvd1cfrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PVD2CFR"]
    #[inline(always)]
    pub fn pvd2cfr(&self) -> Pvd2cfrR {
        Pvd2cfrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - desc XTALSTPCFR"]
    #[inline(always)]
    pub fn xtalstpcfr(&self) -> XtalstpcfrR {
        XtalstpcfrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - desc REPCFR"]
    #[inline(always)]
    pub fn repcfr(&self) -> RepcfrR {
        RepcfrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc RECCCFR"]
    #[inline(always)]
    pub fn recccfr(&self) -> RecccfrR {
        RecccfrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BUSMCFR"]
    #[inline(always)]
    pub fn busmcfr(&self) -> BusmcfrR {
        BusmcfrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc WDTCFR"]
    #[inline(always)]
    pub fn wdtcfr(&self) -> WdtcfrR {
        WdtcfrR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc NMICFR"]
    #[inline(always)]
    pub fn nmicfr(&mut self) -> NmicfrW<'_, NmicfrSpec> {
        NmicfrW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SWDTCFR"]
    #[inline(always)]
    pub fn swdtcfr(&mut self) -> SwdtcfrW<'_, NmicfrSpec> {
        SwdtcfrW::new(self, 1)
    }
    #[doc = "Bit 2 - desc PVD1CFR"]
    #[inline(always)]
    pub fn pvd1cfr(&mut self) -> Pvd1cfrW<'_, NmicfrSpec> {
        Pvd1cfrW::new(self, 2)
    }
    #[doc = "Bit 3 - desc PVD2CFR"]
    #[inline(always)]
    pub fn pvd2cfr(&mut self) -> Pvd2cfrW<'_, NmicfrSpec> {
        Pvd2cfrW::new(self, 3)
    }
    #[doc = "Bit 5 - desc XTALSTPCFR"]
    #[inline(always)]
    pub fn xtalstpcfr(&mut self) -> XtalstpcfrW<'_, NmicfrSpec> {
        XtalstpcfrW::new(self, 5)
    }
    #[doc = "Bit 8 - desc REPCFR"]
    #[inline(always)]
    pub fn repcfr(&mut self) -> RepcfrW<'_, NmicfrSpec> {
        RepcfrW::new(self, 8)
    }
    #[doc = "Bit 9 - desc RECCCFR"]
    #[inline(always)]
    pub fn recccfr(&mut self) -> RecccfrW<'_, NmicfrSpec> {
        RecccfrW::new(self, 9)
    }
    #[doc = "Bit 10 - desc BUSMCFR"]
    #[inline(always)]
    pub fn busmcfr(&mut self) -> BusmcfrW<'_, NmicfrSpec> {
        BusmcfrW::new(self, 10)
    }
    #[doc = "Bit 11 - desc WDTCFR"]
    #[inline(always)]
    pub fn wdtcfr(&mut self) -> WdtcfrW<'_, NmicfrSpec> {
        WdtcfrW::new(self, 11)
    }
}
#[doc = "desc NMICFR\n\nYou can [`read`](crate::Reg::read) this register and get [`nmicfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmicfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmicfrSpec;
impl crate::RegisterSpec for NmicfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmicfr::R`](R) reader structure"]
impl crate::Readable for NmicfrSpec {}
#[doc = "`write(|w| ..)` method takes [`nmicfr::W`](W) writer structure"]
impl crate::Writable for NmicfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NMICFR to value 0"]
impl crate::Resettable for NmicfrSpec {}
