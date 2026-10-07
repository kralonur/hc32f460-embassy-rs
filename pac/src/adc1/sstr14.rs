#[doc = "Register `SSTR14` reader"]
pub type R = crate::R<Sstr14Spec>;
#[doc = "Register `SSTR14` writer"]
pub type W = crate::W<Sstr14Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SSTR14\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstr14Spec;
impl crate::RegisterSpec for Sstr14Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstr14::R`](R) reader structure"]
impl crate::Readable for Sstr14Spec {}
#[doc = "`write(|w| ..)` method takes [`sstr14::W`](W) writer structure"]
impl crate::Writable for Sstr14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTR14 to value 0x0b"]
impl crate::Resettable for Sstr14Spec {
    const RESET_VALUE: u8 = 0x0b;
}
