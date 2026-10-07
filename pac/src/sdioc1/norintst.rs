#[doc = "Register `NORINTST` reader"]
pub type R = crate::R<NorintstSpec>;
#[doc = "Register `NORINTST` writer"]
pub type W = crate::W<NorintstSpec>;
#[doc = "Field `CC` reader - desc CC"]
pub type CcR = crate::BitReader;
#[doc = "Field `CC` writer - desc CC"]
pub type CcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - desc TC"]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - desc TC"]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGE` reader - desc BGE"]
pub type BgeR = crate::BitReader;
#[doc = "Field `BGE` writer - desc BGE"]
pub type BgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BWR` reader - desc BWR"]
pub type BwrR = crate::BitReader;
#[doc = "Field `BWR` writer - desc BWR"]
pub type BwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR` reader - desc BRR"]
pub type BrrR = crate::BitReader;
#[doc = "Field `BRR` writer - desc BRR"]
pub type BrrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIST` reader - desc CIST"]
pub type CistR = crate::BitReader;
#[doc = "Field `CIST` writer - desc CIST"]
pub type CistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRM` reader - desc CRM"]
pub type CrmR = crate::BitReader;
#[doc = "Field `CRM` writer - desc CRM"]
pub type CrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CINT` reader - desc CINT"]
pub type CintR = crate::BitReader;
#[doc = "Field `EI` reader - desc EI"]
pub type EiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc CC"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TC"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc BGE"]
    #[inline(always)]
    pub fn bge(&self) -> BgeR {
        BgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc BWR"]
    #[inline(always)]
    pub fn bwr(&self) -> BwrR {
        BwrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc BRR"]
    #[inline(always)]
    pub fn brr(&self) -> BrrR {
        BrrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CIST"]
    #[inline(always)]
    pub fn cist(&self) -> CistR {
        CistR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CRM"]
    #[inline(always)]
    pub fn crm(&self) -> CrmR {
        CrmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CINT"]
    #[inline(always)]
    pub fn cint(&self) -> CintR {
        CintR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EI"]
    #[inline(always)]
    pub fn ei(&self) -> EiR {
        EiR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CC"]
    #[inline(always)]
    pub fn cc(&mut self) -> CcW<'_, NorintstSpec> {
        CcW::new(self, 0)
    }
    #[doc = "Bit 1 - desc TC"]
    #[inline(always)]
    pub fn tc(&mut self) -> TcW<'_, NorintstSpec> {
        TcW::new(self, 1)
    }
    #[doc = "Bit 2 - desc BGE"]
    #[inline(always)]
    pub fn bge(&mut self) -> BgeW<'_, NorintstSpec> {
        BgeW::new(self, 2)
    }
    #[doc = "Bit 4 - desc BWR"]
    #[inline(always)]
    pub fn bwr(&mut self) -> BwrW<'_, NorintstSpec> {
        BwrW::new(self, 4)
    }
    #[doc = "Bit 5 - desc BRR"]
    #[inline(always)]
    pub fn brr(&mut self) -> BrrW<'_, NorintstSpec> {
        BrrW::new(self, 5)
    }
    #[doc = "Bit 6 - desc CIST"]
    #[inline(always)]
    pub fn cist(&mut self) -> CistW<'_, NorintstSpec> {
        CistW::new(self, 6)
    }
    #[doc = "Bit 7 - desc CRM"]
    #[inline(always)]
    pub fn crm(&mut self) -> CrmW<'_, NorintstSpec> {
        CrmW::new(self, 7)
    }
}
#[doc = "desc NORINTST\n\nYou can [`read`](crate::Reg::read) this register and get [`norintst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`norintst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NorintstSpec;
impl crate::RegisterSpec for NorintstSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`norintst::R`](R) reader structure"]
impl crate::Readable for NorintstSpec {}
#[doc = "`write(|w| ..)` method takes [`norintst::W`](W) writer structure"]
impl crate::Writable for NorintstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NORINTST to value 0"]
impl crate::Resettable for NorintstSpec {}
