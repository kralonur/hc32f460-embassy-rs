#[doc = "Register `PDARW` reader"]
pub type R = crate::R<PdarwSpec>;
#[doc = "Register `PDARW` writer"]
pub type W = crate::W<PdarwSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc PDARW\n\nYou can [`read`](crate::Reg::read) this register and get [`pdarw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdarw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdarwSpec;
impl crate::RegisterSpec for PdarwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdarw::R`](R) reader structure"]
impl crate::Readable for PdarwSpec {}
#[doc = "`write(|w| ..)` method takes [`pdarw::W`](W) writer structure"]
impl crate::Writable for PdarwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDARW to value 0"]
impl crate::Resettable for PdarwSpec {}
