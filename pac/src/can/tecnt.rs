#[doc = "Register `TECNT` reader"]
pub type R = crate::R<TecntSpec>;
#[doc = "Register `TECNT` writer"]
pub type W = crate::W<TecntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc TECNT\n\nYou can [`read`](crate::Reg::read) this register and get [`tecnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tecnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TecntSpec;
impl crate::RegisterSpec for TecntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tecnt::R`](R) reader structure"]
impl crate::Readable for TecntSpec {}
#[doc = "`write(|w| ..)` method takes [`tecnt::W`](W) writer structure"]
impl crate::Writable for TecntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TECNT to value 0"]
impl crate::Resettable for TecntSpec {}
