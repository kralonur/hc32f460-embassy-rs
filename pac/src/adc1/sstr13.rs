#[doc = "Register `SSTR13` reader"]
pub type R = crate::R<Sstr13Spec>;
#[doc = "Register `SSTR13` writer"]
pub type W = crate::W<Sstr13Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SSTR13\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstr13Spec;
impl crate::RegisterSpec for Sstr13Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstr13::R`](R) reader structure"]
impl crate::Readable for Sstr13Spec {}
#[doc = "`write(|w| ..)` method takes [`sstr13::W`](W) writer structure"]
impl crate::Writable for Sstr13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTR13 to value 0x0b"]
impl crate::Resettable for Sstr13Spec {
    const RESET_VALUE: u8 = 0x0b;
}
