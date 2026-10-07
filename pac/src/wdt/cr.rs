#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `PERI` reader - desc PERI"]
pub type PeriR = crate::FieldReader;
#[doc = "Field `PERI` writer - desc PERI"]
pub type PeriW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKS` reader - desc CKS"]
pub type CksR = crate::FieldReader;
#[doc = "Field `CKS` writer - desc CKS"]
pub type CksW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WDPT` reader - desc WDPT"]
pub type WdptR = crate::FieldReader;
#[doc = "Field `WDPT` writer - desc WDPT"]
pub type WdptW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SLPOFF` reader - desc SLPOFF"]
pub type SlpoffR = crate::BitReader;
#[doc = "Field `SLPOFF` writer - desc SLPOFF"]
pub type SlpoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITS` reader - desc ITS"]
pub type ItsR = crate::BitReader;
#[doc = "Field `ITS` writer - desc ITS"]
pub type ItsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc PERI"]
    #[inline(always)]
    pub fn peri(&self) -> PeriR {
        PeriR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - desc CKS"]
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc WDPT"]
    #[inline(always)]
    pub fn wdpt(&self) -> WdptR {
        WdptR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - desc SLPOFF"]
    #[inline(always)]
    pub fn slpoff(&self) -> SlpoffR {
        SlpoffR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - desc ITS"]
    #[inline(always)]
    pub fn its(&self) -> ItsR {
        ItsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc PERI"]
    #[inline(always)]
    pub fn peri(&mut self) -> PeriW<'_, CrSpec> {
        PeriW::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc CKS"]
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<'_, CrSpec> {
        CksW::new(self, 4)
    }
    #[doc = "Bits 8:11 - desc WDPT"]
    #[inline(always)]
    pub fn wdpt(&mut self) -> WdptW<'_, CrSpec> {
        WdptW::new(self, 8)
    }
    #[doc = "Bit 16 - desc SLPOFF"]
    #[inline(always)]
    pub fn slpoff(&mut self) -> SlpoffW<'_, CrSpec> {
        SlpoffW::new(self, 16)
    }
    #[doc = "Bit 31 - desc ITS"]
    #[inline(always)]
    pub fn its(&mut self) -> ItsW<'_, CrSpec> {
        ItsW::new(self, 31)
    }
}
#[doc = "desc CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0x8001_0ff3"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x8001_0ff3;
}
