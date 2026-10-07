#[doc = "Register `DR15` reader"]
pub type R = crate::R<Dr15Spec>;
#[doc = "Register `DR15` writer"]
pub type W = crate::W<Dr15Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DR15\n\nYou can [`read`](crate::Reg::read) this register and get [`dr15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr15Spec;
impl crate::RegisterSpec for Dr15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr15::R`](R) reader structure"]
impl crate::Readable for Dr15Spec {}
#[doc = "`write(|w| ..)` method takes [`dr15::W`](W) writer structure"]
impl crate::Writable for Dr15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR15 to value 0"]
impl crate::Resettable for Dr15Spec {}
