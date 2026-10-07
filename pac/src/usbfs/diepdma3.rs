#[doc = "Register `DIEPDMA3` reader"]
pub type R = crate::R<Diepdma3Spec>;
#[doc = "Register `DIEPDMA3` writer"]
pub type W = crate::W<Diepdma3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DIEPDMA3\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diepdma3Spec;
impl crate::RegisterSpec for Diepdma3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdma3::R`](R) reader structure"]
impl crate::Readable for Diepdma3Spec {}
#[doc = "`write(|w| ..)` method takes [`diepdma3::W`](W) writer structure"]
impl crate::Writable for Diepdma3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPDMA3 to value 0"]
impl crate::Resettable for Diepdma3Spec {}
