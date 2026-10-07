#[doc = "Register `DOEPDMA5` reader"]
pub type R = crate::R<Doepdma5Spec>;
#[doc = "Register `DOEPDMA5` writer"]
pub type W = crate::W<Doepdma5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DOEPDMA5\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepdma5Spec;
impl crate::RegisterSpec for Doepdma5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma5::R`](R) reader structure"]
impl crate::Readable for Doepdma5Spec {}
#[doc = "`write(|w| ..)` method takes [`doepdma5::W`](W) writer structure"]
impl crate::Writable for Doepdma5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPDMA5 to value 0"]
impl crate::Resettable for Doepdma5Spec {}
