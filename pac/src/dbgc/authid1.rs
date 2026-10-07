#[doc = "Register `AUTHID1` reader"]
pub type R = crate::R<Authid1Spec>;
#[doc = "Register `AUTHID1` writer"]
pub type W = crate::W<Authid1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc AUTHID1\n\nYou can [`read`](crate::Reg::read) this register and get [`authid1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`authid1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Authid1Spec;
impl crate::RegisterSpec for Authid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`authid1::R`](R) reader structure"]
impl crate::Readable for Authid1Spec {}
#[doc = "`write(|w| ..)` method takes [`authid1::W`](W) writer structure"]
impl crate::Writable for Authid1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUTHID1 to value 0"]
impl crate::Resettable for Authid1Spec {}
