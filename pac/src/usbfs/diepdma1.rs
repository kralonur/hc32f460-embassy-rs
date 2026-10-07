#[doc = "Register `DIEPDMA1` reader"]
pub type R = crate::R<Diepdma1Spec>;
#[doc = "Register `DIEPDMA1` writer"]
pub type W = crate::W<Diepdma1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DIEPDMA1\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diepdma1Spec;
impl crate::RegisterSpec for Diepdma1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdma1::R`](R) reader structure"]
impl crate::Readable for Diepdma1Spec {}
#[doc = "`write(|w| ..)` method takes [`diepdma1::W`](W) writer structure"]
impl crate::Writable for Diepdma1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPDMA1 to value 0"]
impl crate::Resettable for Diepdma1Spec {}
