#[doc = "Register `DIEPDMA4` reader"]
pub type R = crate::R<Diepdma4Spec>;
#[doc = "Register `DIEPDMA4` writer"]
pub type W = crate::W<Diepdma4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DIEPDMA4\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diepdma4Spec;
impl crate::RegisterSpec for Diepdma4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdma4::R`](R) reader structure"]
impl crate::Readable for Diepdma4Spec {}
#[doc = "`write(|w| ..)` method takes [`diepdma4::W`](W) writer structure"]
impl crate::Writable for Diepdma4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPDMA4 to value 0"]
impl crate::Resettable for Diepdma4Spec {}
