#[doc = "Register `HCDMA10` reader"]
pub type R = crate::R<Hcdma10Spec>;
#[doc = "Register `HCDMA10` writer"]
pub type W = crate::W<Hcdma10Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HCDMA10\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcdma10Spec;
impl crate::RegisterSpec for Hcdma10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma10::R`](R) reader structure"]
impl crate::Readable for Hcdma10Spec {}
#[doc = "`write(|w| ..)` method takes [`hcdma10::W`](W) writer structure"]
impl crate::Writable for Hcdma10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCDMA10 to value 0"]
impl crate::Resettable for Hcdma10Spec {}
