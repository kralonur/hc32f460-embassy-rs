#[doc = "Register `MRCTRM` reader"]
pub type R = crate::R<MrctrmSpec>;
#[doc = "Register `MRCTRM` writer"]
pub type W = crate::W<MrctrmSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc MRCTRM\n\nYou can [`read`](crate::Reg::read) this register and get [`mrctrm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrctrm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrctrmSpec;
impl crate::RegisterSpec for MrctrmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mrctrm::R`](R) reader structure"]
impl crate::Readable for MrctrmSpec {}
#[doc = "`write(|w| ..)` method takes [`mrctrm::W`](W) writer structure"]
impl crate::Writable for MrctrmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MRCTRM to value 0"]
impl crate::Resettable for MrctrmSpec {}
