#[doc = "Register `CNTR` reader"]
pub type R = crate::R<CntrSpec>;
#[doc = "Register `CNTR` writer"]
pub type W = crate::W<CntrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc CNTR\n\nYou can [`read`](crate::Reg::read) this register and get [`cntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntrSpec;
impl crate::RegisterSpec for CntrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cntr::R`](R) reader structure"]
impl crate::Readable for CntrSpec {}
#[doc = "`write(|w| ..)` method takes [`cntr::W`](W) writer structure"]
impl crate::Writable for CntrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNTR to value 0"]
impl crate::Resettable for CntrSpec {}
