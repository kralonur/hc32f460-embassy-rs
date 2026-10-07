#[doc = "Register `BUF1` reader"]
pub type R = crate::R<Buf1Spec>;
#[doc = "Register `BUF1` writer"]
pub type W = crate::W<Buf1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc BUF1\n\nYou can [`read`](crate::Reg::read) this register and get [`buf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf1Spec;
impl crate::RegisterSpec for Buf1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`buf1::R`](R) reader structure"]
impl crate::Readable for Buf1Spec {}
#[doc = "`write(|w| ..)` method takes [`buf1::W`](W) writer structure"]
impl crate::Writable for Buf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF1 to value 0"]
impl crate::Resettable for Buf1Spec {}
