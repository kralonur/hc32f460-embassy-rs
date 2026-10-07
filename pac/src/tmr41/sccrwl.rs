#[doc = "Register `SCCRWL` reader"]
pub type R = crate::R<SccrwlSpec>;
#[doc = "Register `SCCRWL` writer"]
pub type W = crate::W<SccrwlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SCCRWL\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrwl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrwl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SccrwlSpec;
impl crate::RegisterSpec for SccrwlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sccrwl::R`](R) reader structure"]
impl crate::Readable for SccrwlSpec {}
#[doc = "`write(|w| ..)` method takes [`sccrwl::W`](W) writer structure"]
impl crate::Writable for SccrwlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCCRWL to value 0"]
impl crate::Resettable for SccrwlSpec {}
