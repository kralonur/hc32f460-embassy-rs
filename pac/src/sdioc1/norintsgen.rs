#[doc = "Register `NORINTSGEN` reader"]
pub type R = crate::R<NorintsgenSpec>;
#[doc = "Register `NORINTSGEN` writer"]
pub type W = crate::W<NorintsgenSpec>;
#[doc = "Field `CCSEN` reader - desc CCSEN"]
pub type CcsenR = crate::BitReader;
#[doc = "Field `CCSEN` writer - desc CCSEN"]
pub type CcsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCSEN` reader - desc TCSEN"]
pub type TcsenR = crate::BitReader;
#[doc = "Field `TCSEN` writer - desc TCSEN"]
pub type TcsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGESEN` reader - desc BGESEN"]
pub type BgesenR = crate::BitReader;
#[doc = "Field `BGESEN` writer - desc BGESEN"]
pub type BgesenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BWRSEN` reader - desc BWRSEN"]
pub type BwrsenR = crate::BitReader;
#[doc = "Field `BWRSEN` writer - desc BWRSEN"]
pub type BwrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRRSEN` reader - desc BRRSEN"]
pub type BrrsenR = crate::BitReader;
#[doc = "Field `BRRSEN` writer - desc BRRSEN"]
pub type BrrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CISTSEN` reader - desc CISTSEN"]
pub type CistsenR = crate::BitReader;
#[doc = "Field `CISTSEN` writer - desc CISTSEN"]
pub type CistsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRMSEN` reader - desc CRMSEN"]
pub type CrmsenR = crate::BitReader;
#[doc = "Field `CRMSEN` writer - desc CRMSEN"]
pub type CrmsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CINTSEN` reader - desc CINTSEN"]
pub type CintsenR = crate::BitReader;
#[doc = "Field `CINTSEN` writer - desc CINTSEN"]
pub type CintsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CCSEN"]
    #[inline(always)]
    pub fn ccsen(&self) -> CcsenR {
        CcsenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TCSEN"]
    #[inline(always)]
    pub fn tcsen(&self) -> TcsenR {
        TcsenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc BGESEN"]
    #[inline(always)]
    pub fn bgesen(&self) -> BgesenR {
        BgesenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc BWRSEN"]
    #[inline(always)]
    pub fn bwrsen(&self) -> BwrsenR {
        BwrsenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc BRRSEN"]
    #[inline(always)]
    pub fn brrsen(&self) -> BrrsenR {
        BrrsenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CISTSEN"]
    #[inline(always)]
    pub fn cistsen(&self) -> CistsenR {
        CistsenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CRMSEN"]
    #[inline(always)]
    pub fn crmsen(&self) -> CrmsenR {
        CrmsenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CINTSEN"]
    #[inline(always)]
    pub fn cintsen(&self) -> CintsenR {
        CintsenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CCSEN"]
    #[inline(always)]
    pub fn ccsen(&mut self) -> CcsenW<'_, NorintsgenSpec> {
        CcsenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc TCSEN"]
    #[inline(always)]
    pub fn tcsen(&mut self) -> TcsenW<'_, NorintsgenSpec> {
        TcsenW::new(self, 1)
    }
    #[doc = "Bit 2 - desc BGESEN"]
    #[inline(always)]
    pub fn bgesen(&mut self) -> BgesenW<'_, NorintsgenSpec> {
        BgesenW::new(self, 2)
    }
    #[doc = "Bit 4 - desc BWRSEN"]
    #[inline(always)]
    pub fn bwrsen(&mut self) -> BwrsenW<'_, NorintsgenSpec> {
        BwrsenW::new(self, 4)
    }
    #[doc = "Bit 5 - desc BRRSEN"]
    #[inline(always)]
    pub fn brrsen(&mut self) -> BrrsenW<'_, NorintsgenSpec> {
        BrrsenW::new(self, 5)
    }
    #[doc = "Bit 6 - desc CISTSEN"]
    #[inline(always)]
    pub fn cistsen(&mut self) -> CistsenW<'_, NorintsgenSpec> {
        CistsenW::new(self, 6)
    }
    #[doc = "Bit 7 - desc CRMSEN"]
    #[inline(always)]
    pub fn crmsen(&mut self) -> CrmsenW<'_, NorintsgenSpec> {
        CrmsenW::new(self, 7)
    }
    #[doc = "Bit 8 - desc CINTSEN"]
    #[inline(always)]
    pub fn cintsen(&mut self) -> CintsenW<'_, NorintsgenSpec> {
        CintsenW::new(self, 8)
    }
}
#[doc = "desc NORINTSGEN\n\nYou can [`read`](crate::Reg::read) this register and get [`norintsgen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`norintsgen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NorintsgenSpec;
impl crate::RegisterSpec for NorintsgenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`norintsgen::R`](R) reader structure"]
impl crate::Readable for NorintsgenSpec {}
#[doc = "`write(|w| ..)` method takes [`norintsgen::W`](W) writer structure"]
impl crate::Writable for NorintsgenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NORINTSGEN to value 0"]
impl crate::Resettable for NorintsgenSpec {}
