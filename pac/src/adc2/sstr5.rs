#[doc = "Register `SSTR5` reader"]
pub type R = crate::R<Sstr5Spec>;
#[doc = "Register `SSTR5` writer"]
pub type W = crate::W<Sstr5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SSTR5\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstr5Spec;
impl crate::RegisterSpec for Sstr5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstr5::R`](R) reader structure"]
impl crate::Readable for Sstr5Spec {}
#[doc = "`write(|w| ..)` method takes [`sstr5::W`](W) writer structure"]
impl crate::Writable for Sstr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTR5 to value 0x0b"]
impl crate::Resettable for Sstr5Spec {
    const RESET_VALUE: u8 = 0x0b;
}
