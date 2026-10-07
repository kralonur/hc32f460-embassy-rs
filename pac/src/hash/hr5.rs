#[doc = "Register `HR5` reader"]
pub type R = crate::R<Hr5Spec>;
#[doc = "Register `HR5` writer"]
pub type W = crate::W<Hr5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HR5\n\nYou can [`read`](crate::Reg::read) this register and get [`hr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hr5Spec;
impl crate::RegisterSpec for Hr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr5::R`](R) reader structure"]
impl crate::Readable for Hr5Spec {}
#[doc = "`write(|w| ..)` method takes [`hr5::W`](W) writer structure"]
impl crate::Writable for Hr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HR5 to value 0"]
impl crate::Resettable for Hr5Spec {}
