#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `MDSEL` reader - desc MDSEL"]
pub type MdselR = crate::FieldReader;
#[doc = "Field `MDSEL` writer - desc MDSEL"]
pub type MdselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PFE` reader - desc PFE"]
pub type PfeR = crate::BitReader;
#[doc = "Field `PFE` writer - desc PFE"]
pub type PfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFSAE` reader - desc PFSAE"]
pub type PfsaeR = crate::BitReader;
#[doc = "Field `PFSAE` writer - desc PFSAE"]
pub type PfsaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCOME` reader - desc DCOME"]
pub type DcomeR = crate::BitReader;
#[doc = "Field `DCOME` writer - desc DCOME"]
pub type DcomeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIPE` reader - desc XIPE"]
pub type XipeR = crate::BitReader;
#[doc = "Field `XIPE` writer - desc XIPE"]
pub type XipeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIMD3` reader - desc SPIMD3"]
pub type Spimd3R = crate::BitReader;
#[doc = "Field `SPIMD3` writer - desc SPIMD3"]
pub type Spimd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPRSL` reader - desc IPRSL"]
pub type IprslR = crate::FieldReader;
#[doc = "Field `IPRSL` writer - desc IPRSL"]
pub type IprslW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `APRSL` reader - desc APRSL"]
pub type AprslR = crate::FieldReader;
#[doc = "Field `APRSL` writer - desc APRSL"]
pub type AprslW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DPRSL` reader - desc DPRSL"]
pub type DprslR = crate::FieldReader;
#[doc = "Field `DPRSL` writer - desc DPRSL"]
pub type DprslW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIV` reader - desc DIV"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - desc DIV"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:2 - desc MDSEL"]
    #[inline(always)]
    pub fn mdsel(&self) -> MdselR {
        MdselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc PFE"]
    #[inline(always)]
    pub fn pfe(&self) -> PfeR {
        PfeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PFSAE"]
    #[inline(always)]
    pub fn pfsae(&self) -> PfsaeR {
        PfsaeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DCOME"]
    #[inline(always)]
    pub fn dcome(&self) -> DcomeR {
        DcomeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc XIPE"]
    #[inline(always)]
    pub fn xipe(&self) -> XipeR {
        XipeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc SPIMD3"]
    #[inline(always)]
    pub fn spimd3(&self) -> Spimd3R {
        Spimd3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - desc IPRSL"]
    #[inline(always)]
    pub fn iprsl(&self) -> IprslR {
        IprslR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - desc APRSL"]
    #[inline(always)]
    pub fn aprsl(&self) -> AprslR {
        AprslR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - desc DPRSL"]
    #[inline(always)]
    pub fn dprsl(&self) -> DprslR {
        DprslR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:21 - desc DIV"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc MDSEL"]
    #[inline(always)]
    pub fn mdsel(&mut self) -> MdselW<'_, CrSpec> {
        MdselW::new(self, 0)
    }
    #[doc = "Bit 3 - desc PFE"]
    #[inline(always)]
    pub fn pfe(&mut self) -> PfeW<'_, CrSpec> {
        PfeW::new(self, 3)
    }
    #[doc = "Bit 4 - desc PFSAE"]
    #[inline(always)]
    pub fn pfsae(&mut self) -> PfsaeW<'_, CrSpec> {
        PfsaeW::new(self, 4)
    }
    #[doc = "Bit 5 - desc DCOME"]
    #[inline(always)]
    pub fn dcome(&mut self) -> DcomeW<'_, CrSpec> {
        DcomeW::new(self, 5)
    }
    #[doc = "Bit 6 - desc XIPE"]
    #[inline(always)]
    pub fn xipe(&mut self) -> XipeW<'_, CrSpec> {
        XipeW::new(self, 6)
    }
    #[doc = "Bit 7 - desc SPIMD3"]
    #[inline(always)]
    pub fn spimd3(&mut self) -> Spimd3W<'_, CrSpec> {
        Spimd3W::new(self, 7)
    }
    #[doc = "Bits 8:9 - desc IPRSL"]
    #[inline(always)]
    pub fn iprsl(&mut self) -> IprslW<'_, CrSpec> {
        IprslW::new(self, 8)
    }
    #[doc = "Bits 10:11 - desc APRSL"]
    #[inline(always)]
    pub fn aprsl(&mut self) -> AprslW<'_, CrSpec> {
        AprslW::new(self, 10)
    }
    #[doc = "Bits 12:13 - desc DPRSL"]
    #[inline(always)]
    pub fn dprsl(&mut self) -> DprslW<'_, CrSpec> {
        DprslW::new(self, 12)
    }
    #[doc = "Bits 16:21 - desc DIV"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, CrSpec> {
        DivW::new(self, 16)
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
#[doc = "`reset()` method sets CR to value 0x003f_0000"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x003f_0000;
}
