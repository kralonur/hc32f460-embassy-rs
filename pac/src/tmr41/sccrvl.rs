#[doc = "Register `SCCRVL` reader"]
pub type R = crate::R<SccrvlSpec>;
#[doc = "Register `SCCRVL` writer"]
pub type W = crate::W<SccrvlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SCCRVL\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrvl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SccrvlSpec;
impl crate::RegisterSpec for SccrvlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sccrvl::R`](R) reader structure"]
impl crate::Readable for SccrvlSpec {}
#[doc = "`write(|w| ..)` method takes [`sccrvl::W`](W) writer structure"]
impl crate::Writable for SccrvlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCCRVL to value 0"]
impl crate::Resettable for SccrvlSpec {}
