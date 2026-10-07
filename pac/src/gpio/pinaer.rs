#[doc = "Register `PINAER` reader"]
pub type R = crate::R<PinaerSpec>;
#[doc = "Register `PINAER` writer"]
pub type W = crate::W<PinaerSpec>;
#[doc = "Field `PINAE` reader - desc PINAE"]
pub type PinaeR = crate::FieldReader;
#[doc = "Field `PINAE` writer - desc PINAE"]
pub type PinaeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - desc PINAE"]
    #[inline(always)]
    pub fn pinae(&self) -> PinaeR {
        PinaeR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc PINAE"]
    #[inline(always)]
    pub fn pinae(&mut self) -> PinaeW<'_, PinaerSpec> {
        PinaeW::new(self, 0)
    }
}
#[doc = "desc PINAER\n\nYou can [`read`](crate::Reg::read) this register and get [`pinaer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinaer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PinaerSpec;
impl crate::RegisterSpec for PinaerSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pinaer::R`](R) reader structure"]
impl crate::Readable for PinaerSpec {}
#[doc = "`write(|w| ..)` method takes [`pinaer::W`](W) writer structure"]
impl crate::Writable for PinaerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINAER to value 0"]
impl crate::Resettable for PinaerSpec {}
