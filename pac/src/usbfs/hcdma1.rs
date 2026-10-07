#[doc = "Register `HCDMA1` reader"]
pub type R = crate::R<Hcdma1Spec>;
#[doc = "Register `HCDMA1` writer"]
pub type W = crate::W<Hcdma1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HCDMA1\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcdma1Spec;
impl crate::RegisterSpec for Hcdma1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma1::R`](R) reader structure"]
impl crate::Readable for Hcdma1Spec {}
#[doc = "`write(|w| ..)` method takes [`hcdma1::W`](W) writer structure"]
impl crate::Writable for Hcdma1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCDMA1 to value 0"]
impl crate::Resettable for Hcdma1Spec {}
