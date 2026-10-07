#[doc = "Register `HR0` reader"]
pub type R = crate::R<Hr0Spec>;
#[doc = "Register `HR0` writer"]
pub type W = crate::W<Hr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HR0\n\nYou can [`read`](crate::Reg::read) this register and get [`hr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hr0Spec;
impl crate::RegisterSpec for Hr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr0::R`](R) reader structure"]
impl crate::Readable for Hr0Spec {}
#[doc = "`write(|w| ..)` method takes [`hr0::W`](W) writer structure"]
impl crate::Writable for Hr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HR0 to value 0"]
impl crate::Resettable for Hr0Spec {}
