#[doc = "Register `SAR1` reader"]
pub type R = crate::R<Sar1Spec>;
#[doc = "Register `SAR1` writer"]
pub type W = crate::W<Sar1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SAR1\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sar1Spec;
impl crate::RegisterSpec for Sar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar1::R`](R) reader structure"]
impl crate::Readable for Sar1Spec {}
#[doc = "`write(|w| ..)` method takes [`sar1::W`](W) writer structure"]
impl crate::Writable for Sar1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR1 to value 0"]
impl crate::Resettable for Sar1Spec {}
