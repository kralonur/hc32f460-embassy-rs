#[doc = "Register `AUTHID0` reader"]
pub type R = crate::R<Authid0Spec>;
#[doc = "Register `AUTHID0` writer"]
pub type W = crate::W<Authid0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc AUTHID0\n\nYou can [`read`](crate::Reg::read) this register and get [`authid0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`authid0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Authid0Spec;
impl crate::RegisterSpec for Authid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`authid0::R`](R) reader structure"]
impl crate::Readable for Authid0Spec {}
#[doc = "`write(|w| ..)` method takes [`authid0::W`](W) writer structure"]
impl crate::Writable for Authid0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUTHID0 to value 0"]
impl crate::Resettable for Authid0Spec {}
