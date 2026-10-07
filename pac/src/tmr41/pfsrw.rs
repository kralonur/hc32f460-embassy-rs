#[doc = "Register `PFSRW` reader"]
pub type R = crate::R<PfsrwSpec>;
#[doc = "Register `PFSRW` writer"]
pub type W = crate::W<PfsrwSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc PFSRW\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfsrwSpec;
impl crate::RegisterSpec for PfsrwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pfsrw::R`](R) reader structure"]
impl crate::Readable for PfsrwSpec {}
#[doc = "`write(|w| ..)` method takes [`pfsrw::W`](W) writer structure"]
impl crate::Writable for PfsrwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PFSRW to value 0"]
impl crate::Resettable for PfsrwSpec {}
