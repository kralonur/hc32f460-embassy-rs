#[doc = "Register `OCCRUH` reader"]
pub type R = crate::R<OccruhSpec>;
#[doc = "Register `OCCRUH` writer"]
pub type W = crate::W<OccruhSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc OCCRUH\n\nYou can [`read`](crate::Reg::read) this register and get [`occruh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occruh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OccruhSpec;
impl crate::RegisterSpec for OccruhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`occruh::R`](R) reader structure"]
impl crate::Readable for OccruhSpec {}
#[doc = "`write(|w| ..)` method takes [`occruh::W`](W) writer structure"]
impl crate::Writable for OccruhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCCRUH to value 0"]
impl crate::Resettable for OccruhSpec {}
