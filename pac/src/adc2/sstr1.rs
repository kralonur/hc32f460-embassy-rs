#[doc = "Register `SSTR1` reader"]
pub type R = crate::R<Sstr1Spec>;
#[doc = "Register `SSTR1` writer"]
pub type W = crate::W<Sstr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SSTR1\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstr1Spec;
impl crate::RegisterSpec for Sstr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstr1::R`](R) reader structure"]
impl crate::Readable for Sstr1Spec {}
#[doc = "`write(|w| ..)` method takes [`sstr1::W`](W) writer structure"]
impl crate::Writable for Sstr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTR1 to value 0x0b"]
impl crate::Resettable for Sstr1Spec {
    const RESET_VALUE: u8 = 0x0b;
}
