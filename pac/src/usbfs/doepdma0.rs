#[doc = "Register `DOEPDMA0` reader"]
pub type R = crate::R<Doepdma0Spec>;
#[doc = "Register `DOEPDMA0` writer"]
pub type W = crate::W<Doepdma0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DOEPDMA0\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepdma0Spec;
impl crate::RegisterSpec for Doepdma0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma0::R`](R) reader structure"]
impl crate::Readable for Doepdma0Spec {}
#[doc = "`write(|w| ..)` method takes [`doepdma0::W`](W) writer structure"]
impl crate::Writable for Doepdma0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPDMA0 to value 0"]
impl crate::Resettable for Doepdma0Spec {}
