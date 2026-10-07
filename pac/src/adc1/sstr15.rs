#[doc = "Register `SSTR15` reader"]
pub type R = crate::R<Sstr15Spec>;
#[doc = "Register `SSTR15` writer"]
pub type W = crate::W<Sstr15Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SSTR15\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstr15Spec;
impl crate::RegisterSpec for Sstr15Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstr15::R`](R) reader structure"]
impl crate::Readable for Sstr15Spec {}
#[doc = "`write(|w| ..)` method takes [`sstr15::W`](W) writer structure"]
impl crate::Writable for Sstr15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTR15 to value 0x0b"]
impl crate::Resettable for Sstr15Spec {
    const RESET_VALUE: u8 = 0x0b;
}
