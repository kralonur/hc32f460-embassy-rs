#[doc = "Register `HCDMA5` reader"]
pub type R = crate::R<Hcdma5Spec>;
#[doc = "Register `HCDMA5` writer"]
pub type W = crate::W<Hcdma5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HCDMA5\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcdma5Spec;
impl crate::RegisterSpec for Hcdma5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma5::R`](R) reader structure"]
impl crate::Readable for Hcdma5Spec {}
#[doc = "`write(|w| ..)` method takes [`hcdma5::W`](W) writer structure"]
impl crate::Writable for Hcdma5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCDMA5 to value 0"]
impl crate::Resettable for Hcdma5Spec {}
