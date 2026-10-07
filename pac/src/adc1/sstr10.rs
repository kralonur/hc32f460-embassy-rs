#[doc = "Register `SSTR10` reader"]
pub type R = crate::R<Sstr10Spec>;
#[doc = "Register `SSTR10` writer"]
pub type W = crate::W<Sstr10Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SSTR10\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstr10Spec;
impl crate::RegisterSpec for Sstr10Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstr10::R`](R) reader structure"]
impl crate::Readable for Sstr10Spec {}
#[doc = "`write(|w| ..)` method takes [`sstr10::W`](W) writer structure"]
impl crate::Writable for Sstr10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTR10 to value 0x0b"]
impl crate::Resettable for Sstr10Spec {
    const RESET_VALUE: u8 = 0x0b;
}
