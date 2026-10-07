#[doc = "Register `OCCRVH` reader"]
pub type R = crate::R<OccrvhSpec>;
#[doc = "Register `OCCRVH` writer"]
pub type W = crate::W<OccrvhSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc OCCRVH\n\nYou can [`read`](crate::Reg::read) this register and get [`occrvh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrvh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OccrvhSpec;
impl crate::RegisterSpec for OccrvhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`occrvh::R`](R) reader structure"]
impl crate::Readable for OccrvhSpec {}
#[doc = "`write(|w| ..)` method takes [`occrvh::W`](W) writer structure"]
impl crate::Writable for OccrvhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCCRVH to value 0"]
impl crate::Resettable for OccrvhSpec {}
