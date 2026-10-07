#[doc = "Register `NMIFR` reader"]
pub type R = crate::R<NmifrSpec>;
#[doc = "Register `NMIFR` writer"]
pub type W = crate::W<NmifrSpec>;
#[doc = "Field `NMIFR` reader - desc NMIFR"]
pub type NmifrR = crate::BitReader;
#[doc = "Field `NMIFR` writer - desc NMIFR"]
pub type NmifrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDTFR` reader - desc SWDTFR"]
pub type SwdtfrR = crate::BitReader;
#[doc = "Field `SWDTFR` writer - desc SWDTFR"]
pub type SwdtfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD1FR` reader - desc PVD1FR"]
pub type Pvd1frR = crate::BitReader;
#[doc = "Field `PVD1FR` writer - desc PVD1FR"]
pub type Pvd1frW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2FR` reader - desc PVD2FR"]
pub type Pvd2frR = crate::BitReader;
#[doc = "Field `PVD2FR` writer - desc PVD2FR"]
pub type Pvd2frW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALSTPFR` reader - desc XTALSTPFR"]
pub type XtalstpfrR = crate::BitReader;
#[doc = "Field `XTALSTPFR` writer - desc XTALSTPFR"]
pub type XtalstpfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPFR` reader - desc REPFR"]
pub type RepfrR = crate::BitReader;
#[doc = "Field `REPFR` writer - desc REPFR"]
pub type RepfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECCFR` reader - desc RECCFR"]
pub type ReccfrR = crate::BitReader;
#[doc = "Field `RECCFR` writer - desc RECCFR"]
pub type ReccfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSMFR` reader - desc BUSMFR"]
pub type BusmfrR = crate::BitReader;
#[doc = "Field `BUSMFR` writer - desc BUSMFR"]
pub type BusmfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTFR` reader - desc WDTFR"]
pub type WdtfrR = crate::BitReader;
#[doc = "Field `WDTFR` writer - desc WDTFR"]
pub type WdtfrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc NMIFR"]
    #[inline(always)]
    pub fn nmifr(&self) -> NmifrR {
        NmifrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SWDTFR"]
    #[inline(always)]
    pub fn swdtfr(&self) -> SwdtfrR {
        SwdtfrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PVD1FR"]
    #[inline(always)]
    pub fn pvd1fr(&self) -> Pvd1frR {
        Pvd1frR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PVD2FR"]
    #[inline(always)]
    pub fn pvd2fr(&self) -> Pvd2frR {
        Pvd2frR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - desc XTALSTPFR"]
    #[inline(always)]
    pub fn xtalstpfr(&self) -> XtalstpfrR {
        XtalstpfrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - desc REPFR"]
    #[inline(always)]
    pub fn repfr(&self) -> RepfrR {
        RepfrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc RECCFR"]
    #[inline(always)]
    pub fn reccfr(&self) -> ReccfrR {
        ReccfrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BUSMFR"]
    #[inline(always)]
    pub fn busmfr(&self) -> BusmfrR {
        BusmfrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc WDTFR"]
    #[inline(always)]
    pub fn wdtfr(&self) -> WdtfrR {
        WdtfrR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc NMIFR"]
    #[inline(always)]
    pub fn nmifr(&mut self) -> NmifrW<'_, NmifrSpec> {
        NmifrW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SWDTFR"]
    #[inline(always)]
    pub fn swdtfr(&mut self) -> SwdtfrW<'_, NmifrSpec> {
        SwdtfrW::new(self, 1)
    }
    #[doc = "Bit 2 - desc PVD1FR"]
    #[inline(always)]
    pub fn pvd1fr(&mut self) -> Pvd1frW<'_, NmifrSpec> {
        Pvd1frW::new(self, 2)
    }
    #[doc = "Bit 3 - desc PVD2FR"]
    #[inline(always)]
    pub fn pvd2fr(&mut self) -> Pvd2frW<'_, NmifrSpec> {
        Pvd2frW::new(self, 3)
    }
    #[doc = "Bit 5 - desc XTALSTPFR"]
    #[inline(always)]
    pub fn xtalstpfr(&mut self) -> XtalstpfrW<'_, NmifrSpec> {
        XtalstpfrW::new(self, 5)
    }
    #[doc = "Bit 8 - desc REPFR"]
    #[inline(always)]
    pub fn repfr(&mut self) -> RepfrW<'_, NmifrSpec> {
        RepfrW::new(self, 8)
    }
    #[doc = "Bit 9 - desc RECCFR"]
    #[inline(always)]
    pub fn reccfr(&mut self) -> ReccfrW<'_, NmifrSpec> {
        ReccfrW::new(self, 9)
    }
    #[doc = "Bit 10 - desc BUSMFR"]
    #[inline(always)]
    pub fn busmfr(&mut self) -> BusmfrW<'_, NmifrSpec> {
        BusmfrW::new(self, 10)
    }
    #[doc = "Bit 11 - desc WDTFR"]
    #[inline(always)]
    pub fn wdtfr(&mut self) -> WdtfrW<'_, NmifrSpec> {
        WdtfrW::new(self, 11)
    }
}
#[doc = "desc NMIFR\n\nYou can [`read`](crate::Reg::read) this register and get [`nmifr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmifrSpec;
impl crate::RegisterSpec for NmifrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmifr::R`](R) reader structure"]
impl crate::Readable for NmifrSpec {}
#[doc = "`write(|w| ..)` method takes [`nmifr::W`](W) writer structure"]
impl crate::Writable for NmifrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NMIFR to value 0"]
impl crate::Resettable for NmifrSpec {}
