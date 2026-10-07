#[doc = "Register `LRCCR` reader"]
pub type R = crate::R<LrccrSpec>;
#[doc = "Register `LRCCR` writer"]
pub type W = crate::W<LrccrSpec>;
#[doc = "Field `LRCSTP` reader - desc LRCSTP"]
pub type LrcstpR = crate::BitReader;
#[doc = "Field `LRCSTP` writer - desc LRCSTP"]
pub type LrcstpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc LRCSTP"]
    #[inline(always)]
    pub fn lrcstp(&self) -> LrcstpR {
        LrcstpR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc LRCSTP"]
    #[inline(always)]
    pub fn lrcstp(&mut self) -> LrcstpW<'_, LrccrSpec> {
        LrcstpW::new(self, 0)
    }
}
#[doc = "desc LRCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`lrccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lrccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LrccrSpec;
impl crate::RegisterSpec for LrccrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lrccr::R`](R) reader structure"]
impl crate::Readable for LrccrSpec {}
#[doc = "`write(|w| ..)` method takes [`lrccr::W`](W) writer structure"]
impl crate::Writable for LrccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LRCCR to value 0"]
impl crate::Resettable for LrccrSpec {}
