#[doc = "Register `KR2` reader"]
pub type R = crate::R<Kr2Spec>;
#[doc = "Register `KR2` writer"]
pub type W = crate::W<Kr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc KR2\n\nYou can [`read`](crate::Reg::read) this register and get [`kr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Kr2Spec;
impl crate::RegisterSpec for Kr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`kr2::R`](R) reader structure"]
impl crate::Readable for Kr2Spec {}
#[doc = "`write(|w| ..)` method takes [`kr2::W`](W) writer structure"]
impl crate::Writable for Kr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KR2 to value 0"]
impl crate::Resettable for Kr2Spec {}
