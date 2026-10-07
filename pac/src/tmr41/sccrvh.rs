#[doc = "Register `SCCRVH` reader"]
pub type R = crate::R<SccrvhSpec>;
#[doc = "Register `SCCRVH` writer"]
pub type W = crate::W<SccrvhSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SCCRVH\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrvh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrvh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SccrvhSpec;
impl crate::RegisterSpec for SccrvhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sccrvh::R`](R) reader structure"]
impl crate::Readable for SccrvhSpec {}
#[doc = "`write(|w| ..)` method takes [`sccrvh::W`](W) writer structure"]
impl crate::Writable for SccrvhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCCRVH to value 0"]
impl crate::Resettable for SccrvhSpec {}
