#[doc = "Register `PCRB9` reader"]
pub type R = crate::R<Pcrb9Spec>;
#[doc = "Register `PCRB9` writer"]
pub type W = crate::W<Pcrb9Spec>;
#[doc = "Field `POUT` reader - desc POUT"]
pub type PoutR = crate::BitReader;
#[doc = "Field `POUT` writer - desc POUT"]
pub type PoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE` reader - desc POUTE"]
pub type PouteR = crate::BitReader;
#[doc = "Field `POUTE` writer - desc POUTE"]
pub type PouteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOD` reader - desc NOD"]
pub type NodR = crate::BitReader;
#[doc = "Field `NOD` writer - desc NOD"]
pub type NodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRV` reader - desc DRV"]
pub type DrvR = crate::FieldReader;
#[doc = "Field `DRV` writer - desc DRV"]
pub type DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUU` reader - desc PUU"]
pub type PuuR = crate::BitReader;
#[doc = "Field `PUU` writer - desc PUU"]
pub type PuuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN` reader - desc PIN"]
pub type PinR = crate::BitReader;
#[doc = "Field `INVE` reader - desc INVE"]
pub type InveR = crate::BitReader;
#[doc = "Field `INVE` writer - desc INVE"]
pub type InveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTE` reader - desc INTE"]
pub type InteR = crate::BitReader;
#[doc = "Field `INTE` writer - desc INTE"]
pub type InteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTE` reader - desc LTE"]
pub type LteR = crate::BitReader;
#[doc = "Field `LTE` writer - desc LTE"]
pub type LteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDIS` reader - desc DDIS"]
pub type DdisR = crate::BitReader;
#[doc = "Field `DDIS` writer - desc DDIS"]
pub type DdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc POUT"]
    #[inline(always)]
    pub fn pout(&self) -> PoutR {
        PoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc POUTE"]
    #[inline(always)]
    pub fn poute(&self) -> PouteR {
        PouteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc NOD"]
    #[inline(always)]
    pub fn nod(&self) -> NodR {
        NodR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - desc DRV"]
    #[inline(always)]
    pub fn drv(&self) -> DrvR {
        DrvR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc PUU"]
    #[inline(always)]
    pub fn puu(&self) -> PuuR {
        PuuR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PIN"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc INVE"]
    #[inline(always)]
    pub fn inve(&self) -> InveR {
        InveR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - desc INTE"]
    #[inline(always)]
    pub fn inte(&self) -> InteR {
        InteR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - desc LTE"]
    #[inline(always)]
    pub fn lte(&self) -> LteR {
        LteR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc DDIS"]
    #[inline(always)]
    pub fn ddis(&self) -> DdisR {
        DdisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc POUT"]
    #[inline(always)]
    pub fn pout(&mut self) -> PoutW<'_, Pcrb9Spec> {
        PoutW::new(self, 0)
    }
    #[doc = "Bit 1 - desc POUTE"]
    #[inline(always)]
    pub fn poute(&mut self) -> PouteW<'_, Pcrb9Spec> {
        PouteW::new(self, 1)
    }
    #[doc = "Bit 2 - desc NOD"]
    #[inline(always)]
    pub fn nod(&mut self) -> NodW<'_, Pcrb9Spec> {
        NodW::new(self, 2)
    }
    #[doc = "Bits 4:5 - desc DRV"]
    #[inline(always)]
    pub fn drv(&mut self) -> DrvW<'_, Pcrb9Spec> {
        DrvW::new(self, 4)
    }
    #[doc = "Bit 6 - desc PUU"]
    #[inline(always)]
    pub fn puu(&mut self) -> PuuW<'_, Pcrb9Spec> {
        PuuW::new(self, 6)
    }
    #[doc = "Bit 9 - desc INVE"]
    #[inline(always)]
    pub fn inve(&mut self) -> InveW<'_, Pcrb9Spec> {
        InveW::new(self, 9)
    }
    #[doc = "Bit 12 - desc INTE"]
    #[inline(always)]
    pub fn inte(&mut self) -> InteW<'_, Pcrb9Spec> {
        InteW::new(self, 12)
    }
    #[doc = "Bit 14 - desc LTE"]
    #[inline(always)]
    pub fn lte(&mut self) -> LteW<'_, Pcrb9Spec> {
        LteW::new(self, 14)
    }
    #[doc = "Bit 15 - desc DDIS"]
    #[inline(always)]
    pub fn ddis(&mut self) -> DdisW<'_, Pcrb9Spec> {
        DdisW::new(self, 15)
    }
}
#[doc = "desc PCRB9\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrb9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrb9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcrb9Spec;
impl crate::RegisterSpec for Pcrb9Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pcrb9::R`](R) reader structure"]
impl crate::Readable for Pcrb9Spec {}
#[doc = "`write(|w| ..)` method takes [`pcrb9::W`](W) writer structure"]
impl crate::Writable for Pcrb9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCRB9 to value 0"]
impl crate::Resettable for Pcrb9Spec {}
