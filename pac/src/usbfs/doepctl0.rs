#[doc = "Register `DOEPCTL0` reader"]
pub type R = crate::R<Doepctl0Spec>;
#[doc = "Register `DOEPCTL0` writer"]
pub type W = crate::W<Doepctl0Spec>;
#[doc = "Field `MPSIZ` reader - desc MPSIZ"]
pub type MpsizR = crate::FieldReader;
#[doc = "Field `MPSIZ` writer - desc MPSIZ"]
pub type MpsizW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USBAEP` reader - desc USBAEP"]
pub type UsbaepR = crate::BitReader;
#[doc = "Field `NAKSTS` reader - desc NAKSTS"]
pub type NakstsR = crate::BitReader;
#[doc = "Field `EPTYP` reader - desc EPTYP"]
pub type EptypR = crate::FieldReader;
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
#[doc = "Field `EPDIS` reader - desc EPDIS"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPENA` reader - desc EPENA"]
pub type EpenaR = crate::BitReader;
#[doc = "Field `EPENA` writer - desc EPENA"]
pub type EpenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MpsizR {
        MpsizR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - desc USBAEP"]
    #[inline(always)]
    pub fn usbaep(&self) -> UsbaepR {
        UsbaepR::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bits 0:1 - desc MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MpsizW<'_, Doepctl0Spec> {
        MpsizW::new(self, 0)
    }
    #[doc = "Bit 20 - desc SNPM"]
    #[inline(always)]
    pub fn snpm(&mut self) -> SnpmW<'_, Doepctl0Spec> {
        SnpmW::new(self, 20)
    }
    #[doc = "Bit 21 - desc STALL"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<'_, Doepctl0Spec> {
        StallW::new(self, 21)
    }
    #[doc = "Bit 26 - desc CNAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CnakW<'_, Doepctl0Spec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - desc SNAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SnakW<'_, Doepctl0Spec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 31 - desc EPENA"]
    #[inline(always)]
    pub fn epena(&mut self) -> EpenaW<'_, Doepctl0Spec> {
        EpenaW::new(self, 31)
    }
}
#[doc = "desc DOEPCTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepctl0Spec;
impl crate::RegisterSpec for Doepctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepctl0::R`](R) reader structure"]
impl crate::Readable for Doepctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`doepctl0::W`](W) writer structure"]
impl crate::Writable for Doepctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPCTL0 to value 0x8000"]
impl crate::Resettable for Doepctl0Spec {
    const RESET_VALUE: u32 = 0x8000;
}
