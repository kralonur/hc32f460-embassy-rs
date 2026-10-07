#[doc = "Register `KR0` reader"]
pub type R = crate::R<Kr0Spec>;
#[doc = "Register `KR0` writer"]
pub type W = crate::W<Kr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc KR0\n\nYou can [`read`](crate::Reg::read) this register and get [`kr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Kr0Spec;
impl crate::RegisterSpec for Kr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`kr0::R`](R) reader structure"]
impl crate::Readable for Kr0Spec {}
#[doc = "`write(|w| ..)` method takes [`kr0::W`](W) writer structure"]
impl crate::Writable for Kr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KR0 to value 0"]
impl crate::Resettable for Kr0Spec {}
