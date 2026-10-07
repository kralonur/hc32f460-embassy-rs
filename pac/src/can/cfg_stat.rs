#[doc = "Register `CFG_STAT` reader"]
pub type R = crate::R<CfgStatSpec>;
#[doc = "Register `CFG_STAT` writer"]
pub type W = crate::W<CfgStatSpec>;
#[doc = "Field `BUSOFF` reader - desc BUSOFF"]
pub type BusoffR = crate::BitReader;
#[doc = "Field `BUSOFF` writer - desc BUSOFF"]
pub type BusoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TACTIVE` reader - desc TACTIVE"]
pub type TactiveR = crate::BitReader;
#[doc = "Field `RACTIVE` reader - desc RACTIVE"]
pub type RactiveR = crate::BitReader;
#[doc = "Field `TSSS` reader - desc TSSS"]
pub type TsssR = crate::BitReader;
#[doc = "Field `TSSS` writer - desc TSSS"]
pub type TsssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPSS` reader - desc TPSS"]
pub type TpssR = crate::BitReader;
#[doc = "Field `TPSS` writer - desc TPSS"]
pub type TpssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBMI` reader - desc LBMI"]
pub type LbmiR = crate::BitReader;
#[doc = "Field `LBMI` writer - desc LBMI"]
pub type LbmiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBME` reader - desc LBME"]
pub type LbmeR = crate::BitReader;
#[doc = "Field `LBME` writer - desc LBME"]
pub type LbmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - desc RESET"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - desc RESET"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc BUSOFF"]
    #[inline(always)]
    pub fn busoff(&self) -> BusoffR {
        BusoffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TACTIVE"]
    #[inline(always)]
    pub fn tactive(&self) -> TactiveR {
        TactiveR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc RACTIVE"]
    #[inline(always)]
    pub fn ractive(&self) -> RactiveR {
        RactiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TSSS"]
    #[inline(always)]
    pub fn tsss(&self) -> TsssR {
        TsssR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TPSS"]
    #[inline(always)]
    pub fn tpss(&self) -> TpssR {
        TpssR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc LBMI"]
    #[inline(always)]
    pub fn lbmi(&self) -> LbmiR {
        LbmiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc LBME"]
    #[inline(always)]
    pub fn lbme(&self) -> LbmeR {
        LbmeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RESET"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BUSOFF"]
    #[inline(always)]
    pub fn busoff(&mut self) -> BusoffW<'_, CfgStatSpec> {
        BusoffW::new(self, 0)
    }
    #[doc = "Bit 3 - desc TSSS"]
    #[inline(always)]
    pub fn tsss(&mut self) -> TsssW<'_, CfgStatSpec> {
        TsssW::new(self, 3)
    }
    #[doc = "Bit 4 - desc TPSS"]
    #[inline(always)]
    pub fn tpss(&mut self) -> TpssW<'_, CfgStatSpec> {
        TpssW::new(self, 4)
    }
    #[doc = "Bit 5 - desc LBMI"]
    #[inline(always)]
    pub fn lbmi(&mut self) -> LbmiW<'_, CfgStatSpec> {
        LbmiW::new(self, 5)
    }
    #[doc = "Bit 6 - desc LBME"]
    #[inline(always)]
    pub fn lbme(&mut self) -> LbmeW<'_, CfgStatSpec> {
        LbmeW::new(self, 6)
    }
    #[doc = "Bit 7 - desc RESET"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, CfgStatSpec> {
        ResetW::new(self, 7)
    }
}
#[doc = "desc CFG_STAT\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgStatSpec;
impl crate::RegisterSpec for CfgStatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cfg_stat::R`](R) reader structure"]
impl crate::Readable for CfgStatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_stat::W`](W) writer structure"]
impl crate::Writable for CfgStatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_STAT to value 0x80"]
impl crate::Resettable for CfgStatSpec {
    const RESET_VALUE: u8 = 0x80;
}
