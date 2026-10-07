#[doc = "Register `FCG3` reader"]
pub type R = crate::R<Fcg3Spec>;
#[doc = "Register `FCG3` writer"]
pub type W = crate::W<Fcg3Spec>;
#[doc = "Field `ADC1` reader - desc ADC1"]
pub type Adc1R = crate::BitReader;
#[doc = "Field `ADC1` writer - desc ADC1"]
pub type Adc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2` reader - desc ADC2"]
pub type Adc2R = crate::BitReader;
#[doc = "Field `ADC2` writer - desc ADC2"]
pub type Adc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP` reader - desc CMP"]
pub type CmpR = crate::BitReader;
#[doc = "Field `CMP` writer - desc CMP"]
pub type CmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTS` reader - desc OTS"]
pub type OtsR = crate::BitReader;
#[doc = "Field `OTS` writer - desc OTS"]
pub type OtsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc ADC1"]
    #[inline(always)]
    pub fn adc1(&self) -> Adc1R {
        Adc1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ADC2"]
    #[inline(always)]
    pub fn adc2(&self) -> Adc2R {
        Adc2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CMP"]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - desc OTS"]
    #[inline(always)]
    pub fn ots(&self) -> OtsR {
        OtsR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ADC1"]
    #[inline(always)]
    pub fn adc1(&mut self) -> Adc1W<'_, Fcg3Spec> {
        Adc1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc ADC2"]
    #[inline(always)]
    pub fn adc2(&mut self) -> Adc2W<'_, Fcg3Spec> {
        Adc2W::new(self, 1)
    }
    #[doc = "Bit 8 - desc CMP"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<'_, Fcg3Spec> {
        CmpW::new(self, 8)
    }
    #[doc = "Bit 12 - desc OTS"]
    #[inline(always)]
    pub fn ots(&mut self) -> OtsW<'_, Fcg3Spec> {
        OtsW::new(self, 12)
    }
}
#[doc = "desc FCG3\n\nYou can [`read`](crate::Reg::read) this register and get [`fcg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fcg3Spec;
impl crate::RegisterSpec for Fcg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcg3::R`](R) reader structure"]
impl crate::Readable for Fcg3Spec {}
#[doc = "`write(|w| ..)` method takes [`fcg3::W`](W) writer structure"]
impl crate::Writable for Fcg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCG3 to value 0xffff_ffff"]
impl crate::Resettable for Fcg3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
