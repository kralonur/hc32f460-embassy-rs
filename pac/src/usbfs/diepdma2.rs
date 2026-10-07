#[doc = "Register `DIEPDMA2` reader"]
pub type R = crate::R<Diepdma2Spec>;
#[doc = "Register `DIEPDMA2` writer"]
pub type W = crate::W<Diepdma2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DIEPDMA2\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diepdma2Spec;
impl crate::RegisterSpec for Diepdma2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdma2::R`](R) reader structure"]
impl crate::Readable for Diepdma2Spec {}
#[doc = "`write(|w| ..)` method takes [`diepdma2::W`](W) writer structure"]
impl crate::Writable for Diepdma2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPDMA2 to value 0"]
impl crate::Resettable for Diepdma2Spec {}
