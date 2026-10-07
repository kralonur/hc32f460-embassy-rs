#[doc = "Register `HCDMA3` reader"]
pub type R = crate::R<Hcdma3Spec>;
#[doc = "Register `HCDMA3` writer"]
pub type W = crate::W<Hcdma3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HCDMA3\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcdma3Spec;
impl crate::RegisterSpec for Hcdma3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma3::R`](R) reader structure"]
impl crate::Readable for Hcdma3Spec {}
#[doc = "`write(|w| ..)` method takes [`hcdma3::W`](W) writer structure"]
impl crate::Writable for Hcdma3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCDMA3 to value 0"]
impl crate::Resettable for Hcdma3Spec {}
