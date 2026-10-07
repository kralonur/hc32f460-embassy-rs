#[doc = "Register `DOEPDMA2` reader"]
pub type R = crate::R<Doepdma2Spec>;
#[doc = "Register `DOEPDMA2` writer"]
pub type W = crate::W<Doepdma2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DOEPDMA2\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepdma2Spec;
impl crate::RegisterSpec for Doepdma2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma2::R`](R) reader structure"]
impl crate::Readable for Doepdma2Spec {}
#[doc = "`write(|w| ..)` method takes [`doepdma2::W`](W) writer structure"]
impl crate::Writable for Doepdma2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPDMA2 to value 0"]
impl crate::Resettable for Doepdma2Spec {}
