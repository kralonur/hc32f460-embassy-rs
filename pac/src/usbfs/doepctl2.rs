#[doc = "Register `DOEPCTL2` reader"]
pub type R = crate::R<Doepctl2Spec>;
#[doc = "Register `DOEPCTL2` writer"]
pub type W = crate::W<Doepctl2Spec>;
#[doc = "Field `MPSIZ` reader - desc MPSIZ"]
pub type MpsizR = crate::FieldReader<u16>;
#[doc = "Field `MPSIZ` writer - desc MPSIZ"]
pub type MpsizW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `USBAEP` reader - desc USBAEP"]
pub type UsbaepR = crate::BitReader;
#[doc = "Field `USBAEP` writer - desc USBAEP"]
pub type UsbaepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPID` reader - desc DPID"]
pub type DpidR = crate::BitReader;
#[doc = "Field `NAKSTS` reader - desc NAKSTS"]
pub type NakstsR = crate::BitReader;
#[doc = "Field `EPTYP` reader - desc EPTYP"]
pub type EptypR = crate::FieldReader;
#[doc = "Field `EPTYP` writer - desc EPTYP"]
pub type EptypW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SNPM` reader - desc SNPM"]
pub type SnpmR = crate::BitReader;
#[doc = "Field `SNPM` writer - desc SNPM"]
pub type SnpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - desc STALL"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - desc STALL"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` reader - desc CNAK"]
pub type CnakR = crate::BitReader;
#[doc = "Field `CNAK` writer - desc CNAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` reader - desc SNAK"]
pub type SnakR = crate::BitReader;
#[doc = "Field `SNAK` writer - desc SNAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD0PID` reader - desc SD0PID"]
pub type Sd0pidR = crate::BitReader;
#[doc = "Field `SD0PID` writer - desc SD0PID"]
pub type Sd0pidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD1PID` reader - desc SD1PID"]
pub type Sd1pidR = crate::BitReader;
#[doc = "Field `SD1PID` writer - desc SD1PID"]
pub type Sd1pidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - desc EPDIS"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPDIS` writer - desc EPDIS"]
pub type EpdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPENA` reader - desc EPENA"]
pub type EpenaR = crate::BitReader;
#[doc = "Field `EPENA` writer - desc EPENA"]
pub type EpenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - desc MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MpsizR {
        MpsizR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - desc USBAEP"]
    #[inline(always)]
    pub fn usbaep(&self) -> UsbaepR {
        UsbaepR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc DPID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc NAKSTS"]
    #[inline(always)]
    pub fn naksts(&self) -> NakstsR {
        NakstsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - desc EPTYP"]
    #[inline(always)]
    pub fn eptyp(&self) -> EptypR {
        EptypR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - desc SNPM"]
    #[inline(always)]
    pub fn snpm(&self) -> SnpmR {
        SnpmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc STALL"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - desc CNAK"]
    #[inline(always)]
    pub fn cnak(&self) -> CnakR {
        CnakR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc SNAK"]
    #[inline(always)]
    pub fn snak(&self) -> SnakR {
        SnakR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc SD0PID"]
    #[inline(always)]
    pub fn sd0pid(&self) -> Sd0pidR {
        Sd0pidR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc SD1PID"]
    #[inline(always)]
    pub fn sd1pid(&self) -> Sd1pidR {
        Sd1pidR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc EPDIS"]
    #[inline(always)]
    pub fn epdis(&self) -> EpdisR {
        EpdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc EPENA"]
    #[inline(always)]
    pub fn epena(&self) -> EpenaR {
        EpenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - desc MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MpsizW<'_, Doepctl2Spec> {
        MpsizW::new(self, 0)
    }
    #[doc = "Bit 15 - desc USBAEP"]
    #[inline(always)]
    pub fn usbaep(&mut self) -> UsbaepW<'_, Doepctl2Spec> {
        UsbaepW::new(self, 15)
    }
    #[doc = "Bits 18:19 - desc EPTYP"]
    #[inline(always)]
    pub fn eptyp(&mut self) -> EptypW<'_, Doepctl2Spec> {
        EptypW::new(self, 18)
    }
    #[doc = "Bit 20 - desc SNPM"]
    #[inline(always)]
    pub fn snpm(&mut self) -> SnpmW<'_, Doepctl2Spec> {
        SnpmW::new(self, 20)
    }
    #[doc = "Bit 21 - desc STALL"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<'_, Doepctl2Spec> {
        StallW::new(self, 21)
    }
    #[doc = "Bit 26 - desc CNAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CnakW<'_, Doepctl2Spec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - desc SNAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SnakW<'_, Doepctl2Spec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 28 - desc SD0PID"]
    #[inline(always)]
    pub fn sd0pid(&mut self) -> Sd0pidW<'_, Doepctl2Spec> {
        Sd0pidW::new(self, 28)
    }
    #[doc = "Bit 29 - desc SD1PID"]
    #[inline(always)]
    pub fn sd1pid(&mut self) -> Sd1pidW<'_, Doepctl2Spec> {
        Sd1pidW::new(self, 29)
    }
    #[doc = "Bit 30 - desc EPDIS"]
    #[inline(always)]
    pub fn epdis(&mut self) -> EpdisW<'_, Doepctl2Spec> {
        EpdisW::new(self, 30)
    }
    #[doc = "Bit 31 - desc EPENA"]
    #[inline(always)]
    pub fn epena(&mut self) -> EpenaW<'_, Doepctl2Spec> {
        EpenaW::new(self, 31)
    }
}
#[doc = "desc DOEPCTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepctl2Spec;
impl crate::RegisterSpec for Doepctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepctl2::R`](R) reader structure"]
impl crate::Readable for Doepctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`doepctl2::W`](W) writer structure"]
impl crate::Writable for Doepctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPCTL2 to value 0"]
impl crate::Resettable for Doepctl2Spec {}
