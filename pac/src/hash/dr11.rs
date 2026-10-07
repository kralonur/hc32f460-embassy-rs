#[doc = "Register `DR11` reader"]
pub type R = crate::R<Dr11Spec>;
#[doc = "Register `DR11` writer"]
pub type W = crate::W<Dr11Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DR11\n\nYou can [`read`](crate::Reg::read) this register and get [`dr11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr11Spec;
impl crate::RegisterSpec for Dr11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr11::R`](R) reader structure"]
impl crate::Readable for Dr11Spec {}
#[doc = "`write(|w| ..)` method takes [`dr11::W`](W) writer structure"]
impl crate::Writable for Dr11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR11 to value 0"]
impl crate::Resettable for Dr11Spec {}
