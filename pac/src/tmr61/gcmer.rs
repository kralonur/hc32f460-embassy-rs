#[doc = "Register `GCMER` reader"]
pub type R = crate::R<GcmerSpec>;
#[doc = "Register `GCMER` writer"]
pub type W = crate::W<GcmerSpec>;
#[doc = "Field `GCME` reader - desc GCME"]
pub type GcmeR = crate::FieldReader<u16>;
#[doc = "Field `GCME` writer - desc GCME"]
pub type GcmeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc GCME"]
    #[inline(always)]
    pub fn gcme(&self) -> GcmeR {
        GcmeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc GCME"]
    #[inline(always)]
    pub fn gcme(&mut self) -> GcmeW<'_, GcmerSpec> {
        GcmeW::new(self, 0)
    }
}
#[doc = "desc GCMER\n\nYou can [`read`](crate::Reg::read) this register and get [`gcmer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcmer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcmerSpec;
impl crate::RegisterSpec for GcmerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcmer::R`](R) reader structure"]
impl crate::Readable for GcmerSpec {}
#[doc = "`write(|w| ..)` method takes [`gcmer::W`](W) writer structure"]
impl crate::Writable for GcmerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCMER to value 0xffff"]
impl crate::Resettable for GcmerSpec {
    const RESET_VALUE: u32 = 0xffff;
}
