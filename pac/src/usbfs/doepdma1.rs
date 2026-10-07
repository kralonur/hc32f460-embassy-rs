#[doc = "Register `DOEPDMA1` reader"]
pub type R = crate::R<Doepdma1Spec>;
#[doc = "Register `DOEPDMA1` writer"]
pub type W = crate::W<Doepdma1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DOEPDMA1\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepdma1Spec;
impl crate::RegisterSpec for Doepdma1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma1::R`](R) reader structure"]
impl crate::Readable for Doepdma1Spec {}
#[doc = "`write(|w| ..)` method takes [`doepdma1::W`](W) writer structure"]
impl crate::Writable for Doepdma1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPDMA1 to value 0"]
impl crate::Resettable for Doepdma1Spec {}
