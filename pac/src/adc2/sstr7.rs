#[doc = "Register `SSTR7` reader"]
pub type R = crate::R<Sstr7Spec>;
#[doc = "Register `SSTR7` writer"]
pub type W = crate::W<Sstr7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SSTR7\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstr7Spec;
impl crate::RegisterSpec for Sstr7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstr7::R`](R) reader structure"]
impl crate::Readable for Sstr7Spec {}
#[doc = "`write(|w| ..)` method takes [`sstr7::W`](W) writer structure"]
impl crate::Writable for Sstr7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTR7 to value 0x0b"]
impl crate::Resettable for Sstr7Spec {
    const RESET_VALUE: u8 = 0x0b;
}
