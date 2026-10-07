#[doc = "Register `DR4` reader"]
pub type R = crate::R<Dr4Spec>;
#[doc = "Register `DR4` writer"]
pub type W = crate::W<Dr4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DR4\n\nYou can [`read`](crate::Reg::read) this register and get [`dr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr4Spec;
impl crate::RegisterSpec for Dr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr4::R`](R) reader structure"]
impl crate::Readable for Dr4Spec {}
#[doc = "`write(|w| ..)` method takes [`dr4::W`](W) writer structure"]
impl crate::Writable for Dr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR4 to value 0"]
impl crate::Resettable for Dr4Spec {}
