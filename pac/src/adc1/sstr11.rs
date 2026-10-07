#[doc = "Register `SSTR11` reader"]
pub type R = crate::R<Sstr11Spec>;
#[doc = "Register `SSTR11` writer"]
pub type W = crate::W<Sstr11Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SSTR11\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstr11Spec;
impl crate::RegisterSpec for Sstr11Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstr11::R`](R) reader structure"]
impl crate::Readable for Sstr11Spec {}
#[doc = "`write(|w| ..)` method takes [`sstr11::W`](W) writer structure"]
impl crate::Writable for Sstr11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTR11 to value 0x0b"]
impl crate::Resettable for Sstr11Spec {
    const RESET_VALUE: u8 = 0x0b;
}
