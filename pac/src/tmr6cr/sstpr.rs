#[doc = "Register `SSTPR` reader"]
pub type R = crate::R<SstprSpec>;
#[doc = "Register `SSTPR` writer"]
pub type W = crate::W<SstprSpec>;
#[doc = "Field `SSTP1` reader - desc SSTP1"]
pub type Sstp1R = crate::BitReader;
#[doc = "Field `SSTP1` writer - desc SSTP1"]
pub type Sstp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTP2` reader - desc SSTP2"]
pub type Sstp2R = crate::BitReader;
#[doc = "Field `SSTP2` writer - desc SSTP2"]
pub type Sstp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTP3` reader - desc SSTP3"]
pub type Sstp3R = crate::BitReader;
#[doc = "Field `SSTP3` writer - desc SSTP3"]
pub type Sstp3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SSTP1"]
    #[inline(always)]
    pub fn sstp1(&self) -> Sstp1R {
        Sstp1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SSTP2"]
    #[inline(always)]
    pub fn sstp2(&self) -> Sstp2R {
        Sstp2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SSTP3"]
    #[inline(always)]
    pub fn sstp3(&self) -> Sstp3R {
        Sstp3R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SSTP1"]
    #[inline(always)]
    pub fn sstp1(&mut self) -> Sstp1W<'_, SstprSpec> {
        Sstp1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc SSTP2"]
    #[inline(always)]
    pub fn sstp2(&mut self) -> Sstp2W<'_, SstprSpec> {
        Sstp2W::new(self, 1)
    }
    #[doc = "Bit 2 - desc SSTP3"]
    #[inline(always)]
    pub fn sstp3(&mut self) -> Sstp3W<'_, SstprSpec> {
        Sstp3W::new(self, 2)
    }
}
#[doc = "desc SSTPR\n\nYou can [`read`](crate::Reg::read) this register and get [`sstpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SstprSpec;
impl crate::RegisterSpec for SstprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstpr::R`](R) reader structure"]
impl crate::Readable for SstprSpec {}
#[doc = "`write(|w| ..)` method takes [`sstpr::W`](W) writer structure"]
impl crate::Writable for SstprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTPR to value 0"]
impl crate::Resettable for SstprSpec {}
