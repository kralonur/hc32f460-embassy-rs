#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `OVRERF` reader - desc OVRERF"]
pub type OvrerfR = crate::BitReader;
#[doc = "Field `OVRERF` writer - desc OVRERF"]
pub type OvrerfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLNF` reader - desc IDLNF"]
pub type IdlnfR = crate::BitReader;
#[doc = "Field `MODFERF` reader - desc MODFERF"]
pub type ModferfR = crate::BitReader;
#[doc = "Field `MODFERF` writer - desc MODFERF"]
pub type ModferfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERF` reader - desc PERF"]
pub type PerfR = crate::BitReader;
#[doc = "Field `PERF` writer - desc PERF"]
pub type PerfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRERF` reader - desc UDRERF"]
pub type UdrerfR = crate::BitReader;
#[doc = "Field `UDRERF` writer - desc UDRERF"]
pub type UdrerfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDEF` reader - desc TDEF"]
pub type TdefR = crate::BitReader;
#[doc = "Field `TDEF` writer - desc TDEF"]
pub type TdefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDFF` reader - desc RDFF"]
pub type RdffR = crate::BitReader;
#[doc = "Field `RDFF` writer - desc RDFF"]
pub type RdffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc OVRERF"]
    #[inline(always)]
    pub fn ovrerf(&self) -> OvrerfR {
        OvrerfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc IDLNF"]
    #[inline(always)]
    pub fn idlnf(&self) -> IdlnfR {
        IdlnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc MODFERF"]
    #[inline(always)]
    pub fn modferf(&self) -> ModferfR {
        ModferfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PERF"]
    #[inline(always)]
    pub fn perf(&self) -> PerfR {
        PerfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc UDRERF"]
    #[inline(always)]
    pub fn udrerf(&self) -> UdrerfR {
        UdrerfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TDEF"]
    #[inline(always)]
    pub fn tdef(&self) -> TdefR {
        TdefR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RDFF"]
    #[inline(always)]
    pub fn rdff(&self) -> RdffR {
        RdffR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc OVRERF"]
    #[inline(always)]
    pub fn ovrerf(&mut self) -> OvrerfW<'_, SrSpec> {
        OvrerfW::new(self, 0)
    }
    #[doc = "Bit 2 - desc MODFERF"]
    #[inline(always)]
    pub fn modferf(&mut self) -> ModferfW<'_, SrSpec> {
        ModferfW::new(self, 2)
    }
    #[doc = "Bit 3 - desc PERF"]
    #[inline(always)]
    pub fn perf(&mut self) -> PerfW<'_, SrSpec> {
        PerfW::new(self, 3)
    }
    #[doc = "Bit 4 - desc UDRERF"]
    #[inline(always)]
    pub fn udrerf(&mut self) -> UdrerfW<'_, SrSpec> {
        UdrerfW::new(self, 4)
    }
    #[doc = "Bit 5 - desc TDEF"]
    #[inline(always)]
    pub fn tdef(&mut self) -> TdefW<'_, SrSpec> {
        TdefW::new(self, 5)
    }
    #[doc = "Bit 7 - desc RDFF"]
    #[inline(always)]
    pub fn rdff(&mut self) -> RdffW<'_, SrSpec> {
        RdffW::new(self, 7)
    }
}
#[doc = "desc SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0x20"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x20;
}
