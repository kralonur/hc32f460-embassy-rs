#[doc = "Register `NORINTSTEN` reader"]
pub type R = crate::R<NorintstenSpec>;
#[doc = "Register `NORINTSTEN` writer"]
pub type W = crate::W<NorintstenSpec>;
#[doc = "Field `CCEN` reader - desc CCEN"]
pub type CcenR = crate::BitReader;
#[doc = "Field `CCEN` writer - desc CCEN"]
pub type CcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCEN` reader - desc TCEN"]
pub type TcenR = crate::BitReader;
#[doc = "Field `TCEN` writer - desc TCEN"]
pub type TcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGEEN` reader - desc BGEEN"]
pub type BgeenR = crate::BitReader;
#[doc = "Field `BGEEN` writer - desc BGEEN"]
pub type BgeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BWREN` reader - desc BWREN"]
pub type BwrenR = crate::BitReader;
#[doc = "Field `BWREN` writer - desc BWREN"]
pub type BwrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRREN` reader - desc BRREN"]
pub type BrrenR = crate::BitReader;
#[doc = "Field `BRREN` writer - desc BRREN"]
pub type BrrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CISTEN` reader - desc CISTEN"]
pub type CistenR = crate::BitReader;
#[doc = "Field `CISTEN` writer - desc CISTEN"]
pub type CistenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRMEN` reader - desc CRMEN"]
pub type CrmenR = crate::BitReader;
#[doc = "Field `CRMEN` writer - desc CRMEN"]
pub type CrmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CINTEN` reader - desc CINTEN"]
pub type CintenR = crate::BitReader;
#[doc = "Field `CINTEN` writer - desc CINTEN"]
pub type CintenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CCEN"]
    #[inline(always)]
    pub fn ccen(&self) -> CcenR {
        CcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TCEN"]
    #[inline(always)]
    pub fn tcen(&self) -> TcenR {
        TcenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc BGEEN"]
    #[inline(always)]
    pub fn bgeen(&self) -> BgeenR {
        BgeenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc BWREN"]
    #[inline(always)]
    pub fn bwren(&self) -> BwrenR {
        BwrenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc BRREN"]
    #[inline(always)]
    pub fn brren(&self) -> BrrenR {
        BrrenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CISTEN"]
    #[inline(always)]
    pub fn cisten(&self) -> CistenR {
        CistenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CRMEN"]
    #[inline(always)]
    pub fn crmen(&self) -> CrmenR {
        CrmenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CINTEN"]
    #[inline(always)]
    pub fn cinten(&self) -> CintenR {
        CintenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CCEN"]
    #[inline(always)]
    pub fn ccen(&mut self) -> CcenW<'_, NorintstenSpec> {
        CcenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc TCEN"]
    #[inline(always)]
    pub fn tcen(&mut self) -> TcenW<'_, NorintstenSpec> {
        TcenW::new(self, 1)
    }
    #[doc = "Bit 2 - desc BGEEN"]
    #[inline(always)]
    pub fn bgeen(&mut self) -> BgeenW<'_, NorintstenSpec> {
        BgeenW::new(self, 2)
    }
    #[doc = "Bit 4 - desc BWREN"]
    #[inline(always)]
    pub fn bwren(&mut self) -> BwrenW<'_, NorintstenSpec> {
        BwrenW::new(self, 4)
    }
    #[doc = "Bit 5 - desc BRREN"]
    #[inline(always)]
    pub fn brren(&mut self) -> BrrenW<'_, NorintstenSpec> {
        BrrenW::new(self, 5)
    }
    #[doc = "Bit 6 - desc CISTEN"]
    #[inline(always)]
    pub fn cisten(&mut self) -> CistenW<'_, NorintstenSpec> {
        CistenW::new(self, 6)
    }
    #[doc = "Bit 7 - desc CRMEN"]
    #[inline(always)]
    pub fn crmen(&mut self) -> CrmenW<'_, NorintstenSpec> {
        CrmenW::new(self, 7)
    }
    #[doc = "Bit 8 - desc CINTEN"]
    #[inline(always)]
    pub fn cinten(&mut self) -> CintenW<'_, NorintstenSpec> {
        CintenW::new(self, 8)
    }
}
#[doc = "desc NORINTSTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`norintsten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`norintsten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NorintstenSpec;
impl crate::RegisterSpec for NorintstenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`norintsten::R`](R) reader structure"]
impl crate::Readable for NorintstenSpec {}
#[doc = "`write(|w| ..)` method takes [`norintsten::W`](W) writer structure"]
impl crate::Writable for NorintstenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NORINTSTEN to value 0"]
impl crate::Resettable for NorintstenSpec {}
