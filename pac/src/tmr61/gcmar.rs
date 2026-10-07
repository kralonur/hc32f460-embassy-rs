#[doc = "Register `GCMAR` reader"]
pub type R = crate::R<GcmarSpec>;
#[doc = "Register `GCMAR` writer"]
pub type W = crate::W<GcmarSpec>;
#[doc = "Field `GCMA` reader - desc GCMA"]
pub type GcmaR = crate::FieldReader<u16>;
#[doc = "Field `GCMA` writer - desc GCMA"]
pub type GcmaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc GCMA"]
    #[inline(always)]
    pub fn gcma(&self) -> GcmaR {
        GcmaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc GCMA"]
    #[inline(always)]
    pub fn gcma(&mut self) -> GcmaW<'_, GcmarSpec> {
        GcmaW::new(self, 0)
    }
}
#[doc = "desc GCMAR\n\nYou can [`read`](crate::Reg::read) this register and get [`gcmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcmarSpec;
impl crate::RegisterSpec for GcmarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcmar::R`](R) reader structure"]
impl crate::Readable for GcmarSpec {}
#[doc = "`write(|w| ..)` method takes [`gcmar::W`](W) writer structure"]
impl crate::Writable for GcmarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCMAR to value 0xffff"]
impl crate::Resettable for GcmarSpec {
    const RESET_VALUE: u32 = 0xffff;
}
