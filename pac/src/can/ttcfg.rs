#[doc = "Register `TTCFG` reader"]
pub type R = crate::R<TtcfgSpec>;
#[doc = "Register `TTCFG` writer"]
pub type W = crate::W<TtcfgSpec>;
#[doc = "Field `TTEN` reader - desc TTEN"]
pub type TtenR = crate::BitReader;
#[doc = "Field `TTEN` writer - desc TTEN"]
pub type TtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T_PRESC` reader - desc T_PRESC"]
pub type TPrescR = crate::FieldReader;
#[doc = "Field `T_PRESC` writer - desc T_PRESC"]
pub type TPrescW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TTIF` reader - desc TTIF"]
pub type TtifR = crate::BitReader;
#[doc = "Field `TTIF` writer - desc TTIF"]
pub type TtifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTIE` reader - desc TTIE"]
pub type TtieR = crate::BitReader;
#[doc = "Field `TTIE` writer - desc TTIE"]
pub type TtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIF` reader - desc TEIF"]
pub type TeifR = crate::BitReader;
#[doc = "Field `TEIF` writer - desc TEIF"]
pub type TeifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTIF` reader - desc WTIF"]
pub type WtifR = crate::BitReader;
#[doc = "Field `WTIF` writer - desc WTIF"]
pub type WtifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTIE` reader - desc WTIE"]
pub type WtieR = crate::BitReader;
#[doc = "Field `WTIE` writer - desc WTIE"]
pub type WtieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TTEN"]
    #[inline(always)]
    pub fn tten(&self) -> TtenR {
        TtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc T_PRESC"]
    #[inline(always)]
    pub fn t_presc(&self) -> TPrescR {
        TPrescR::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - desc TTIF"]
    #[inline(always)]
    pub fn ttif(&self) -> TtifR {
        TtifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TTIE"]
    #[inline(always)]
    pub fn ttie(&self) -> TtieR {
        TtieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TEIF"]
    #[inline(always)]
    pub fn teif(&self) -> TeifR {
        TeifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc WTIF"]
    #[inline(always)]
    pub fn wtif(&self) -> WtifR {
        WtifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc WTIE"]
    #[inline(always)]
    pub fn wtie(&self) -> WtieR {
        WtieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TTEN"]
    #[inline(always)]
    pub fn tten(&mut self) -> TtenW<'_, TtcfgSpec> {
        TtenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - desc T_PRESC"]
    #[inline(always)]
    pub fn t_presc(&mut self) -> TPrescW<'_, TtcfgSpec> {
        TPrescW::new(self, 1)
    }
    #[doc = "Bit 3 - desc TTIF"]
    #[inline(always)]
    pub fn ttif(&mut self) -> TtifW<'_, TtcfgSpec> {
        TtifW::new(self, 3)
    }
    #[doc = "Bit 4 - desc TTIE"]
    #[inline(always)]
    pub fn ttie(&mut self) -> TtieW<'_, TtcfgSpec> {
        TtieW::new(self, 4)
    }
    #[doc = "Bit 5 - desc TEIF"]
    #[inline(always)]
    pub fn teif(&mut self) -> TeifW<'_, TtcfgSpec> {
        TeifW::new(self, 5)
    }
    #[doc = "Bit 6 - desc WTIF"]
    #[inline(always)]
    pub fn wtif(&mut self) -> WtifW<'_, TtcfgSpec> {
        WtifW::new(self, 6)
    }
    #[doc = "Bit 7 - desc WTIE"]
    #[inline(always)]
    pub fn wtie(&mut self) -> WtieW<'_, TtcfgSpec> {
        WtieW::new(self, 7)
    }
}
#[doc = "desc TTCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`ttcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TtcfgSpec;
impl crate::RegisterSpec for TtcfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ttcfg::R`](R) reader structure"]
impl crate::Readable for TtcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ttcfg::W`](W) writer structure"]
impl crate::Writable for TtcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TTCFG to value 0x90"]
impl crate::Resettable for TtcfgSpec {
    const RESET_VALUE: u8 = 0x90;
}
