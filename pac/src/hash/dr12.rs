#[doc = "Register `DR12` reader"]
pub type R = crate::R<Dr12Spec>;
#[doc = "Register `DR12` writer"]
pub type W = crate::W<Dr12Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DR12\n\nYou can [`read`](crate::Reg::read) this register and get [`dr12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr12Spec;
impl crate::RegisterSpec for Dr12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr12::R`](R) reader structure"]
impl crate::Readable for Dr12Spec {}
#[doc = "`write(|w| ..)` method takes [`dr12::W`](W) writer structure"]
impl crate::Writable for Dr12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR12 to value 0"]
impl crate::Resettable for Dr12Spec {}
