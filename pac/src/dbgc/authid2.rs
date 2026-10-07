#[doc = "Register `AUTHID2` reader"]
pub type R = crate::R<Authid2Spec>;
#[doc = "Register `AUTHID2` writer"]
pub type W = crate::W<Authid2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc AUTHID2\n\nYou can [`read`](crate::Reg::read) this register and get [`authid2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`authid2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Authid2Spec;
impl crate::RegisterSpec for Authid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`authid2::R`](R) reader structure"]
impl crate::Readable for Authid2Spec {}
#[doc = "`write(|w| ..)` method takes [`authid2::W`](W) writer structure"]
impl crate::Writable for Authid2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUTHID2 to value 0"]
impl crate::Resettable for Authid2Spec {}
