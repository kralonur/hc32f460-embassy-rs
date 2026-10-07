#[doc = "Register `HR4` reader"]
pub type R = crate::R<Hr4Spec>;
#[doc = "Register `HR4` writer"]
pub type W = crate::W<Hr4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HR4\n\nYou can [`read`](crate::Reg::read) this register and get [`hr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hr4Spec;
impl crate::RegisterSpec for Hr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr4::R`](R) reader structure"]
impl crate::Readable for Hr4Spec {}
#[doc = "`write(|w| ..)` method takes [`hr4::W`](W) writer structure"]
impl crate::Writable for Hr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HR4 to value 0"]
impl crate::Resettable for Hr4Spec {}
