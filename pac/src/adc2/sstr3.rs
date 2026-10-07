#[doc = "Register `SSTR3` reader"]
pub type R = crate::R<Sstr3Spec>;
#[doc = "Register `SSTR3` writer"]
pub type W = crate::W<Sstr3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SSTR3\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstr3Spec;
impl crate::RegisterSpec for Sstr3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstr3::R`](R) reader structure"]
impl crate::Readable for Sstr3Spec {}
#[doc = "`write(|w| ..)` method takes [`sstr3::W`](W) writer structure"]
impl crate::Writable for Sstr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTR3 to value 0x0b"]
impl crate::Resettable for Sstr3Spec {
    const RESET_VALUE: u8 = 0x0b;
}
