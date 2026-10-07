#[doc = "Register `SSTR4` reader"]
pub type R = crate::R<Sstr4Spec>;
#[doc = "Register `SSTR4` writer"]
pub type W = crate::W<Sstr4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SSTR4\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstr4Spec;
impl crate::RegisterSpec for Sstr4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstr4::R`](R) reader structure"]
impl crate::Readable for Sstr4Spec {}
#[doc = "`write(|w| ..)` method takes [`sstr4::W`](W) writer structure"]
impl crate::Writable for Sstr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTR4 to value 0x0b"]
impl crate::Resettable for Sstr4Spec {
    const RESET_VALUE: u8 = 0x0b;
}
