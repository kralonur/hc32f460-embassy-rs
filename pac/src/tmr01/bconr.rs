#[doc = "Register `BCONR` reader"]
pub type R = crate::R<BconrSpec>;
#[doc = "Register `BCONR` writer"]
pub type W = crate::W<BconrSpec>;
#[doc = "Field `CSTA` reader - desc CSTA"]
pub type CstaR = crate::BitReader;
#[doc = "Field `CSTA` writer - desc CSTA"]
pub type CstaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPMDA` reader - desc CAPMDA"]
pub type CapmdaR = crate::BitReader;
#[doc = "Field `CAPMDA` writer - desc CAPMDA"]
pub type CapmdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENA` reader - desc INTENA"]
pub type IntenaR = crate::BitReader;
#[doc = "Field `INTENA` writer - desc INTENA"]
pub type IntenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKDIVA` reader - desc CKDIVA"]
pub type CkdivaR = crate::FieldReader;
#[doc = "Field `CKDIVA` writer - desc CKDIVA"]
pub type CkdivaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SYNSA` reader - desc SYNSA"]
pub type SynsaR = crate::BitReader;
#[doc = "Field `SYNSA` writer - desc SYNSA"]
pub type SynsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCLKA` reader - desc SYNCLKA"]
pub type SynclkaR = crate::BitReader;
#[doc = "Field `SYNCLKA` writer - desc SYNCLKA"]
pub type SynclkaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCLKA` reader - desc ASYNCLKA"]
pub type AsynclkaR = crate::BitReader;
#[doc = "Field `ASYNCLKA` writer - desc ASYNCLKA"]
pub type AsynclkaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTAA` reader - desc HSTAA"]
pub type HstaaR = crate::BitReader;
#[doc = "Field `HSTAA` writer - desc HSTAA"]
pub type HstaaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTPA` reader - desc HSTPA"]
pub type HstpaR = crate::BitReader;
#[doc = "Field `HSTPA` writer - desc HSTPA"]
pub type HstpaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLEA` reader - desc HCLEA"]
pub type HcleaR = crate::BitReader;
#[doc = "Field `HCLEA` writer - desc HCLEA"]
pub type HcleaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICPA` reader - desc HICPA"]
pub type HicpaR = crate::BitReader;
#[doc = "Field `HICPA` writer - desc HICPA"]
pub type HicpaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTB` reader - desc CSTB"]
pub type CstbR = crate::BitReader;
#[doc = "Field `CSTB` writer - desc CSTB"]
pub type CstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPMDB` reader - desc CAPMDB"]
pub type CapmdbR = crate::BitReader;
#[doc = "Field `CAPMDB` writer - desc CAPMDB"]
pub type CapmdbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENB` reader - desc INTENB"]
pub type IntenbR = crate::BitReader;
#[doc = "Field `INTENB` writer - desc INTENB"]
pub type IntenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKDIVB` reader - desc CKDIVB"]
pub type CkdivbR = crate::FieldReader;
#[doc = "Field `CKDIVB` writer - desc CKDIVB"]
pub type CkdivbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SYNSB` reader - desc SYNSB"]
pub type SynsbR = crate::BitReader;
#[doc = "Field `SYNSB` writer - desc SYNSB"]
pub type SynsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCLKB` reader - desc SYNCLKB"]
pub type SynclkbR = crate::BitReader;
#[doc = "Field `SYNCLKB` writer - desc SYNCLKB"]
pub type SynclkbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCLKB` reader - desc ASYNCLKB"]
pub type AsynclkbR = crate::BitReader;
#[doc = "Field `ASYNCLKB` writer - desc ASYNCLKB"]
pub type AsynclkbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTAB` reader - desc HSTAB"]
pub type HstabR = crate::BitReader;
#[doc = "Field `HSTAB` writer - desc HSTAB"]
pub type HstabW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTPB` reader - desc HSTPB"]
pub type HstpbR = crate::BitReader;
#[doc = "Field `HSTPB` writer - desc HSTPB"]
pub type HstpbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLEB` reader - desc HCLEB"]
pub type HclebR = crate::BitReader;
#[doc = "Field `HCLEB` writer - desc HCLEB"]
pub type HclebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICPB` reader - desc HICPB"]
pub type HicpbR = crate::BitReader;
#[doc = "Field `HICPB` writer - desc HICPB"]
pub type HicpbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CSTA"]
    #[inline(always)]
    pub fn csta(&self) -> CstaR {
        CstaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CAPMDA"]
    #[inline(always)]
    pub fn capmda(&self) -> CapmdaR {
        CapmdaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc INTENA"]
    #[inline(always)]
    pub fn intena(&self) -> IntenaR {
        IntenaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - desc CKDIVA"]
    #[inline(always)]
    pub fn ckdiva(&self) -> CkdivaR {
        CkdivaR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - desc SYNSA"]
    #[inline(always)]
    pub fn synsa(&self) -> SynsaR {
        SynsaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc SYNCLKA"]
    #[inline(always)]
    pub fn synclka(&self) -> SynclkaR {
        SynclkaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ASYNCLKA"]
    #[inline(always)]
    pub fn asynclka(&self) -> AsynclkaR {
        AsynclkaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - desc HSTAA"]
    #[inline(always)]
    pub fn hstaa(&self) -> HstaaR {
        HstaaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc HSTPA"]
    #[inline(always)]
    pub fn hstpa(&self) -> HstpaR {
        HstpaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc HCLEA"]
    #[inline(always)]
    pub fn hclea(&self) -> HcleaR {
        HcleaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc HICPA"]
    #[inline(always)]
    pub fn hicpa(&self) -> HicpaR {
        HicpaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc CSTB"]
    #[inline(always)]
    pub fn cstb(&self) -> CstbR {
        CstbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc CAPMDB"]
    #[inline(always)]
    pub fn capmdb(&self) -> CapmdbR {
        CapmdbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc INTENB"]
    #[inline(always)]
    pub fn intenb(&self) -> IntenbR {
        IntenbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - desc CKDIVB"]
    #[inline(always)]
    pub fn ckdivb(&self) -> CkdivbR {
        CkdivbR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - desc SYNSB"]
    #[inline(always)]
    pub fn synsb(&self) -> SynsbR {
        SynsbR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc SYNCLKB"]
    #[inline(always)]
    pub fn synclkb(&self) -> SynclkbR {
        SynclkbR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc ASYNCLKB"]
    #[inline(always)]
    pub fn asynclkb(&self) -> AsynclkbR {
        AsynclkbR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - desc HSTAB"]
    #[inline(always)]
    pub fn hstab(&self) -> HstabR {
        HstabR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc HSTPB"]
    #[inline(always)]
    pub fn hstpb(&self) -> HstpbR {
        HstpbR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc HCLEB"]
    #[inline(always)]
    pub fn hcleb(&self) -> HclebR {
        HclebR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc HICPB"]
    #[inline(always)]
    pub fn hicpb(&self) -> HicpbR {
        HicpbR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CSTA"]
    #[inline(always)]
    pub fn csta(&mut self) -> CstaW<'_, BconrSpec> {
        CstaW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CAPMDA"]
    #[inline(always)]
    pub fn capmda(&mut self) -> CapmdaW<'_, BconrSpec> {
        CapmdaW::new(self, 1)
    }
    #[doc = "Bit 2 - desc INTENA"]
    #[inline(always)]
    pub fn intena(&mut self) -> IntenaW<'_, BconrSpec> {
        IntenaW::new(self, 2)
    }
    #[doc = "Bits 4:7 - desc CKDIVA"]
    #[inline(always)]
    pub fn ckdiva(&mut self) -> CkdivaW<'_, BconrSpec> {
        CkdivaW::new(self, 4)
    }
    #[doc = "Bit 8 - desc SYNSA"]
    #[inline(always)]
    pub fn synsa(&mut self) -> SynsaW<'_, BconrSpec> {
        SynsaW::new(self, 8)
    }
    #[doc = "Bit 9 - desc SYNCLKA"]
    #[inline(always)]
    pub fn synclka(&mut self) -> SynclkaW<'_, BconrSpec> {
        SynclkaW::new(self, 9)
    }
    #[doc = "Bit 10 - desc ASYNCLKA"]
    #[inline(always)]
    pub fn asynclka(&mut self) -> AsynclkaW<'_, BconrSpec> {
        AsynclkaW::new(self, 10)
    }
    #[doc = "Bit 12 - desc HSTAA"]
    #[inline(always)]
    pub fn hstaa(&mut self) -> HstaaW<'_, BconrSpec> {
        HstaaW::new(self, 12)
    }
    #[doc = "Bit 13 - desc HSTPA"]
    #[inline(always)]
    pub fn hstpa(&mut self) -> HstpaW<'_, BconrSpec> {
        HstpaW::new(self, 13)
    }
    #[doc = "Bit 14 - desc HCLEA"]
    #[inline(always)]
    pub fn hclea(&mut self) -> HcleaW<'_, BconrSpec> {
        HcleaW::new(self, 14)
    }
    #[doc = "Bit 15 - desc HICPA"]
    #[inline(always)]
    pub fn hicpa(&mut self) -> HicpaW<'_, BconrSpec> {
        HicpaW::new(self, 15)
    }
    #[doc = "Bit 16 - desc CSTB"]
    #[inline(always)]
    pub fn cstb(&mut self) -> CstbW<'_, BconrSpec> {
        CstbW::new(self, 16)
    }
    #[doc = "Bit 17 - desc CAPMDB"]
    #[inline(always)]
    pub fn capmdb(&mut self) -> CapmdbW<'_, BconrSpec> {
        CapmdbW::new(self, 17)
    }
    #[doc = "Bit 18 - desc INTENB"]
    #[inline(always)]
    pub fn intenb(&mut self) -> IntenbW<'_, BconrSpec> {
        IntenbW::new(self, 18)
    }
    #[doc = "Bits 20:23 - desc CKDIVB"]
    #[inline(always)]
    pub fn ckdivb(&mut self) -> CkdivbW<'_, BconrSpec> {
        CkdivbW::new(self, 20)
    }
    #[doc = "Bit 24 - desc SYNSB"]
    #[inline(always)]
    pub fn synsb(&mut self) -> SynsbW<'_, BconrSpec> {
        SynsbW::new(self, 24)
    }
    #[doc = "Bit 25 - desc SYNCLKB"]
    #[inline(always)]
    pub fn synclkb(&mut self) -> SynclkbW<'_, BconrSpec> {
        SynclkbW::new(self, 25)
    }
    #[doc = "Bit 26 - desc ASYNCLKB"]
    #[inline(always)]
    pub fn asynclkb(&mut self) -> AsynclkbW<'_, BconrSpec> {
        AsynclkbW::new(self, 26)
    }
    #[doc = "Bit 28 - desc HSTAB"]
    #[inline(always)]
    pub fn hstab(&mut self) -> HstabW<'_, BconrSpec> {
        HstabW::new(self, 28)
    }
    #[doc = "Bit 29 - desc HSTPB"]
    #[inline(always)]
    pub fn hstpb(&mut self) -> HstpbW<'_, BconrSpec> {
        HstpbW::new(self, 29)
    }
    #[doc = "Bit 30 - desc HCLEB"]
    #[inline(always)]
    pub fn hcleb(&mut self) -> HclebW<'_, BconrSpec> {
        HclebW::new(self, 30)
    }
    #[doc = "Bit 31 - desc HICPB"]
    #[inline(always)]
    pub fn hicpb(&mut self) -> HicpbW<'_, BconrSpec> {
        HicpbW::new(self, 31)
    }
}
#[doc = "desc BCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`bconr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bconr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BconrSpec;
impl crate::RegisterSpec for BconrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bconr::R`](R) reader structure"]
impl crate::Readable for BconrSpec {}
#[doc = "`write(|w| ..)` method takes [`bconr::W`](W) writer structure"]
impl crate::Writable for BconrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCONR to value 0"]
impl crate::Resettable for BconrSpec {}
