#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `I2SSTD` reader - desc I2SSTD"]
pub type I2sstdR = crate::FieldReader;
#[doc = "Field `I2SSTD` writer - desc I2SSTD"]
pub type I2sstdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATLEN` reader - desc DATLEN"]
pub type DatlenR = crate::FieldReader;
#[doc = "Field `DATLEN` writer - desc DATLEN"]
pub type DatlenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHLEN` reader - desc CHLEN"]
pub type ChlenR = crate::BitReader;
#[doc = "Field `CHLEN` writer - desc CHLEN"]
pub type ChlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCMSYNC` reader - desc PCMSYNC"]
pub type PcmsyncR = crate::BitReader;
#[doc = "Field `PCMSYNC` writer - desc PCMSYNC"]
pub type PcmsyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc I2SSTD"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2sstdR {
        I2sstdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc DATLEN"]
    #[inline(always)]
    pub fn datlen(&self) -> DatlenR {
        DatlenR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - desc CHLEN"]
    #[inline(always)]
    pub fn chlen(&self) -> ChlenR {
        ChlenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PCMSYNC"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PcmsyncR {
        PcmsyncR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc I2SSTD"]
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2sstdW<'_, CfgrSpec> {
        I2sstdW::new(self, 0)
    }
    #[doc = "Bits 2:3 - desc DATLEN"]
    #[inline(always)]
    pub fn datlen(&mut self) -> DatlenW<'_, CfgrSpec> {
        DatlenW::new(self, 2)
    }
    #[doc = "Bit 4 - desc CHLEN"]
    #[inline(always)]
    pub fn chlen(&mut self) -> ChlenW<'_, CfgrSpec> {
        ChlenW::new(self, 4)
    }
    #[doc = "Bit 5 - desc PCMSYNC"]
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PcmsyncW<'_, CfgrSpec> {
        PcmsyncW::new(self, 5)
    }
}
#[doc = "desc CFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
