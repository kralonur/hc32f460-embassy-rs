#[doc = "Register `SSTR2` reader"]
pub type R = crate::R<Sstr2Spec>;
#[doc = "Register `SSTR2` writer"]
pub type W = crate::W<Sstr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SSTR2\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstr2Spec;
impl crate::RegisterSpec for Sstr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstr2::R`](R) reader structure"]
impl crate::Readable for Sstr2Spec {}
#[doc = "`write(|w| ..)` method takes [`sstr2::W`](W) writer structure"]
impl crate::Writable for Sstr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTR2 to value 0x0b"]
impl crate::Resettable for Sstr2Spec {
    const RESET_VALUE: u8 = 0x0b;
}
