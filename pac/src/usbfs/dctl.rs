#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DctlSpec>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DctlSpec>;
#[doc = "Field `RWUSIG` reader - desc RWUSIG"]
pub type RwusigR = crate::BitReader;
#[doc = "Field `RWUSIG` writer - desc RWUSIG"]
pub type RwusigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIS` reader - desc SDIS"]
pub type SdisR = crate::BitReader;
#[doc = "Field `SDIS` writer - desc SDIS"]
pub type SdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINSTS` reader - desc GINSTS"]
pub type GinstsR = crate::BitReader;
#[doc = "Field `GONSTS` reader - desc GONSTS"]
pub type GonstsR = crate::BitReader;
#[doc = "Field `SGINAK` writer - desc SGINAK"]
pub type SginakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGINAK` writer - desc CGINAK"]
pub type CginakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGONAK` writer - desc SGONAK"]
pub type SgonakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGONAK` writer - desc CGONAK"]
pub type CgonakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POPRGDNE` reader - desc POPRGDNE"]
pub type PoprgdneR = crate::BitReader;
#[doc = "Field `POPRGDNE` writer - desc POPRGDNE"]
pub type PoprgdneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc RWUSIG"]
    #[inline(always)]
    pub fn rwusig(&self) -> RwusigR {
        RwusigR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SDIS"]
    #[inline(always)]
    pub fn sdis(&self) -> SdisR {
        SdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc GINSTS"]
    #[inline(always)]
    pub fn ginsts(&self) -> GinstsR {
        GinstsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc GONSTS"]
    #[inline(always)]
    pub fn gonsts(&self) -> GonstsR {
        GonstsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - desc POPRGDNE"]
    #[inline(always)]
    pub fn poprgdne(&self) -> PoprgdneR {
        PoprgdneR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc RWUSIG"]
    #[inline(always)]
    pub fn rwusig(&mut self) -> RwusigW<'_, DctlSpec> {
        RwusigW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SDIS"]
    #[inline(always)]
    pub fn sdis(&mut self) -> SdisW<'_, DctlSpec> {
        SdisW::new(self, 1)
    }
    #[doc = "Bit 7 - desc SGINAK"]
    #[inline(always)]
    pub fn sginak(&mut self) -> SginakW<'_, DctlSpec> {
        SginakW::new(self, 7)
    }
    #[doc = "Bit 8 - desc CGINAK"]
    #[inline(always)]
    pub fn cginak(&mut self) -> CginakW<'_, DctlSpec> {
        CginakW::new(self, 8)
    }
    #[doc = "Bit 9 - desc SGONAK"]
    #[inline(always)]
    pub fn sgonak(&mut self) -> SgonakW<'_, DctlSpec> {
        SgonakW::new(self, 9)
    }
    #[doc = "Bit 10 - desc CGONAK"]
    #[inline(always)]
    pub fn cgonak(&mut self) -> CgonakW<'_, DctlSpec> {
        CgonakW::new(self, 10)
    }
    #[doc = "Bit 11 - desc POPRGDNE"]
    #[inline(always)]
    pub fn poprgdne(&mut self) -> PoprgdneW<'_, DctlSpec> {
        PoprgdneW::new(self, 11)
    }
}
#[doc = "desc DCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`dctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DctlSpec;
impl crate::RegisterSpec for DctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCTL to value 0x02"]
impl crate::Resettable for DctlSpec {
    const RESET_VALUE: u32 = 0x02;
}
