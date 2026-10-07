#[doc = "Register `DOEPDMA3` reader"]
pub type R = crate::R<Doepdma3Spec>;
#[doc = "Register `DOEPDMA3` writer"]
pub type W = crate::W<Doepdma3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DOEPDMA3\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepdma3Spec;
impl crate::RegisterSpec for Doepdma3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma3::R`](R) reader structure"]
impl crate::Readable for Doepdma3Spec {}
#[doc = "`write(|w| ..)` method takes [`doepdma3::W`](W) writer structure"]
impl crate::Writable for Doepdma3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPDMA3 to value 0"]
impl crate::Resettable for Doepdma3Spec {}
