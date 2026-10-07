#[doc = "Register `HR2` reader"]
pub type R = crate::R<Hr2Spec>;
#[doc = "Register `HR2` writer"]
pub type W = crate::W<Hr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HR2\n\nYou can [`read`](crate::Reg::read) this register and get [`hr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hr2Spec;
impl crate::RegisterSpec for Hr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr2::R`](R) reader structure"]
impl crate::Readable for Hr2Spec {}
#[doc = "`write(|w| ..)` method takes [`hr2::W`](W) writer structure"]
impl crate::Writable for Hr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HR2 to value 0"]
impl crate::Resettable for Hr2Spec {}
