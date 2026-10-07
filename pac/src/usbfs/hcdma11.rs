#[doc = "Register `HCDMA11` reader"]
pub type R = crate::R<Hcdma11Spec>;
#[doc = "Register `HCDMA11` writer"]
pub type W = crate::W<Hcdma11Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HCDMA11\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcdma11Spec;
impl crate::RegisterSpec for Hcdma11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma11::R`](R) reader structure"]
impl crate::Readable for Hcdma11Spec {}
#[doc = "`write(|w| ..)` method takes [`hcdma11::W`](W) writer structure"]
impl crate::Writable for Hcdma11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCDMA11 to value 0"]
impl crate::Resettable for Hcdma11Spec {}
