#[doc = "Register `HRCTRM` reader"]
pub type R = crate::R<HrctrmSpec>;
#[doc = "Register `HRCTRM` writer"]
pub type W = crate::W<HrctrmSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc HRCTRM\n\nYou can [`read`](crate::Reg::read) this register and get [`hrctrm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrctrm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrctrmSpec;
impl crate::RegisterSpec for HrctrmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hrctrm::R`](R) reader structure"]
impl crate::Readable for HrctrmSpec {}
#[doc = "`write(|w| ..)` method takes [`hrctrm::W`](W) writer structure"]
impl crate::Writable for HrctrmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HRCTRM to value 0"]
impl crate::Resettable for HrctrmSpec {}
