#[doc = "Register `EXAR` reader"]
pub type R = crate::R<ExarSpec>;
#[doc = "Register `EXAR` writer"]
pub type W = crate::W<ExarSpec>;
#[doc = "Field `EXADR` reader - desc EXADR"]
pub type ExadrR = crate::FieldReader;
#[doc = "Field `EXADR` writer - desc EXADR"]
pub type ExadrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 26:31 - desc EXADR"]
    #[inline(always)]
    pub fn exadr(&self) -> ExadrR {
        ExadrR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 26:31 - desc EXADR"]
    #[inline(always)]
    pub fn exadr(&mut self) -> ExadrW<'_, ExarSpec> {
        ExadrW::new(self, 26)
    }
}
#[doc = "desc EXAR\n\nYou can [`read`](crate::Reg::read) this register and get [`exar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExarSpec;
impl crate::RegisterSpec for ExarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exar::R`](R) reader structure"]
impl crate::Readable for ExarSpec {}
#[doc = "`write(|w| ..)` method takes [`exar::W`](W) writer structure"]
impl crate::Writable for ExarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXAR to value 0"]
impl crate::Resettable for ExarSpec {}
