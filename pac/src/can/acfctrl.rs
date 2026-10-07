#[doc = "Register `ACFCTRL` reader"]
pub type R = crate::R<AcfctrlSpec>;
#[doc = "Register `ACFCTRL` writer"]
pub type W = crate::W<AcfctrlSpec>;
#[doc = "Field `ACFADR` reader - desc ACFADR"]
pub type AcfadrR = crate::FieldReader;
#[doc = "Field `ACFADR` writer - desc ACFADR"]
pub type AcfadrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SELMASK` reader - desc SELMASK"]
pub type SelmaskR = crate::BitReader;
#[doc = "Field `SELMASK` writer - desc SELMASK"]
pub type SelmaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - desc ACFADR"]
    #[inline(always)]
    pub fn acfadr(&self) -> AcfadrR {
        AcfadrR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 5 - desc SELMASK"]
    #[inline(always)]
    pub fn selmask(&self) -> SelmaskR {
        SelmaskR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc ACFADR"]
    #[inline(always)]
    pub fn acfadr(&mut self) -> AcfadrW<'_, AcfctrlSpec> {
        AcfadrW::new(self, 0)
    }
    #[doc = "Bit 5 - desc SELMASK"]
    #[inline(always)]
    pub fn selmask(&mut self) -> SelmaskW<'_, AcfctrlSpec> {
        SelmaskW::new(self, 5)
    }
}
#[doc = "desc ACFCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`acfctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acfctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcfctrlSpec;
impl crate::RegisterSpec for AcfctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`acfctrl::R`](R) reader structure"]
impl crate::Readable for AcfctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`acfctrl::W`](W) writer structure"]
impl crate::Writable for AcfctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACFCTRL to value 0"]
impl crate::Resettable for AcfctrlSpec {}
