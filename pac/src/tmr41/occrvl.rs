#[doc = "Register `OCCRVL` reader"]
pub type R = crate::R<OccrvlSpec>;
#[doc = "Register `OCCRVL` writer"]
pub type W = crate::W<OccrvlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc OCCRVL\n\nYou can [`read`](crate::Reg::read) this register and get [`occrvl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OccrvlSpec;
impl crate::RegisterSpec for OccrvlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`occrvl::R`](R) reader structure"]
impl crate::Readable for OccrvlSpec {}
#[doc = "`write(|w| ..)` method takes [`occrvl::W`](W) writer structure"]
impl crate::Writable for OccrvlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCCRVL to value 0"]
impl crate::Resettable for OccrvlSpec {}
