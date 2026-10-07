#[doc = "Register `GCMFR` reader"]
pub type R = crate::R<GcmfrSpec>;
#[doc = "Register `GCMFR` writer"]
pub type W = crate::W<GcmfrSpec>;
#[doc = "Field `GCMF` reader - desc GCMF"]
pub type GcmfR = crate::FieldReader<u16>;
#[doc = "Field `GCMF` writer - desc GCMF"]
pub type GcmfW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc GCMF"]
    #[inline(always)]
    pub fn gcmf(&self) -> GcmfR {
        GcmfR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc GCMF"]
    #[inline(always)]
    pub fn gcmf(&mut self) -> GcmfW<'_, GcmfrSpec> {
        GcmfW::new(self, 0)
    }
}
#[doc = "desc GCMFR\n\nYou can [`read`](crate::Reg::read) this register and get [`gcmfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcmfrSpec;
impl crate::RegisterSpec for GcmfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcmfr::R`](R) reader structure"]
impl crate::Readable for GcmfrSpec {}
#[doc = "`write(|w| ..)` method takes [`gcmfr::W`](W) writer structure"]
impl crate::Writable for GcmfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCMFR to value 0xffff"]
impl crate::Resettable for GcmfrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
