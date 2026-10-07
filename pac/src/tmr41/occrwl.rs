#[doc = "Register `OCCRWL` reader"]
pub type R = crate::R<OccrwlSpec>;
#[doc = "Register `OCCRWL` writer"]
pub type W = crate::W<OccrwlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc OCCRWL\n\nYou can [`read`](crate::Reg::read) this register and get [`occrwl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrwl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OccrwlSpec;
impl crate::RegisterSpec for OccrwlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`occrwl::R`](R) reader structure"]
impl crate::Readable for OccrwlSpec {}
#[doc = "`write(|w| ..)` method takes [`occrwl::W`](W) writer structure"]
impl crate::Writable for OccrwlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCCRWL to value 0"]
impl crate::Resettable for OccrwlSpec {}
