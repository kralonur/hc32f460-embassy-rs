#[doc = "Register `MCUDBGSTAT` reader"]
pub type R = crate::R<McudbgstatSpec>;
#[doc = "Register `MCUDBGSTAT` writer"]
pub type W = crate::W<McudbgstatSpec>;
#[doc = "Field `CDBGPWRUPREQ` reader - desc CDBGPWRUPREQ"]
pub type CdbgpwrupreqR = crate::BitReader;
#[doc = "Field `CDBGPWRUPREQ` writer - desc CDBGPWRUPREQ"]
pub type CdbgpwrupreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDBGPWRUPACK` reader - desc CDBGPWRUPACK"]
pub type CdbgpwrupackR = crate::BitReader;
#[doc = "Field `CDBGPWRUPACK` writer - desc CDBGPWRUPACK"]
pub type CdbgpwrupackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CDBGPWRUPREQ"]
    #[inline(always)]
    pub fn cdbgpwrupreq(&self) -> CdbgpwrupreqR {
        CdbgpwrupreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CDBGPWRUPACK"]
    #[inline(always)]
    pub fn cdbgpwrupack(&self) -> CdbgpwrupackR {
        CdbgpwrupackR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CDBGPWRUPREQ"]
    #[inline(always)]
    pub fn cdbgpwrupreq(&mut self) -> CdbgpwrupreqW<'_, McudbgstatSpec> {
        CdbgpwrupreqW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CDBGPWRUPACK"]
    #[inline(always)]
    pub fn cdbgpwrupack(&mut self) -> CdbgpwrupackW<'_, McudbgstatSpec> {
        CdbgpwrupackW::new(self, 1)
    }
}
#[doc = "desc MCUDBGSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`mcudbgstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcudbgstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McudbgstatSpec;
impl crate::RegisterSpec for McudbgstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcudbgstat::R`](R) reader structure"]
impl crate::Readable for McudbgstatSpec {}
#[doc = "`write(|w| ..)` method takes [`mcudbgstat::W`](W) writer structure"]
impl crate::Writable for McudbgstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCUDBGSTAT to value 0"]
impl crate::Resettable for McudbgstatSpec {}
