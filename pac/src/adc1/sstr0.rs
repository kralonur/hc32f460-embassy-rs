#[doc = "Register `SSTR0` reader"]
pub type R = crate::R<Sstr0Spec>;
#[doc = "Register `SSTR0` writer"]
pub type W = crate::W<Sstr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SSTR0\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstr0Spec;
impl crate::RegisterSpec for Sstr0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstr0::R`](R) reader structure"]
impl crate::Readable for Sstr0Spec {}
#[doc = "`write(|w| ..)` method takes [`sstr0::W`](W) writer structure"]
impl crate::Writable for Sstr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTR0 to value 0x0b"]
impl crate::Resettable for Sstr0Spec {
    const RESET_VALUE: u8 = 0x0b;
}
