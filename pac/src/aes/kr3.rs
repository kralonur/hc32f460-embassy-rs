#[doc = "Register `KR3` reader"]
pub type R = crate::R<Kr3Spec>;
#[doc = "Register `KR3` writer"]
pub type W = crate::W<Kr3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc KR3\n\nYou can [`read`](crate::Reg::read) this register and get [`kr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Kr3Spec;
impl crate::RegisterSpec for Kr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`kr3::R`](R) reader structure"]
impl crate::Readable for Kr3Spec {}
#[doc = "`write(|w| ..)` method takes [`kr3::W`](W) writer structure"]
impl crate::Writable for Kr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KR3 to value 0"]
impl crate::Resettable for Kr3Spec {}
