#[doc = "Register `SCCRWH` reader"]
pub type R = crate::R<SccrwhSpec>;
#[doc = "Register `SCCRWH` writer"]
pub type W = crate::W<SccrwhSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SCCRWH\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrwh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrwh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SccrwhSpec;
impl crate::RegisterSpec for SccrwhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sccrwh::R`](R) reader structure"]
impl crate::Readable for SccrwhSpec {}
#[doc = "`write(|w| ..)` method takes [`sccrwh::W`](W) writer structure"]
impl crate::Writable for SccrwhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCCRWH to value 0"]
impl crate::Resettable for SccrwhSpec {}
