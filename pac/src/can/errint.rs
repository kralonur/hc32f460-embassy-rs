#[doc = "Register `ERRINT` reader"]
pub type R = crate::R<ErrintSpec>;
#[doc = "Register `ERRINT` writer"]
pub type W = crate::W<ErrintSpec>;
#[doc = "Field `BEIF` reader - desc BEIF"]
pub type BeifR = crate::BitReader;
#[doc = "Field `BEIF` writer - desc BEIF"]
pub type BeifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIE` reader - desc BEIE"]
pub type BeieR = crate::BitReader;
#[doc = "Field `BEIE` writer - desc BEIE"]
pub type BeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALIF` reader - desc ALIF"]
pub type AlifR = crate::BitReader;
#[doc = "Field `ALIF` writer - desc ALIF"]
pub type AlifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALIE` reader - desc ALIE"]
pub type AlieR = crate::BitReader;
#[doc = "Field `ALIE` writer - desc ALIE"]
pub type AlieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPIF` reader - desc EPIF"]
pub type EpifR = crate::BitReader;
#[doc = "Field `EPIF` writer - desc EPIF"]
pub type EpifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPIE` reader - desc EPIE"]
pub type EpieR = crate::BitReader;
#[doc = "Field `EPIE` writer - desc EPIE"]
pub type EpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPASS` reader - desc EPASS"]
pub type EpassR = crate::BitReader;
#[doc = "Field `EWARN` reader - desc EWARN"]
pub type EwarnR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc BEIF"]
    #[inline(always)]
    pub fn beif(&self) -> BeifR {
        BeifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc BEIE"]
    #[inline(always)]
    pub fn beie(&self) -> BeieR {
        BeieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ALIF"]
    #[inline(always)]
    pub fn alif(&self) -> AlifR {
        AlifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc ALIE"]
    #[inline(always)]
    pub fn alie(&self) -> AlieR {
        AlieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc EPIF"]
    #[inline(always)]
    pub fn epif(&self) -> EpifR {
        EpifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc EPIE"]
    #[inline(always)]
    pub fn epie(&self) -> EpieR {
        EpieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc EPASS"]
    #[inline(always)]
    pub fn epass(&self) -> EpassR {
        EpassR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc EWARN"]
    #[inline(always)]
    pub fn ewarn(&self) -> EwarnR {
        EwarnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BEIF"]
    #[inline(always)]
    pub fn beif(&mut self) -> BeifW<'_, ErrintSpec> {
        BeifW::new(self, 0)
    }
    #[doc = "Bit 1 - desc BEIE"]
    #[inline(always)]
    pub fn beie(&mut self) -> BeieW<'_, ErrintSpec> {
        BeieW::new(self, 1)
    }
    #[doc = "Bit 2 - desc ALIF"]
    #[inline(always)]
    pub fn alif(&mut self) -> AlifW<'_, ErrintSpec> {
        AlifW::new(self, 2)
    }
    #[doc = "Bit 3 - desc ALIE"]
    #[inline(always)]
    pub fn alie(&mut self) -> AlieW<'_, ErrintSpec> {
        AlieW::new(self, 3)
    }
    #[doc = "Bit 4 - desc EPIF"]
    #[inline(always)]
    pub fn epif(&mut self) -> EpifW<'_, ErrintSpec> {
        EpifW::new(self, 4)
    }
    #[doc = "Bit 5 - desc EPIE"]
    #[inline(always)]
    pub fn epie(&mut self) -> EpieW<'_, ErrintSpec> {
        EpieW::new(self, 5)
    }
}
#[doc = "desc ERRINT\n\nYou can [`read`](crate::Reg::read) this register and get [`errint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrintSpec;
impl crate::RegisterSpec for ErrintSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`errint::R`](R) reader structure"]
impl crate::Readable for ErrintSpec {}
#[doc = "`write(|w| ..)` method takes [`errint::W`](W) writer structure"]
impl crate::Writable for ErrintSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERRINT to value 0"]
impl crate::Resettable for ErrintSpec {}
