#[doc = "Register `HR6` reader"]
pub type R = crate::R<Hr6Spec>;
#[doc = "Register `HR6` writer"]
pub type W = crate::W<Hr6Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HR6\n\nYou can [`read`](crate::Reg::read) this register and get [`hr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hr6Spec;
impl crate::RegisterSpec for Hr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr6::R`](R) reader structure"]
impl crate::Readable for Hr6Spec {}
#[doc = "`write(|w| ..)` method takes [`hr6::W`](W) writer structure"]
impl crate::Writable for Hr6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HR6 to value 0"]
impl crate::Resettable for Hr6Spec {}
