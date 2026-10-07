#[doc = "Register `SCCRUH` reader"]
pub type R = crate::R<SccruhSpec>;
#[doc = "Register `SCCRUH` writer"]
pub type W = crate::W<SccruhSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SCCRUH\n\nYou can [`read`](crate::Reg::read) this register and get [`sccruh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccruh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SccruhSpec;
impl crate::RegisterSpec for SccruhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sccruh::R`](R) reader structure"]
impl crate::Readable for SccruhSpec {}
#[doc = "`write(|w| ..)` method takes [`sccruh::W`](W) writer structure"]
impl crate::Writable for SccruhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCCRUH to value 0"]
impl crate::Resettable for SccruhSpec {}
