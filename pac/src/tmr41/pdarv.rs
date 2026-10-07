#[doc = "Register `PDARV` reader"]
pub type R = crate::R<PdarvSpec>;
#[doc = "Register `PDARV` writer"]
pub type W = crate::W<PdarvSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc PDARV\n\nYou can [`read`](crate::Reg::read) this register and get [`pdarv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdarv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdarvSpec;
impl crate::RegisterSpec for PdarvSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdarv::R`](R) reader structure"]
impl crate::Readable for PdarvSpec {}
#[doc = "`write(|w| ..)` method takes [`pdarv::W`](W) writer structure"]
impl crate::Writable for PdarvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDARV to value 0"]
impl crate::Resettable for PdarvSpec {}
