#[doc = "Register `CFG2` reader"]
pub type R = crate::R<Cfg2Spec>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<Cfg2Spec>;
#[doc = "Field `CPHA` reader - desc CPHA"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - desc CPHA"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - desc CPOL"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - desc CPOL"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBR` reader - desc MBR"]
pub type MbrR = crate::FieldReader;
#[doc = "Field `MBR` writer - desc MBR"]
pub type MbrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SSA` reader - desc SSA"]
pub type SsaR = crate::FieldReader;
#[doc = "Field `SSA` writer - desc SSA"]
pub type SsaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DSIZE` reader - desc DSIZE"]
pub type DsizeR = crate::FieldReader;
#[doc = "Field `DSIZE` writer - desc DSIZE"]
pub type DsizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LSBF` reader - desc LSBF"]
pub type LsbfR = crate::BitReader;
#[doc = "Field `LSBF` writer - desc LSBF"]
pub type LsbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIDIE` reader - desc MIDIE"]
pub type MidieR = crate::BitReader;
#[doc = "Field `MIDIE` writer - desc MIDIE"]
pub type MidieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSSDLE` reader - desc MSSDLE"]
pub type MssdleR = crate::BitReader;
#[doc = "Field `MSSDLE` writer - desc MSSDLE"]
pub type MssdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSSIE` reader - desc MSSIE"]
pub type MssieR = crate::BitReader;
#[doc = "Field `MSSIE` writer - desc MSSIE"]
pub type MssieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CPHA"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CPOL"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - desc MBR"]
    #[inline(always)]
    pub fn mbr(&self) -> MbrR {
        MbrR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - desc SSA"]
    #[inline(always)]
    pub fn ssa(&self) -> SsaR {
        SsaR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - desc DSIZE"]
    #[inline(always)]
    pub fn dsize(&self) -> DsizeR {
        DsizeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - desc LSBF"]
    #[inline(always)]
    pub fn lsbf(&self) -> LsbfR {
        LsbfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc MIDIE"]
    #[inline(always)]
    pub fn midie(&self) -> MidieR {
        MidieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc MSSDLE"]
    #[inline(always)]
    pub fn mssdle(&self) -> MssdleR {
        MssdleR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc MSSIE"]
    #[inline(always)]
    pub fn mssie(&self) -> MssieR {
        MssieR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CPHA"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, Cfg2Spec> {
        CphaW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CPOL"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, Cfg2Spec> {
        CpolW::new(self, 1)
    }
    #[doc = "Bits 2:4 - desc MBR"]
    #[inline(always)]
    pub fn mbr(&mut self) -> MbrW<'_, Cfg2Spec> {
        MbrW::new(self, 2)
    }
    #[doc = "Bits 5:7 - desc SSA"]
    #[inline(always)]
    pub fn ssa(&mut self) -> SsaW<'_, Cfg2Spec> {
        SsaW::new(self, 5)
    }
    #[doc = "Bits 8:11 - desc DSIZE"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DsizeW<'_, Cfg2Spec> {
        DsizeW::new(self, 8)
    }
    #[doc = "Bit 12 - desc LSBF"]
    #[inline(always)]
    pub fn lsbf(&mut self) -> LsbfW<'_, Cfg2Spec> {
        LsbfW::new(self, 12)
    }
    #[doc = "Bit 13 - desc MIDIE"]
    #[inline(always)]
    pub fn midie(&mut self) -> MidieW<'_, Cfg2Spec> {
        MidieW::new(self, 13)
    }
    #[doc = "Bit 14 - desc MSSDLE"]
    #[inline(always)]
    pub fn mssdle(&mut self) -> MssdleW<'_, Cfg2Spec> {
        MssdleW::new(self, 14)
    }
    #[doc = "Bit 15 - desc MSSIE"]
    #[inline(always)]
    pub fn mssie(&mut self) -> MssieW<'_, Cfg2Spec> {
        MssieW::new(self, 15)
    }
}
#[doc = "desc CFG2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg2Spec;
impl crate::RegisterSpec for Cfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for Cfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for Cfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG2 to value 0x0f1d"]
impl crate::Resettable for Cfg2Spec {
    const RESET_VALUE: u32 = 0x0f1d;
}
