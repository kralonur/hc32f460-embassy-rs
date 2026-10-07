#[doc = "Register `CKSR` reader"]
pub type R = crate::R<CksrSpec>;
#[doc = "Register `CKSR` writer"]
pub type W = crate::W<CksrSpec>;
#[doc = "Field `SRAM3_1ERR` reader - desc SRAM3_1ERR"]
pub type Sram3_1errR = crate::BitReader;
#[doc = "Field `SRAM3_1ERR` writer - desc SRAM3_1ERR"]
pub type Sram3_1errW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3_2ERR` reader - desc SRAM3_2ERR"]
pub type Sram3_2errR = crate::BitReader;
#[doc = "Field `SRAM3_2ERR` writer - desc SRAM3_2ERR"]
pub type Sram3_2errW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM12_PYERR` reader - desc SRAM12_PYERR"]
pub type Sram12PyerrR = crate::BitReader;
#[doc = "Field `SRAM12_PYERR` writer - desc SRAM12_PYERR"]
pub type Sram12PyerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMH_PYERR` reader - desc SRAMH_PYERR"]
pub type SramhPyerrR = crate::BitReader;
#[doc = "Field `SRAMH_PYERR` writer - desc SRAMH_PYERR"]
pub type SramhPyerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMR_PYERR` reader - desc SRAMR_PYERR"]
pub type SramrPyerrR = crate::BitReader;
#[doc = "Field `SRAMR_PYERR` writer - desc SRAMR_PYERR"]
pub type SramrPyerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SRAM3_1ERR"]
    #[inline(always)]
    pub fn sram3_1err(&self) -> Sram3_1errR {
        Sram3_1errR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SRAM3_2ERR"]
    #[inline(always)]
    pub fn sram3_2err(&self) -> Sram3_2errR {
        Sram3_2errR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SRAM12_PYERR"]
    #[inline(always)]
    pub fn sram12_pyerr(&self) -> Sram12PyerrR {
        Sram12PyerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SRAMH_PYERR"]
    #[inline(always)]
    pub fn sramh_pyerr(&self) -> SramhPyerrR {
        SramhPyerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc SRAMR_PYERR"]
    #[inline(always)]
    pub fn sramr_pyerr(&self) -> SramrPyerrR {
        SramrPyerrR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SRAM3_1ERR"]
    #[inline(always)]
    pub fn sram3_1err(&mut self) -> Sram3_1errW<'_, CksrSpec> {
        Sram3_1errW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SRAM3_2ERR"]
    #[inline(always)]
    pub fn sram3_2err(&mut self) -> Sram3_2errW<'_, CksrSpec> {
        Sram3_2errW::new(self, 1)
    }
    #[doc = "Bit 2 - desc SRAM12_PYERR"]
    #[inline(always)]
    pub fn sram12_pyerr(&mut self) -> Sram12PyerrW<'_, CksrSpec> {
        Sram12PyerrW::new(self, 2)
    }
    #[doc = "Bit 3 - desc SRAMH_PYERR"]
    #[inline(always)]
    pub fn sramh_pyerr(&mut self) -> SramhPyerrW<'_, CksrSpec> {
        SramhPyerrW::new(self, 3)
    }
    #[doc = "Bit 4 - desc SRAMR_PYERR"]
    #[inline(always)]
    pub fn sramr_pyerr(&mut self) -> SramrPyerrW<'_, CksrSpec> {
        SramrPyerrW::new(self, 4)
    }
}
#[doc = "desc CKSR\n\nYou can [`read`](crate::Reg::read) this register and get [`cksr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CksrSpec;
impl crate::RegisterSpec for CksrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cksr::R`](R) reader structure"]
impl crate::Readable for CksrSpec {}
#[doc = "`write(|w| ..)` method takes [`cksr::W`](W) writer structure"]
impl crate::Writable for CksrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CKSR to value 0"]
impl crate::Resettable for CksrSpec {}
