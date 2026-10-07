#[doc = "Register `DR8` reader"]
pub type R = crate::R<Dr8Spec>;
#[doc = "Register `DR8` writer"]
pub type W = crate::W<Dr8Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DR8\n\nYou can [`read`](crate::Reg::read) this register and get [`dr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr8Spec;
impl crate::RegisterSpec for Dr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr8::R`](R) reader structure"]
impl crate::Readable for Dr8Spec {}
#[doc = "`write(|w| ..)` method takes [`dr8::W`](W) writer structure"]
impl crate::Writable for Dr8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR8 to value 0"]
impl crate::Resettable for Dr8Spec {}
