#[doc = "Register `DR14` reader"]
pub type R = crate::R<Dr14Spec>;
#[doc = "Register `DR14` writer"]
pub type W = crate::W<Dr14Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DR14\n\nYou can [`read`](crate::Reg::read) this register and get [`dr14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr14Spec;
impl crate::RegisterSpec for Dr14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr14::R`](R) reader structure"]
impl crate::Readable for Dr14Spec {}
#[doc = "`write(|w| ..)` method takes [`dr14::W`](W) writer structure"]
impl crate::Writable for Dr14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR14 to value 0"]
impl crate::Resettable for Dr14Spec {}
