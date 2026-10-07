#[doc = "Register `DAR3` reader"]
pub type R = crate::R<Dar3Spec>;
#[doc = "Register `DAR3` writer"]
pub type W = crate::W<Dar3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DAR3\n\nYou can [`read`](crate::Reg::read) this register and get [`dar3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dar3Spec;
impl crate::RegisterSpec for Dar3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar3::R`](R) reader structure"]
impl crate::Readable for Dar3Spec {}
#[doc = "`write(|w| ..)` method takes [`dar3::W`](W) writer structure"]
impl crate::Writable for Dar3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAR3 to value 0"]
impl crate::Resettable for Dar3Spec {}
