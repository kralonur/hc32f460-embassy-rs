#[doc = "Register `ICG1` reader"]
pub type R = crate::R<Icg1Spec>;
#[doc = "Field `HRCFREQSEL` reader - desc HRCFREQSEL"]
pub type HrcfreqselR = crate::BitReader;
#[doc = "Field `HRCSTOP` reader - desc HRCSTOP"]
pub type HrcstopR = crate::BitReader;
#[doc = "Field `BOR_LEV` reader - desc BOR_LEV"]
pub type BorLevR = crate::FieldReader;
#[doc = "Field `BORDIS` reader - desc BORDIS"]
pub type BordisR = crate::BitReader;
#[doc = "Field `SMPCLK` reader - desc SMPCLK"]
pub type SmpclkR = crate::FieldReader;
#[doc = "Field `NMITRG` reader - desc NMITRG"]
pub type NmitrgR = crate::BitReader;
#[doc = "Field `NMIEN` reader - desc NMIEN"]
pub type NmienR = crate::BitReader;
#[doc = "Field `NFEN` reader - desc NFEN"]
pub type NfenR = crate::BitReader;
#[doc = "Field `NMIICGEN` reader - desc NMIICGEN"]
pub type NmiicgenR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc HRCFREQSEL"]
    #[inline(always)]
    pub fn hrcfreqsel(&self) -> HrcfreqselR {
        HrcfreqselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - desc HRCSTOP"]
    #[inline(always)]
    pub fn hrcstop(&self) -> HrcstopR {
        HrcstopR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - desc BOR_LEV"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BorLevR {
        BorLevR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - desc BORDIS"]
    #[inline(always)]
    pub fn bordis(&self) -> BordisR {
        BordisR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 26:27 - desc SMPCLK"]
    #[inline(always)]
    pub fn smpclk(&self) -> SmpclkR {
        SmpclkR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - desc NMITRG"]
    #[inline(always)]
    pub fn nmitrg(&self) -> NmitrgR {
        NmitrgR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc NMIEN"]
    #[inline(always)]
    pub fn nmien(&self) -> NmienR {
        NmienR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc NFEN"]
    #[inline(always)]
    pub fn nfen(&self) -> NfenR {
        NfenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc NMIICGEN"]
    #[inline(always)]
    pub fn nmiicgen(&self) -> NmiicgenR {
        NmiicgenR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc ICG1\n\nYou can [`read`](crate::Reg::read) this register and get [`icg1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icg1Spec;
impl crate::RegisterSpec for Icg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icg1::R`](R) reader structure"]
impl crate::Readable for Icg1Spec {}
#[doc = "`reset()` method sets ICG1 to value 0xffff_ffff"]
impl crate::Resettable for Icg1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
