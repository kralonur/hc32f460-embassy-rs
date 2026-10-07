#[doc = "Register `SSTR9` reader"]
pub type R = crate::R<Sstr9Spec>;
#[doc = "Register `SSTR9` writer"]
pub type W = crate::W<Sstr9Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SSTR9\n\nYou can [`read`](crate::Reg::read) this register and get [`sstr9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstr9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstr9Spec;
impl crate::RegisterSpec for Sstr9Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstr9::R`](R) reader structure"]
impl crate::Readable for Sstr9Spec {}
#[doc = "`write(|w| ..)` method takes [`sstr9::W`](W) writer structure"]
impl crate::Writable for Sstr9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSTR9 to value 0x0b"]
impl crate::Resettable for Sstr9Spec {
    const RESET_VALUE: u8 = 0x0b;
}
