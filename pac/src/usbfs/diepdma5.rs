#[doc = "Register `DIEPDMA5` reader"]
pub type R = crate::R<Diepdma5Spec>;
#[doc = "Register `DIEPDMA5` writer"]
pub type W = crate::W<Diepdma5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DIEPDMA5\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diepdma5Spec;
impl crate::RegisterSpec for Diepdma5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdma5::R`](R) reader structure"]
impl crate::Readable for Diepdma5Spec {}
#[doc = "`write(|w| ..)` method takes [`diepdma5::W`](W) writer structure"]
impl crate::Writable for Diepdma5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPDMA5 to value 0"]
impl crate::Resettable for Diepdma5Spec {}
