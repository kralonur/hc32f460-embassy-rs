#[doc = "Register `RPTB0` reader"]
pub type R = crate::R<Rptb0Spec>;
#[doc = "Register `RPTB0` writer"]
pub type W = crate::W<Rptb0Spec>;
#[doc = "Field `SRPTB` reader - desc SRPTB"]
pub type SrptbR = crate::FieldReader<u16>;
#[doc = "Field `SRPTB` writer - desc SRPTB"]
pub type SrptbW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DRPTB` reader - desc DRPTB"]
pub type DrptbR = crate::FieldReader<u16>;
#[doc = "Field `DRPTB` writer - desc DRPTB"]
pub type DrptbW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - desc SRPTB"]
    #[inline(always)]
    pub fn srptb(&self) -> SrptbR {
        SrptbR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - desc DRPTB"]
    #[inline(always)]
    pub fn drptb(&self) -> DrptbR {
        DrptbR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - desc SRPTB"]
    #[inline(always)]
    pub fn srptb(&mut self) -> SrptbW<'_, Rptb0Spec> {
        SrptbW::new(self, 0)
    }
    #[doc = "Bits 16:25 - desc DRPTB"]
    #[inline(always)]
    pub fn drptb(&mut self) -> DrptbW<'_, Rptb0Spec> {
        DrptbW::new(self, 16)
    }
}
#[doc = "desc RPTB0\n\nYou can [`read`](crate::Reg::read) this register and get [`rptb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rptb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rptb0Spec;
impl crate::RegisterSpec for Rptb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rptb0::R`](R) reader structure"]
impl crate::Readable for Rptb0Spec {}
#[doc = "`write(|w| ..)` method takes [`rptb0::W`](W) writer structure"]
impl crate::Writable for Rptb0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RPTB0 to value 0"]
impl crate::Resettable for Rptb0Spec {}
