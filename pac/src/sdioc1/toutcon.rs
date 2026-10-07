#[doc = "Register `TOUTCON` reader"]
pub type R = crate::R<ToutconSpec>;
#[doc = "Register `TOUTCON` writer"]
pub type W = crate::W<ToutconSpec>;
#[doc = "Field `DTO` reader - desc DTO"]
pub type DtoR = crate::FieldReader;
#[doc = "Field `DTO` writer - desc DTO"]
pub type DtoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc DTO"]
    #[inline(always)]
    pub fn dto(&self) -> DtoR {
        DtoR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc DTO"]
    #[inline(always)]
    pub fn dto(&mut self) -> DtoW<'_, ToutconSpec> {
        DtoW::new(self, 0)
    }
}
#[doc = "desc TOUTCON\n\nYou can [`read`](crate::Reg::read) this register and get [`toutcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`toutcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ToutconSpec;
impl crate::RegisterSpec for ToutconSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`toutcon::R`](R) reader structure"]
impl crate::Readable for ToutconSpec {}
#[doc = "`write(|w| ..)` method takes [`toutcon::W`](W) writer structure"]
impl crate::Writable for ToutconSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUTCON to value 0"]
impl crate::Resettable for ToutconSpec {}
