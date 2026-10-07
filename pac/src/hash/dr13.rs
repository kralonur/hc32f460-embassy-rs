#[doc = "Register `DR13` reader"]
pub type R = crate::R<Dr13Spec>;
#[doc = "Register `DR13` writer"]
pub type W = crate::W<Dr13Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DR13\n\nYou can [`read`](crate::Reg::read) this register and get [`dr13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr13Spec;
impl crate::RegisterSpec for Dr13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr13::R`](R) reader structure"]
impl crate::Readable for Dr13Spec {}
#[doc = "`write(|w| ..)` method takes [`dr13::W`](W) writer structure"]
impl crate::Writable for Dr13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR13 to value 0"]
impl crate::Resettable for Dr13Spec {}
