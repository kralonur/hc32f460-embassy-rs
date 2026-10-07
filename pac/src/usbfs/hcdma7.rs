#[doc = "Register `HCDMA7` reader"]
pub type R = crate::R<Hcdma7Spec>;
#[doc = "Register `HCDMA7` writer"]
pub type W = crate::W<Hcdma7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HCDMA7\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcdma7Spec;
impl crate::RegisterSpec for Hcdma7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma7::R`](R) reader structure"]
impl crate::Readable for Hcdma7Spec {}
#[doc = "`write(|w| ..)` method takes [`hcdma7::W`](W) writer structure"]
impl crate::Writable for Hcdma7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCDMA7 to value 0"]
impl crate::Resettable for Hcdma7Spec {}
