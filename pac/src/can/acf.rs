#[doc = "Register `ACF` reader"]
pub type R = crate::R<AcfSpec>;
#[doc = "Register `ACF` writer"]
pub type W = crate::W<AcfSpec>;
#[doc = "Field `ACODEORAMASK` reader - desc ACODEORAMASK"]
pub type AcodeoramaskR = crate::FieldReader<u32>;
#[doc = "Field `ACODEORAMASK` writer - desc ACODEORAMASK"]
pub type AcodeoramaskW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `AIDE` reader - desc AIDE"]
pub type AideR = crate::BitReader;
#[doc = "Field `AIDE` writer - desc AIDE"]
pub type AideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIDEE` reader - desc AIDEE"]
pub type AideeR = crate::BitReader;
#[doc = "Field `AIDEE` writer - desc AIDEE"]
pub type AideeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:28 - desc ACODEORAMASK"]
    #[inline(always)]
    pub fn acodeoramask(&self) -> AcodeoramaskR {
        AcodeoramaskR::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - desc AIDE"]
    #[inline(always)]
    pub fn aide(&self) -> AideR {
        AideR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc AIDEE"]
    #[inline(always)]
    pub fn aidee(&self) -> AideeR {
        AideeR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - desc ACODEORAMASK"]
    #[inline(always)]
    pub fn acodeoramask(&mut self) -> AcodeoramaskW<'_, AcfSpec> {
        AcodeoramaskW::new(self, 0)
    }
    #[doc = "Bit 29 - desc AIDE"]
    #[inline(always)]
    pub fn aide(&mut self) -> AideW<'_, AcfSpec> {
        AideW::new(self, 29)
    }
    #[doc = "Bit 30 - desc AIDEE"]
    #[inline(always)]
    pub fn aidee(&mut self) -> AideeW<'_, AcfSpec> {
        AideeW::new(self, 30)
    }
}
#[doc = "desc ACF\n\nYou can [`read`](crate::Reg::read) this register and get [`acf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcfSpec;
impl crate::RegisterSpec for AcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acf::R`](R) reader structure"]
impl crate::Readable for AcfSpec {}
#[doc = "`write(|w| ..)` method takes [`acf::W`](W) writer structure"]
impl crate::Writable for AcfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACF to value 0"]
impl crate::Resettable for AcfSpec {}
