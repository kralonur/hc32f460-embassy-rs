#[doc = "Register `RTIE` reader"]
pub type R = crate::R<RtieSpec>;
#[doc = "Register `RTIE` writer"]
pub type W = crate::W<RtieSpec>;
#[doc = "Field `TSFF` reader - desc TSFF"]
pub type TsffR = crate::BitReader;
#[doc = "Field `EIE` reader - desc EIE"]
pub type EieR = crate::BitReader;
#[doc = "Field `EIE` writer - desc EIE"]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIE` reader - desc TSIE"]
pub type TsieR = crate::BitReader;
#[doc = "Field `TSIE` writer - desc TSIE"]
pub type TsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPIE` reader - desc TPIE"]
pub type TpieR = crate::BitReader;
#[doc = "Field `TPIE` writer - desc TPIE"]
pub type TpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAFIE` reader - desc RAFIE"]
pub type RafieR = crate::BitReader;
#[doc = "Field `RAFIE` writer - desc RAFIE"]
pub type RafieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIE` reader - desc RFIE"]
pub type RfieR = crate::BitReader;
#[doc = "Field `RFIE` writer - desc RFIE"]
pub type RfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROIE` reader - desc ROIE"]
pub type RoieR = crate::BitReader;
#[doc = "Field `ROIE` writer - desc ROIE"]
pub type RoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - desc RIE"]
pub type RieR = crate::BitReader;
#[doc = "Field `RIE` writer - desc RIE"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TSFF"]
    #[inline(always)]
    pub fn tsff(&self) -> TsffR {
        TsffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EIE"]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TSIE"]
    #[inline(always)]
    pub fn tsie(&self) -> TsieR {
        TsieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TPIE"]
    #[inline(always)]
    pub fn tpie(&self) -> TpieR {
        TpieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RAFIE"]
    #[inline(always)]
    pub fn rafie(&self) -> RafieR {
        RafieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RFIE"]
    #[inline(always)]
    pub fn rfie(&self) -> RfieR {
        RfieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ROIE"]
    #[inline(always)]
    pub fn roie(&self) -> RoieR {
        RoieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RIE"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc EIE"]
    #[inline(always)]
    pub fn eie(&mut self) -> EieW<'_, RtieSpec> {
        EieW::new(self, 1)
    }
    #[doc = "Bit 2 - desc TSIE"]
    #[inline(always)]
    pub fn tsie(&mut self) -> TsieW<'_, RtieSpec> {
        TsieW::new(self, 2)
    }
    #[doc = "Bit 3 - desc TPIE"]
    #[inline(always)]
    pub fn tpie(&mut self) -> TpieW<'_, RtieSpec> {
        TpieW::new(self, 3)
    }
    #[doc = "Bit 4 - desc RAFIE"]
    #[inline(always)]
    pub fn rafie(&mut self) -> RafieW<'_, RtieSpec> {
        RafieW::new(self, 4)
    }
    #[doc = "Bit 5 - desc RFIE"]
    #[inline(always)]
    pub fn rfie(&mut self) -> RfieW<'_, RtieSpec> {
        RfieW::new(self, 5)
    }
    #[doc = "Bit 6 - desc ROIE"]
    #[inline(always)]
    pub fn roie(&mut self) -> RoieW<'_, RtieSpec> {
        RoieW::new(self, 6)
    }
    #[doc = "Bit 7 - desc RIE"]
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<'_, RtieSpec> {
        RieW::new(self, 7)
    }
}
#[doc = "desc RTIE\n\nYou can [`read`](crate::Reg::read) this register and get [`rtie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtieSpec;
impl crate::RegisterSpec for RtieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtie::R`](R) reader structure"]
impl crate::Readable for RtieSpec {}
#[doc = "`write(|w| ..)` method takes [`rtie::W`](W) writer structure"]
impl crate::Writable for RtieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTIE to value 0xfe"]
impl crate::Resettable for RtieSpec {
    const RESET_VALUE: u8 = 0xfe;
}
