#[doc = "Register `GCMDR` reader"]
pub type R = crate::R<GcmdrSpec>;
#[doc = "Register `GCMDR` writer"]
pub type W = crate::W<GcmdrSpec>;
#[doc = "Field `GCMD` reader - desc GCMD"]
pub type GcmdR = crate::FieldReader<u16>;
#[doc = "Field `GCMD` writer - desc GCMD"]
pub type GcmdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc GCMD"]
    #[inline(always)]
    pub fn gcmd(&self) -> GcmdR {
        GcmdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc GCMD"]
    #[inline(always)]
    pub fn gcmd(&mut self) -> GcmdW<'_, GcmdrSpec> {
        GcmdW::new(self, 0)
    }
}
#[doc = "desc GCMDR\n\nYou can [`read`](crate::Reg::read) this register and get [`gcmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcmdrSpec;
impl crate::RegisterSpec for GcmdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcmdr::R`](R) reader structure"]
impl crate::Readable for GcmdrSpec {}
#[doc = "`write(|w| ..)` method takes [`gcmdr::W`](W) writer structure"]
impl crate::Writable for GcmdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCMDR to value 0xffff"]
impl crate::Resettable for GcmdrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
