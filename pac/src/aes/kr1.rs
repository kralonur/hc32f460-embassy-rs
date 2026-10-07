#[doc = "Register `KR1` reader"]
pub type R = crate::R<Kr1Spec>;
#[doc = "Register `KR1` writer"]
pub type W = crate::W<Kr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc KR1\n\nYou can [`read`](crate::Reg::read) this register and get [`kr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Kr1Spec;
impl crate::RegisterSpec for Kr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`kr1::R`](R) reader structure"]
impl crate::Readable for Kr1Spec {}
#[doc = "`write(|w| ..)` method takes [`kr1::W`](W) writer structure"]
impl crate::Writable for Kr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KR1 to value 0"]
impl crate::Resettable for Kr1Spec {}
