#[doc = "Register `HCCHAR2` reader"]
pub type R = crate::R<Hcchar2Spec>;
#[doc = "Register `HCCHAR2` writer"]
pub type W = crate::W<Hcchar2Spec>;
#[doc = "Field `MPSIZ` reader - desc MPSIZ"]
pub type MpsizR = crate::FieldReader<u16>;
#[doc = "Field `MPSIZ` writer - desc MPSIZ"]
pub type MpsizW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPNUM` reader - desc EPNUM"]
pub type EpnumR = crate::FieldReader;
#[doc = "Field `EPNUM` writer - desc EPNUM"]
pub type EpnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPDIR` reader - desc EPDIR"]
pub type EpdirR = crate::BitReader;
#[doc = "Field `EPDIR` writer - desc EPDIR"]
pub type EpdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSDEV` reader - desc LSDEV"]
pub type LsdevR = crate::BitReader;
#[doc = "Field `LSDEV` writer - desc LSDEV"]
pub type LsdevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTYP` reader - desc EPTYP"]
pub type EptypR = crate::FieldReader;
#[doc = "Field `DAD` reader - desc DAD"]
pub type DadR = crate::FieldReader;
#[doc = "Field `DAD` writer - desc DAD"]
pub type DadW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ODDFRM` reader - desc ODDFRM"]
pub type OddfrmR = crate::BitReader;
#[doc = "Field `ODDFRM` writer - desc ODDFRM"]
pub type OddfrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDIS` reader - desc CHDIS"]
pub type ChdisR = crate::BitReader;
#[doc = "Field `CHDIS` writer - desc CHDIS"]
pub type ChdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHENA` reader - desc CHENA"]
pub type ChenaR = crate::BitReader;
#[doc = "Field `CHENA` writer - desc CHENA"]
pub type ChenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - desc MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MpsizR {
        MpsizR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - desc EPNUM"]
    #[inline(always)]
    pub fn epnum(&self) -> EpnumR {
        EpnumR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - desc EPDIR"]
    #[inline(always)]
    pub fn epdir(&self) -> EpdirR {
        EpdirR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - desc LSDEV"]
    #[inline(always)]
    pub fn lsdev(&self) -> LsdevR {
        LsdevR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - desc EPTYP"]
    #[inline(always)]
    pub fn eptyp(&self) -> EptypR {
        EptypR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:28 - desc DAD"]
    #[inline(always)]
    pub fn dad(&self) -> DadR {
        DadR::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - desc ODDFRM"]
    #[inline(always)]
    pub fn oddfrm(&self) -> OddfrmR {
        OddfrmR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc CHDIS"]
    #[inline(always)]
    pub fn chdis(&self) -> ChdisR {
        ChdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc CHENA"]
    #[inline(always)]
    pub fn chena(&self) -> ChenaR {
        ChenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - desc MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MpsizW<'_, Hcchar2Spec> {
        MpsizW::new(self, 0)
    }
    #[doc = "Bits 11:14 - desc EPNUM"]
    #[inline(always)]
    pub fn epnum(&mut self) -> EpnumW<'_, Hcchar2Spec> {
        EpnumW::new(self, 11)
    }
    #[doc = "Bit 15 - desc EPDIR"]
    #[inline(always)]
    pub fn epdir(&mut self) -> EpdirW<'_, Hcchar2Spec> {
        EpdirW::new(self, 15)
    }
    #[doc = "Bit 17 - desc LSDEV"]
    #[inline(always)]
    pub fn lsdev(&mut self) -> LsdevW<'_, Hcchar2Spec> {
        LsdevW::new(self, 17)
    }
    #[doc = "Bits 22:28 - desc DAD"]
    #[inline(always)]
    pub fn dad(&mut self) -> DadW<'_, Hcchar2Spec> {
        DadW::new(self, 22)
    }
    #[doc = "Bit 29 - desc ODDFRM"]
    #[inline(always)]
    pub fn oddfrm(&mut self) -> OddfrmW<'_, Hcchar2Spec> {
        OddfrmW::new(self, 29)
    }
    #[doc = "Bit 30 - desc CHDIS"]
    #[inline(always)]
    pub fn chdis(&mut self) -> ChdisW<'_, Hcchar2Spec> {
        ChdisW::new(self, 30)
    }
    #[doc = "Bit 31 - desc CHENA"]
    #[inline(always)]
    pub fn chena(&mut self) -> ChenaW<'_, Hcchar2Spec> {
        ChenaW::new(self, 31)
    }
}
#[doc = "desc HCCHAR2\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcchar2Spec;
impl crate::RegisterSpec for Hcchar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcchar2::R`](R) reader structure"]
impl crate::Readable for Hcchar2Spec {}
#[doc = "`write(|w| ..)` method takes [`hcchar2::W`](W) writer structure"]
impl crate::Writable for Hcchar2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCCHAR2 to value 0"]
impl crate::Resettable for Hcchar2Spec {}
