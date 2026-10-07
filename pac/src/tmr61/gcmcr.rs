#[doc = "Register `GCMCR` reader"]
pub type R = crate::R<GcmcrSpec>;
#[doc = "Register `GCMCR` writer"]
pub type W = crate::W<GcmcrSpec>;
#[doc = "Field `GCMC` reader - desc GCMC"]
pub type GcmcR = crate::FieldReader<u16>;
#[doc = "Field `GCMC` writer - desc GCMC"]
pub type GcmcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc GCMC"]
    #[inline(always)]
    pub fn gcmc(&self) -> GcmcR {
        GcmcR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc GCMC"]
    #[inline(always)]
    pub fn gcmc(&mut self) -> GcmcW<'_, GcmcrSpec> {
        GcmcW::new(self, 0)
    }
}
#[doc = "desc GCMCR\n\nYou can [`read`](crate::Reg::read) this register and get [`gcmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcmcrSpec;
impl crate::RegisterSpec for GcmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcmcr::R`](R) reader structure"]
impl crate::Readable for GcmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`gcmcr::W`](W) writer structure"]
impl crate::Writable for GcmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCMCR to value 0xffff"]
impl crate::Resettable for GcmcrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
