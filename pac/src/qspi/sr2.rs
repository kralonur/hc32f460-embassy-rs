#[doc = "Register `SR2` writer"]
pub type W = crate::W<Sr2Spec>;
#[doc = "Field `RAERCLR` writer - desc RAERCLR"]
pub type RaerclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 7 - desc RAERCLR"]
    #[inline(always)]
    pub fn raerclr(&mut self) -> RaerclrW<'_, Sr2Spec> {
        RaerclrW::new(self, 7)
    }
}
#[doc = "desc SR2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr2Spec;
impl crate::RegisterSpec for Sr2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sr2::W`](W) writer structure"]
impl crate::Writable for Sr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for Sr2Spec {}
