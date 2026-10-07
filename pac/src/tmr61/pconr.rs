#[doc = "Register `PCONR` reader"]
pub type R = crate::R<PconrSpec>;
#[doc = "Register `PCONR` writer"]
pub type W = crate::W<PconrSpec>;
#[doc = "Field `CAPMDA` reader - desc CAPMDA"]
pub type CapmdaR = crate::BitReader;
#[doc = "Field `CAPMDA` writer - desc CAPMDA"]
pub type CapmdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STACA` reader - desc STACA"]
pub type StacaR = crate::BitReader;
#[doc = "Field `STACA` writer - desc STACA"]
pub type StacaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPCA` reader - desc STPCA"]
pub type StpcaR = crate::BitReader;
#[doc = "Field `STPCA` writer - desc STPCA"]
pub type StpcaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STASTPSA` reader - desc STASTPSA"]
pub type StastpsaR = crate::BitReader;
#[doc = "Field `STASTPSA` writer - desc STASTPSA"]
pub type StastpsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPCA` reader - desc CMPCA"]
pub type CmpcaR = crate::FieldReader;
#[doc = "Field `CMPCA` writer - desc CMPCA"]
pub type CmpcaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PERCA` reader - desc PERCA"]
pub type PercaR = crate::FieldReader;
#[doc = "Field `PERCA` writer - desc PERCA"]
pub type PercaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OUTENA` reader - desc OUTENA"]
pub type OutenaR = crate::BitReader;
#[doc = "Field `OUTENA` writer - desc OUTENA"]
pub type OutenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMBVALA` reader - desc EMBVALA"]
pub type EmbvalaR = crate::FieldReader;
#[doc = "Field `EMBVALA` writer - desc EMBVALA"]
pub type EmbvalaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CAPMDB` reader - desc CAPMDB"]
pub type CapmdbR = crate::BitReader;
#[doc = "Field `CAPMDB` writer - desc CAPMDB"]
pub type CapmdbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STACB` reader - desc STACB"]
pub type StacbR = crate::BitReader;
#[doc = "Field `STACB` writer - desc STACB"]
pub type StacbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPCB` reader - desc STPCB"]
pub type StpcbR = crate::BitReader;
#[doc = "Field `STPCB` writer - desc STPCB"]
pub type StpcbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STASTPSB` reader - desc STASTPSB"]
pub type StastpsbR = crate::BitReader;
#[doc = "Field `STASTPSB` writer - desc STASTPSB"]
pub type StastpsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPCB` reader - desc CMPCB"]
pub type CmpcbR = crate::FieldReader;
#[doc = "Field `CMPCB` writer - desc CMPCB"]
pub type CmpcbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PERCB` reader - desc PERCB"]
pub type PercbR = crate::FieldReader;
#[doc = "Field `PERCB` writer - desc PERCB"]
pub type PercbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OUTENB` reader - desc OUTENB"]
pub type OutenbR = crate::BitReader;
#[doc = "Field `OUTENB` writer - desc OUTENB"]
pub type OutenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMBVALB` reader - desc EMBVALB"]
pub type EmbvalbR = crate::FieldReader;
#[doc = "Field `EMBVALB` writer - desc EMBVALB"]
pub type EmbvalbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc CAPMDA"]
    #[inline(always)]
    pub fn capmda(&self) -> CapmdaR {
        CapmdaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc STACA"]
    #[inline(always)]
    pub fn staca(&self) -> StacaR {
        StacaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc STPCA"]
    #[inline(always)]
    pub fn stpca(&self) -> StpcaR {
        StpcaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc STASTPSA"]
    #[inline(always)]
    pub fn stastpsa(&self) -> StastpsaR {
        StastpsaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - desc CMPCA"]
    #[inline(always)]
    pub fn cmpca(&self) -> CmpcaR {
        CmpcaR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - desc PERCA"]
    #[inline(always)]
    pub fn perca(&self) -> PercaR {
        PercaR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - desc OUTENA"]
    #[inline(always)]
    pub fn outena(&self) -> OutenaR {
        OutenaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 11:12 - desc EMBVALA"]
    #[inline(always)]
    pub fn embvala(&self) -> EmbvalaR {
        EmbvalaR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 16 - desc CAPMDB"]
    #[inline(always)]
    pub fn capmdb(&self) -> CapmdbR {
        CapmdbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc STACB"]
    #[inline(always)]
    pub fn stacb(&self) -> StacbR {
        StacbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc STPCB"]
    #[inline(always)]
    pub fn stpcb(&self) -> StpcbR {
        StpcbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc STASTPSB"]
    #[inline(always)]
    pub fn stastpsb(&self) -> StastpsbR {
        StastpsbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - desc CMPCB"]
    #[inline(always)]
    pub fn cmpcb(&self) -> CmpcbR {
        CmpcbR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - desc PERCB"]
    #[inline(always)]
    pub fn percb(&self) -> PercbR {
        PercbR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - desc OUTENB"]
    #[inline(always)]
    pub fn outenb(&self) -> OutenbR {
        OutenbR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 27:28 - desc EMBVALB"]
    #[inline(always)]
    pub fn embvalb(&self) -> EmbvalbR {
        EmbvalbR::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc CAPMDA"]
    #[inline(always)]
    pub fn capmda(&mut self) -> CapmdaW<'_, PconrSpec> {
        CapmdaW::new(self, 0)
    }
    #[doc = "Bit 1 - desc STACA"]
    #[inline(always)]
    pub fn staca(&mut self) -> StacaW<'_, PconrSpec> {
        StacaW::new(self, 1)
    }
    #[doc = "Bit 2 - desc STPCA"]
    #[inline(always)]
    pub fn stpca(&mut self) -> StpcaW<'_, PconrSpec> {
        StpcaW::new(self, 2)
    }
    #[doc = "Bit 3 - desc STASTPSA"]
    #[inline(always)]
    pub fn stastpsa(&mut self) -> StastpsaW<'_, PconrSpec> {
        StastpsaW::new(self, 3)
    }
    #[doc = "Bits 4:5 - desc CMPCA"]
    #[inline(always)]
    pub fn cmpca(&mut self) -> CmpcaW<'_, PconrSpec> {
        CmpcaW::new(self, 4)
    }
    #[doc = "Bits 6:7 - desc PERCA"]
    #[inline(always)]
    pub fn perca(&mut self) -> PercaW<'_, PconrSpec> {
        PercaW::new(self, 6)
    }
    #[doc = "Bit 8 - desc OUTENA"]
    #[inline(always)]
    pub fn outena(&mut self) -> OutenaW<'_, PconrSpec> {
        OutenaW::new(self, 8)
    }
    #[doc = "Bits 11:12 - desc EMBVALA"]
    #[inline(always)]
    pub fn embvala(&mut self) -> EmbvalaW<'_, PconrSpec> {
        EmbvalaW::new(self, 11)
    }
    #[doc = "Bit 16 - desc CAPMDB"]
    #[inline(always)]
    pub fn capmdb(&mut self) -> CapmdbW<'_, PconrSpec> {
        CapmdbW::new(self, 16)
    }
    #[doc = "Bit 17 - desc STACB"]
    #[inline(always)]
    pub fn stacb(&mut self) -> StacbW<'_, PconrSpec> {
        StacbW::new(self, 17)
    }
    #[doc = "Bit 18 - desc STPCB"]
    #[inline(always)]
    pub fn stpcb(&mut self) -> StpcbW<'_, PconrSpec> {
        StpcbW::new(self, 18)
    }
    #[doc = "Bit 19 - desc STASTPSB"]
    #[inline(always)]
    pub fn stastpsb(&mut self) -> StastpsbW<'_, PconrSpec> {
        StastpsbW::new(self, 19)
    }
    #[doc = "Bits 20:21 - desc CMPCB"]
    #[inline(always)]
    pub fn cmpcb(&mut self) -> CmpcbW<'_, PconrSpec> {
        CmpcbW::new(self, 20)
    }
    #[doc = "Bits 22:23 - desc PERCB"]
    #[inline(always)]
    pub fn percb(&mut self) -> PercbW<'_, PconrSpec> {
        PercbW::new(self, 22)
    }
    #[doc = "Bit 24 - desc OUTENB"]
    #[inline(always)]
    pub fn outenb(&mut self) -> OutenbW<'_, PconrSpec> {
        OutenbW::new(self, 24)
    }
    #[doc = "Bits 27:28 - desc EMBVALB"]
    #[inline(always)]
    pub fn embvalb(&mut self) -> EmbvalbW<'_, PconrSpec> {
        EmbvalbW::new(self, 27)
    }
}
#[doc = "desc PCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`pconr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PconrSpec;
impl crate::RegisterSpec for PconrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pconr::R`](R) reader structure"]
impl crate::Readable for PconrSpec {}
#[doc = "`write(|w| ..)` method takes [`pconr::W`](W) writer structure"]
impl crate::Writable for PconrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCONR to value 0"]
impl crate::Resettable for PconrSpec {}
