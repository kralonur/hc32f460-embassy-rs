#[doc = "Register `HCDMA8` reader"]
pub type R = crate::R<Hcdma8Spec>;
#[doc = "Register `HCDMA8` writer"]
pub type W = crate::W<Hcdma8Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HCDMA8\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcdma8Spec;
impl crate::RegisterSpec for Hcdma8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma8::R`](R) reader structure"]
impl crate::Readable for Hcdma8Spec {}
#[doc = "`write(|w| ..)` method takes [`hcdma8::W`](W) writer structure"]
impl crate::Writable for Hcdma8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCDMA8 to value 0"]
impl crate::Resettable for Hcdma8Spec {}
