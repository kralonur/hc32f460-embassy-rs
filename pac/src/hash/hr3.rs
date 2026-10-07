#[doc = "Register `HR3` reader"]
pub type R = crate::R<Hr3Spec>;
#[doc = "Register `HR3` writer"]
pub type W = crate::W<Hr3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HR3\n\nYou can [`read`](crate::Reg::read) this register and get [`hr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hr3Spec;
impl crate::RegisterSpec for Hr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr3::R`](R) reader structure"]
impl crate::Readable for Hr3Spec {}
#[doc = "`write(|w| ..)` method takes [`hr3::W`](W) writer structure"]
impl crate::Writable for Hr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HR3 to value 0"]
impl crate::Resettable for Hr3Spec {}
