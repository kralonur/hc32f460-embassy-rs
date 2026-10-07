#[doc = "Register `OCCRWH` reader"]
pub type R = crate::R<OccrwhSpec>;
#[doc = "Register `OCCRWH` writer"]
pub type W = crate::W<OccrwhSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc OCCRWH\n\nYou can [`read`](crate::Reg::read) this register and get [`occrwh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrwh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OccrwhSpec;
impl crate::RegisterSpec for OccrwhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`occrwh::R`](R) reader structure"]
impl crate::Readable for OccrwhSpec {}
#[doc = "`write(|w| ..)` method takes [`occrwh::W`](W) writer structure"]
impl crate::Writable for OccrwhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCCRWH to value 0"]
impl crate::Resettable for OccrwhSpec {}
