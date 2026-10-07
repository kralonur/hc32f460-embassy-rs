#[doc = "Register `FRMC` reader"]
pub type R = crate::R<FrmcSpec>;
#[doc = "Register `FRMC` writer"]
pub type W = crate::W<FrmcSpec>;
#[doc = "Field `SLPMD` reader - desc SLPMD"]
pub type SlpmdR = crate::BitReader;
#[doc = "Field `SLPMD` writer - desc SLPMD"]
pub type SlpmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLWT` reader - desc FLWT"]
pub type FlwtR = crate::FieldReader;
#[doc = "Field `FLWT` writer - desc FLWT"]
pub type FlwtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LVM` reader - desc LVM"]
pub type LvmR = crate::BitReader;
#[doc = "Field `LVM` writer - desc LVM"]
pub type LvmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE` reader - desc CACHE"]
pub type CacheR = crate::BitReader;
#[doc = "Field `CACHE` writer - desc CACHE"]
pub type CacheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRST` reader - desc CRST"]
pub type CrstR = crate::BitReader;
#[doc = "Field `CRST` writer - desc CRST"]
pub type CrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SLPMD"]
    #[inline(always)]
    pub fn slpmd(&self) -> SlpmdR {
        SlpmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - desc FLWT"]
    #[inline(always)]
    pub fn flwt(&self) -> FlwtR {
        FlwtR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - desc LVM"]
    #[inline(always)]
    pub fn lvm(&self) -> LvmR {
        LvmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - desc CACHE"]
    #[inline(always)]
    pub fn cache(&self) -> CacheR {
        CacheR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - desc CRST"]
    #[inline(always)]
    pub fn crst(&self) -> CrstR {
        CrstR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SLPMD"]
    #[inline(always)]
    pub fn slpmd(&mut self) -> SlpmdW<'_, FrmcSpec> {
        SlpmdW::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc FLWT"]
    #[inline(always)]
    pub fn flwt(&mut self) -> FlwtW<'_, FrmcSpec> {
        FlwtW::new(self, 4)
    }
    #[doc = "Bit 8 - desc LVM"]
    #[inline(always)]
    pub fn lvm(&mut self) -> LvmW<'_, FrmcSpec> {
        LvmW::new(self, 8)
    }
    #[doc = "Bit 16 - desc CACHE"]
    #[inline(always)]
    pub fn cache(&mut self) -> CacheW<'_, FrmcSpec> {
        CacheW::new(self, 16)
    }
    #[doc = "Bit 24 - desc CRST"]
    #[inline(always)]
    pub fn crst(&mut self) -> CrstW<'_, FrmcSpec> {
        CrstW::new(self, 24)
    }
}
#[doc = "desc FRMC\n\nYou can [`read`](crate::Reg::read) this register and get [`frmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrmcSpec;
impl crate::RegisterSpec for FrmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frmc::R`](R) reader structure"]
impl crate::Readable for FrmcSpec {}
#[doc = "`write(|w| ..)` method takes [`frmc::W`](W) writer structure"]
impl crate::Writable for FrmcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRMC to value 0"]
impl crate::Resettable for FrmcSpec {}
