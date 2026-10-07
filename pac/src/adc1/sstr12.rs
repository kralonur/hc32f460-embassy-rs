#[doc = "Register `SSTR12` reader"]
pub type R = crate::R<Sstr12Spec>;
#[doc = "Register `SSTR12` writer"]
pub type W = crate::W<Sstr12Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SSTR12\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstr12Spec;
impl crate::RegisterSpec for Sstr12Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstr12::R`](R) reader structure"]
impl crate::Readable for Sstr12Spec {}
#[doc = "`write(|w| ..)` method takes [`sstr12::W`](W) writer structure"]
impl crate::Writable for Sstr12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTR12 to value 0x0b"]
impl crate::Resettable for Sstr12Spec {
    const RESET_VALUE: u8 = 0x0b;
}
