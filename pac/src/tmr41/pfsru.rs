#[doc = "Register `PFSRU` reader"]
pub type R = crate::R<PfsruSpec>;
#[doc = "Register `PFSRU` writer"]
pub type W = crate::W<PfsruSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc PFSRU\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfsruSpec;
impl crate::RegisterSpec for PfsruSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pfsru::R`](R) reader structure"]
impl crate::Readable for PfsruSpec {}
#[doc = "`write(|w| ..)` method takes [`pfsru::W`](W) writer structure"]
impl crate::Writable for PfsruSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PFSRU to value 0"]
impl crate::Resettable for PfsruSpec {}
