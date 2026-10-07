#[doc = "Register `LRCTRM` reader"]
pub type R = crate::R<LrctrmSpec>;
#[doc = "Register `LRCTRM` writer"]
pub type W = crate::W<LrctrmSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc LRCTRM\n\nYou can [`read`](crate::Reg::read) this register and get [`lrctrm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lrctrm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LrctrmSpec;
impl crate::RegisterSpec for LrctrmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lrctrm::R`](R) reader structure"]
impl crate::Readable for LrctrmSpec {}
#[doc = "`write(|w| ..)` method takes [`lrctrm::W`](W) writer structure"]
impl crate::Writable for LrctrmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LRCTRM to value 0"]
impl crate::Resettable for LrctrmSpec {}
