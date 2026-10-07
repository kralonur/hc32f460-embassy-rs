#[doc = "Register `RPT3` reader"]
pub type R = crate::R<Rpt3Spec>;
#[doc = "Register `RPT3` writer"]
pub type W = crate::W<Rpt3Spec>;
#[doc = "Field `SRPT` reader - desc SRPT"]
pub type SrptR = crate::FieldReader<u16>;
#[doc = "Field `SRPT` writer - desc SRPT"]
pub type SrptW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DRPT` reader - desc DRPT"]
pub type DrptR = crate::FieldReader<u16>;
#[doc = "Field `DRPT` writer - desc DRPT"]
pub type DrptW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - desc SRPT"]
    #[inline(always)]
    pub fn srpt(&self) -> SrptR {
        SrptR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - desc DRPT"]
    #[inline(always)]
    pub fn drpt(&self) -> DrptR {
        DrptR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - desc SRPT"]
    #[inline(always)]
    pub fn srpt(&mut self) -> SrptW<'_, Rpt3Spec> {
        SrptW::new(self, 0)
    }
    #[doc = "Bits 16:25 - desc DRPT"]
    #[inline(always)]
    pub fn drpt(&mut self) -> DrptW<'_, Rpt3Spec> {
        DrptW::new(self, 16)
    }
}
#[doc = "desc RPT3\n\nYou can [`read`](crate::Reg::read) this register and get [`rpt3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpt3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rpt3Spec;
impl crate::RegisterSpec for Rpt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpt3::R`](R) reader structure"]
impl crate::Readable for Rpt3Spec {}
#[doc = "`write(|w| ..)` method takes [`rpt3::W`](W) writer structure"]
impl crate::Writable for Rpt3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RPT3 to value 0"]
impl crate::Resettable for Rpt3Spec {}
