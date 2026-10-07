#[doc = "Register `SCLRR` reader"]
pub type R = crate::R<SclrrSpec>;
#[doc = "Register `SCLRR` writer"]
pub type W = crate::W<SclrrSpec>;
#[doc = "Field `SCLE1` reader - desc SCLE1"]
pub type Scle1R = crate::BitReader;
#[doc = "Field `SCLE1` writer - desc SCLE1"]
pub type Scle1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLE2` reader - desc SCLE2"]
pub type Scle2R = crate::BitReader;
#[doc = "Field `SCLE2` writer - desc SCLE2"]
pub type Scle2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLE3` reader - desc SCLE3"]
pub type Scle3R = crate::BitReader;
#[doc = "Field `SCLE3` writer - desc SCLE3"]
pub type Scle3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SCLE1"]
    #[inline(always)]
    pub fn scle1(&self) -> Scle1R {
        Scle1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SCLE2"]
    #[inline(always)]
    pub fn scle2(&self) -> Scle2R {
        Scle2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SCLE3"]
    #[inline(always)]
    pub fn scle3(&self) -> Scle3R {
        Scle3R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SCLE1"]
    #[inline(always)]
    pub fn scle1(&mut self) -> Scle1W<'_, SclrrSpec> {
        Scle1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc SCLE2"]
    #[inline(always)]
    pub fn scle2(&mut self) -> Scle2W<'_, SclrrSpec> {
        Scle2W::new(self, 1)
    }
    #[doc = "Bit 2 - desc SCLE3"]
    #[inline(always)]
    pub fn scle3(&mut self) -> Scle3W<'_, SclrrSpec> {
        Scle3W::new(self, 2)
    }
}
#[doc = "desc SCLRR\n\nYou can [`read`](crate::Reg::read) this register and get [`sclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclrrSpec;
impl crate::RegisterSpec for SclrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sclrr::R`](R) reader structure"]
impl crate::Readable for SclrrSpec {}
#[doc = "`write(|w| ..)` method takes [`sclrr::W`](W) writer structure"]
impl crate::Writable for SclrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCLRR to value 0"]
impl crate::Resettable for SclrrSpec {}
