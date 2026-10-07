#[doc = "Register `DOEPDMA4` reader"]
pub type R = crate::R<Doepdma4Spec>;
#[doc = "Register `DOEPDMA4` writer"]
pub type W = crate::W<Doepdma4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DOEPDMA4\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepdma4Spec;
impl crate::RegisterSpec for Doepdma4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma4::R`](R) reader structure"]
impl crate::Readable for Doepdma4Spec {}
#[doc = "`write(|w| ..)` method takes [`doepdma4::W`](W) writer structure"]
impl crate::Writable for Doepdma4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPDMA4 to value 0"]
impl crate::Resettable for Doepdma4Spec {}
