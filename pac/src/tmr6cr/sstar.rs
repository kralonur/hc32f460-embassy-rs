#[doc = "Register `SSTAR` reader"]
pub type R = crate::R<SstarSpec>;
#[doc = "Register `SSTAR` writer"]
pub type W = crate::W<SstarSpec>;
#[doc = "Field `SSTA1` reader - desc SSTA1"]
pub type Ssta1R = crate::BitReader;
#[doc = "Field `SSTA1` writer - desc SSTA1"]
pub type Ssta1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTA2` reader - desc SSTA2"]
pub type Ssta2R = crate::BitReader;
#[doc = "Field `SSTA2` writer - desc SSTA2"]
pub type Ssta2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTA3` reader - desc SSTA3"]
pub type Ssta3R = crate::BitReader;
#[doc = "Field `SSTA3` writer - desc SSTA3"]
pub type Ssta3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SSTA1"]
    #[inline(always)]
    pub fn ssta1(&self) -> Ssta1R {
        Ssta1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SSTA2"]
    #[inline(always)]
    pub fn ssta2(&self) -> Ssta2R {
        Ssta2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SSTA3"]
    #[inline(always)]
    pub fn ssta3(&self) -> Ssta3R {
        Ssta3R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SSTA1"]
    #[inline(always)]
    pub fn ssta1(&mut self) -> Ssta1W<'_, SstarSpec> {
        Ssta1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc SSTA2"]
    #[inline(always)]
    pub fn ssta2(&mut self) -> Ssta2W<'_, SstarSpec> {
        Ssta2W::new(self, 1)
    }
    #[doc = "Bit 2 - desc SSTA3"]
    #[inline(always)]
    pub fn ssta3(&mut self) -> Ssta3W<'_, SstarSpec> {
        Ssta3W::new(self, 2)
    }
}
#[doc = "desc SSTAR\n\nYou can [`read`](crate::Reg::read) this register and get [`sstar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SstarSpec;
impl crate::RegisterSpec for SstarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstar::R`](R) reader structure"]
impl crate::Readable for SstarSpec {}
#[doc = "`write(|w| ..)` method takes [`sstar::W`](W) writer structure"]
impl crate::Writable for SstarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTAR to value 0"]
impl crate::Resettable for SstarSpec {}
