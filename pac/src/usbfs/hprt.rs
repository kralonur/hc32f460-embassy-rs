#[doc = "Register `HPRT` reader"]
pub type R = crate::R<HprtSpec>;
#[doc = "Register `HPRT` writer"]
pub type W = crate::W<HprtSpec>;
#[doc = "Field `PCSTS` reader - desc PCSTS"]
pub type PcstsR = crate::BitReader;
#[doc = "Field `PCDET` reader - desc PCDET"]
pub type PcdetR = crate::BitReader;
#[doc = "Field `PCDET` writer - desc PCDET"]
pub type PcdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENA` reader - desc PENA"]
pub type PenaR = crate::BitReader;
#[doc = "Field `PENA` writer - desc PENA"]
pub type PenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENCHNG` reader - desc PENCHNG"]
pub type PenchngR = crate::BitReader;
#[doc = "Field `PENCHNG` writer - desc PENCHNG"]
pub type PenchngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRES` reader - desc PRES"]
pub type PresR = crate::BitReader;
#[doc = "Field `PRES` writer - desc PRES"]
pub type PresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSUSP` reader - desc PSUSP"]
pub type PsuspR = crate::BitReader;
#[doc = "Field `PSUSP` writer - desc PSUSP"]
pub type PsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST` reader - desc PRST"]
pub type PrstR = crate::BitReader;
#[doc = "Field `PRST` writer - desc PRST"]
pub type PrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLSTS` reader - desc PLSTS"]
pub type PlstsR = crate::FieldReader;
#[doc = "Field `PWPR` reader - desc PWPR"]
pub type PwprR = crate::BitReader;
#[doc = "Field `PWPR` writer - desc PWPR"]
pub type PwprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSPD` reader - desc PSPD"]
pub type PspdR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - desc PCSTS"]
    #[inline(always)]
    pub fn pcsts(&self) -> PcstsR {
        PcstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PCDET"]
    #[inline(always)]
    pub fn pcdet(&self) -> PcdetR {
        PcdetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PENA"]
    #[inline(always)]
    pub fn pena(&self) -> PenaR {
        PenaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PENCHNG"]
    #[inline(always)]
    pub fn penchng(&self) -> PenchngR {
        PenchngR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PRES"]
    #[inline(always)]
    pub fn pres(&self) -> PresR {
        PresR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PSUSP"]
    #[inline(always)]
    pub fn psusp(&self) -> PsuspR {
        PsuspR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PRST"]
    #[inline(always)]
    pub fn prst(&self) -> PrstR {
        PrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - desc PLSTS"]
    #[inline(always)]
    pub fn plsts(&self) -> PlstsR {
        PlstsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - desc PWPR"]
    #[inline(always)]
    pub fn pwpr(&self) -> PwprR {
        PwprR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 17:18 - desc PSPD"]
    #[inline(always)]
    pub fn pspd(&self) -> PspdR {
        PspdR::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - desc PCDET"]
    #[inline(always)]
    pub fn pcdet(&mut self) -> PcdetW<'_, HprtSpec> {
        PcdetW::new(self, 1)
    }
    #[doc = "Bit 2 - desc PENA"]
    #[inline(always)]
    pub fn pena(&mut self) -> PenaW<'_, HprtSpec> {
        PenaW::new(self, 2)
    }
    #[doc = "Bit 3 - desc PENCHNG"]
    #[inline(always)]
    pub fn penchng(&mut self) -> PenchngW<'_, HprtSpec> {
        PenchngW::new(self, 3)
    }
    #[doc = "Bit 6 - desc PRES"]
    #[inline(always)]
    pub fn pres(&mut self) -> PresW<'_, HprtSpec> {
        PresW::new(self, 6)
    }
    #[doc = "Bit 7 - desc PSUSP"]
    #[inline(always)]
    pub fn psusp(&mut self) -> PsuspW<'_, HprtSpec> {
        PsuspW::new(self, 7)
    }
    #[doc = "Bit 8 - desc PRST"]
    #[inline(always)]
    pub fn prst(&mut self) -> PrstW<'_, HprtSpec> {
        PrstW::new(self, 8)
    }
    #[doc = "Bit 12 - desc PWPR"]
    #[inline(always)]
    pub fn pwpr(&mut self) -> PwprW<'_, HprtSpec> {
        PwprW::new(self, 12)
    }
}
#[doc = "desc HPRT\n\nYou can [`read`](crate::Reg::read) this register and get [`hprt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HprtSpec;
impl crate::RegisterSpec for HprtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hprt::R`](R) reader structure"]
impl crate::Readable for HprtSpec {}
#[doc = "`write(|w| ..)` method takes [`hprt::W`](W) writer structure"]
impl crate::Writable for HprtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HPRT to value 0"]
impl crate::Resettable for HprtSpec {}
