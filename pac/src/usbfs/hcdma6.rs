#[doc = "Register `HCDMA6` reader"]
pub type R = crate::R<Hcdma6Spec>;
#[doc = "Register `HCDMA6` writer"]
pub type W = crate::W<Hcdma6Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HCDMA6\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcdma6Spec;
impl crate::RegisterSpec for Hcdma6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma6::R`](R) reader structure"]
impl crate::Readable for Hcdma6Spec {}
#[doc = "`write(|w| ..)` method takes [`hcdma6::W`](W) writer structure"]
impl crate::Writable for Hcdma6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCDMA6 to value 0"]
impl crate::Resettable for Hcdma6Spec {}
