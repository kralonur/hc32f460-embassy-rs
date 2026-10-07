#[doc = "Register `ADC2_TRGSEL1` reader"]
pub type R = crate::R<Adc2Trgsel1Spec>;
#[doc = "Register `ADC2_TRGSEL1` writer"]
pub type W = crate::W<Adc2Trgsel1Spec>;
#[doc = "Field `TRGSEL` reader - desc TRGSEL"]
pub type TrgselR = crate::FieldReader<u16>;
#[doc = "Field `TRGSEL` writer - desc TRGSEL"]
pub type TrgselW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `COMEN` reader - desc COMEN"]
pub type ComenR = crate::FieldReader;
#[doc = "Field `COMEN` writer - desc COMEN"]
pub type ComenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:8 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 30:31 - desc COMEN"]
    #[inline(always)]
    pub fn comen(&self) -> ComenR {
        ComenR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TrgselW<'_, Adc2Trgsel1Spec> {
        TrgselW::new(self, 0)
    }
    #[doc = "Bits 30:31 - desc COMEN"]
    #[inline(always)]
    pub fn comen(&mut self) -> ComenW<'_, Adc2Trgsel1Spec> {
        ComenW::new(self, 30)
    }
}
#[doc = "desc ADC2_TRGSEL1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc2_trgsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc2_trgsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc2Trgsel1Spec;
impl crate::RegisterSpec for Adc2Trgsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc2_trgsel1::R`](R) reader structure"]
impl crate::Readable for Adc2Trgsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc2_trgsel1::W`](W) writer structure"]
impl crate::Writable for Adc2Trgsel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC2_TRGSEL1 to value 0x01ff"]
impl crate::Resettable for Adc2Trgsel1Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
