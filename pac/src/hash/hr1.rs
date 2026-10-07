#[doc = "Register `HR1` reader"]
pub type R = crate::R<Hr1Spec>;
#[doc = "Register `HR1` writer"]
pub type W = crate::W<Hr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HR1\n\nYou can [`read`](crate::Reg::read) this register and get [`hr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hr1Spec;
impl crate::RegisterSpec for Hr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr1::R`](R) reader structure"]
impl crate::Readable for Hr1Spec {}
#[doc = "`write(|w| ..)` method takes [`hr1::W`](W) writer structure"]
impl crate::Writable for Hr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HR1 to value 0"]
impl crate::Resettable for Hr1Spec {}
