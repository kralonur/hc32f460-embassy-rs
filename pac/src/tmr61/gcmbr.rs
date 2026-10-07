#[doc = "Register `GCMBR` reader"]
pub type R = crate::R<GcmbrSpec>;
#[doc = "Register `GCMBR` writer"]
pub type W = crate::W<GcmbrSpec>;
#[doc = "Field `GCMB` reader - desc GCMB"]
pub type GcmbR = crate::FieldReader<u16>;
#[doc = "Field `GCMB` writer - desc GCMB"]
pub type GcmbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc GCMB"]
    #[inline(always)]
    pub fn gcmb(&self) -> GcmbR {
        GcmbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc GCMB"]
    #[inline(always)]
    pub fn gcmb(&mut self) -> GcmbW<'_, GcmbrSpec> {
        GcmbW::new(self, 0)
    }
}
#[doc = "desc GCMBR\n\nYou can [`read`](crate::Reg::read) this register and get [`gcmbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcmbrSpec;
impl crate::RegisterSpec for GcmbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcmbr::R`](R) reader structure"]
impl crate::Readable for GcmbrSpec {}
#[doc = "`write(|w| ..)` method takes [`gcmbr::W`](W) writer structure"]
impl crate::Writable for GcmbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCMBR to value 0xffff"]
impl crate::Resettable for GcmbrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
