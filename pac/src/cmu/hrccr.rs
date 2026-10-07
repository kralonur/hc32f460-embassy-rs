#[doc = "Register `HRCCR` reader"]
pub type R = crate::R<HrccrSpec>;
#[doc = "Register `HRCCR` writer"]
pub type W = crate::W<HrccrSpec>;
#[doc = "Field `HRCSTP` reader - desc HRCSTP"]
pub type HrcstpR = crate::BitReader;
#[doc = "Field `HRCSTP` writer - desc HRCSTP"]
pub type HrcstpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HRCSTP"]
    #[inline(always)]
    pub fn hrcstp(&self) -> HrcstpR {
        HrcstpR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HRCSTP"]
    #[inline(always)]
    pub fn hrcstp(&mut self) -> HrcstpW<'_, HrccrSpec> {
        HrcstpW::new(self, 0)
    }
}
#[doc = "desc HRCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`hrccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrccrSpec;
impl crate::RegisterSpec for HrccrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hrccr::R`](R) reader structure"]
impl crate::Readable for HrccrSpec {}
#[doc = "`write(|w| ..)` method takes [`hrccr::W`](W) writer structure"]
impl crate::Writable for HrccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HRCCR to value 0x01"]
impl crate::Resettable for HrccrSpec {
    const RESET_VALUE: u8 = 0x01;
}
