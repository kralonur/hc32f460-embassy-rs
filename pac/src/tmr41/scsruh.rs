#[doc = "Register `SCSRUH` reader"]
pub type R = crate::R<ScsruhSpec>;
#[doc = "Register `SCSRUH` writer"]
pub type W = crate::W<ScsruhSpec>;
#[doc = "Field `BUFEN` reader - desc BUFEN"]
pub type BufenR = crate::FieldReader;
#[doc = "Field `BUFEN` writer - desc BUFEN"]
pub type BufenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EVTOS` reader - desc EVTOS"]
pub type EvtosR = crate::FieldReader;
#[doc = "Field `EVTOS` writer - desc EVTOS"]
pub type EvtosW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LMC` reader - desc LMC"]
pub type LmcR = crate::BitReader;
#[doc = "Field `LMC` writer - desc LMC"]
pub type LmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTMS` reader - desc EVTMS"]
pub type EvtmsR = crate::BitReader;
#[doc = "Field `EVTMS` writer - desc EVTMS"]
pub type EvtmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTDS` reader - desc EVTDS"]
pub type EvtdsR = crate::BitReader;
#[doc = "Field `EVTDS` writer - desc EVTDS"]
pub type EvtdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEN` reader - desc DEN"]
pub type DenR = crate::BitReader;
#[doc = "Field `DEN` writer - desc DEN"]
pub type DenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - desc PEN"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - desc PEN"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEN` reader - desc UEN"]
pub type UenR = crate::BitReader;
#[doc = "Field `UEN` writer - desc UEN"]
pub type UenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZEN` reader - desc ZEN"]
pub type ZenR = crate::BitReader;
#[doc = "Field `ZEN` writer - desc ZEN"]
pub type ZenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc BUFEN"]
    #[inline(always)]
    pub fn bufen(&self) -> BufenR {
        BufenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - desc EVTOS"]
    #[inline(always)]
    pub fn evtos(&self) -> EvtosR {
        EvtosR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - desc LMC"]
    #[inline(always)]
    pub fn lmc(&self) -> LmcR {
        LmcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - desc EVTMS"]
    #[inline(always)]
    pub fn evtms(&self) -> EvtmsR {
        EvtmsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc EVTDS"]
    #[inline(always)]
    pub fn evtds(&self) -> EvtdsR {
        EvtdsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - desc DEN"]
    #[inline(always)]
    pub fn den(&self) -> DenR {
        DenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PEN"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc UEN"]
    #[inline(always)]
    pub fn uen(&self) -> UenR {
        UenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc ZEN"]
    #[inline(always)]
    pub fn zen(&self) -> ZenR {
        ZenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc BUFEN"]
    #[inline(always)]
    pub fn bufen(&mut self) -> BufenW<'_, ScsruhSpec> {
        BufenW::new(self, 0)
    }
    #[doc = "Bits 2:4 - desc EVTOS"]
    #[inline(always)]
    pub fn evtos(&mut self) -> EvtosW<'_, ScsruhSpec> {
        EvtosW::new(self, 2)
    }
    #[doc = "Bit 5 - desc LMC"]
    #[inline(always)]
    pub fn lmc(&mut self) -> LmcW<'_, ScsruhSpec> {
        LmcW::new(self, 5)
    }
    #[doc = "Bit 8 - desc EVTMS"]
    #[inline(always)]
    pub fn evtms(&mut self) -> EvtmsW<'_, ScsruhSpec> {
        EvtmsW::new(self, 8)
    }
    #[doc = "Bit 9 - desc EVTDS"]
    #[inline(always)]
    pub fn evtds(&mut self) -> EvtdsW<'_, ScsruhSpec> {
        EvtdsW::new(self, 9)
    }
    #[doc = "Bit 12 - desc DEN"]
    #[inline(always)]
    pub fn den(&mut self) -> DenW<'_, ScsruhSpec> {
        DenW::new(self, 12)
    }
    #[doc = "Bit 13 - desc PEN"]
    #[inline(always)]
    pub fn pen(&mut self) -> PenW<'_, ScsruhSpec> {
        PenW::new(self, 13)
    }
    #[doc = "Bit 14 - desc UEN"]
    #[inline(always)]
    pub fn uen(&mut self) -> UenW<'_, ScsruhSpec> {
        UenW::new(self, 14)
    }
    #[doc = "Bit 15 - desc ZEN"]
    #[inline(always)]
    pub fn zen(&mut self) -> ZenW<'_, ScsruhSpec> {
        ZenW::new(self, 15)
    }
}
#[doc = "desc SCSRUH\n\nYou can [`read`](crate::Reg::read) this register and get [`scsruh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsruh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScsruhSpec;
impl crate::RegisterSpec for ScsruhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`scsruh::R`](R) reader structure"]
impl crate::Readable for ScsruhSpec {}
#[doc = "`write(|w| ..)` method takes [`scsruh::W`](W) writer structure"]
impl crate::Writable for ScsruhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCSRUH to value 0"]
impl crate::Resettable for ScsruhSpec {}
