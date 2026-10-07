#[doc = "Register `TRGSR` reader"]
pub type R = crate::R<TrgsrSpec>;
#[doc = "Register `TRGSR` writer"]
pub type W = crate::W<TrgsrSpec>;
#[doc = "Field `TRGSELA` reader - desc TRGSELA"]
pub type TrgselaR = crate::FieldReader;
#[doc = "Field `TRGSELA` writer - desc TRGSELA"]
pub type TrgselaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRGENA` reader - desc TRGENA"]
pub type TrgenaR = crate::BitReader;
#[doc = "Field `TRGENA` writer - desc TRGENA"]
pub type TrgenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSELB` reader - desc TRGSELB"]
pub type TrgselbR = crate::FieldReader;
#[doc = "Field `TRGSELB` writer - desc TRGSELB"]
pub type TrgselbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRGENB` reader - desc TRGENB"]
pub type TrgenbR = crate::BitReader;
#[doc = "Field `TRGENB` writer - desc TRGENB"]
pub type TrgenbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc TRGSELA"]
    #[inline(always)]
    pub fn trgsela(&self) -> TrgselaR {
        TrgselaR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - desc TRGENA"]
    #[inline(always)]
    pub fn trgena(&self) -> TrgenaR {
        TrgenaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - desc TRGSELB"]
    #[inline(always)]
    pub fn trgselb(&self) -> TrgselbR {
        TrgselbR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - desc TRGENB"]
    #[inline(always)]
    pub fn trgenb(&self) -> TrgenbR {
        TrgenbR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc TRGSELA"]
    #[inline(always)]
    pub fn trgsela(&mut self) -> TrgselaW<'_, TrgsrSpec> {
        TrgselaW::new(self, 0)
    }
    #[doc = "Bit 7 - desc TRGENA"]
    #[inline(always)]
    pub fn trgena(&mut self) -> TrgenaW<'_, TrgsrSpec> {
        TrgenaW::new(self, 7)
    }
    #[doc = "Bits 8:10 - desc TRGSELB"]
    #[inline(always)]
    pub fn trgselb(&mut self) -> TrgselbW<'_, TrgsrSpec> {
        TrgselbW::new(self, 8)
    }
    #[doc = "Bit 15 - desc TRGENB"]
    #[inline(always)]
    pub fn trgenb(&mut self) -> TrgenbW<'_, TrgsrSpec> {
        TrgenbW::new(self, 15)
    }
}
#[doc = "desc TRGSR\n\nYou can [`read`](crate::Reg::read) this register and get [`trgsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trgsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrgsrSpec;
impl crate::RegisterSpec for TrgsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`trgsr::R`](R) reader structure"]
impl crate::Readable for TrgsrSpec {}
#[doc = "`write(|w| ..)` method takes [`trgsr::W`](W) writer structure"]
impl crate::Writable for TrgsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRGSR to value 0"]
impl crate::Resettable for TrgsrSpec {}
