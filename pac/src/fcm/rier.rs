#[doc = "Register `RIER` reader"]
pub type R = crate::R<RierSpec>;
#[doc = "Register `RIER` writer"]
pub type W = crate::W<RierSpec>;
#[doc = "Field `ERRIE` reader - desc ERRIE"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - desc ERRIE"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MENDIE` reader - desc MENDIE"]
pub type MendieR = crate::BitReader;
#[doc = "Field `MENDIE` writer - desc MENDIE"]
pub type MendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFIE` reader - desc OVFIE"]
pub type OvfieR = crate::BitReader;
#[doc = "Field `OVFIE` writer - desc OVFIE"]
pub type OvfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRINTRS` reader - desc ERRINTRS"]
pub type ErrintrsR = crate::BitReader;
#[doc = "Field `ERRINTRS` writer - desc ERRINTRS"]
pub type ErrintrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRE` reader - desc ERRE"]
pub type ErreR = crate::BitReader;
#[doc = "Field `ERRE` writer - desc ERRE"]
pub type ErreW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc ERRIE"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc MENDIE"]
    #[inline(always)]
    pub fn mendie(&self) -> MendieR {
        MendieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc OVFIE"]
    #[inline(always)]
    pub fn ovfie(&self) -> OvfieR {
        OvfieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc ERRINTRS"]
    #[inline(always)]
    pub fn errintrs(&self) -> ErrintrsR {
        ErrintrsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - desc ERRE"]
    #[inline(always)]
    pub fn erre(&self) -> ErreR {
        ErreR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ERRIE"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, RierSpec> {
        ErrieW::new(self, 0)
    }
    #[doc = "Bit 1 - desc MENDIE"]
    #[inline(always)]
    pub fn mendie(&mut self) -> MendieW<'_, RierSpec> {
        MendieW::new(self, 1)
    }
    #[doc = "Bit 2 - desc OVFIE"]
    #[inline(always)]
    pub fn ovfie(&mut self) -> OvfieW<'_, RierSpec> {
        OvfieW::new(self, 2)
    }
    #[doc = "Bit 4 - desc ERRINTRS"]
    #[inline(always)]
    pub fn errintrs(&mut self) -> ErrintrsW<'_, RierSpec> {
        ErrintrsW::new(self, 4)
    }
    #[doc = "Bit 7 - desc ERRE"]
    #[inline(always)]
    pub fn erre(&mut self) -> ErreW<'_, RierSpec> {
        ErreW::new(self, 7)
    }
}
#[doc = "desc RIER\n\nYou can [`read`](crate::Reg::read) this register and get [`rier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RierSpec;
impl crate::RegisterSpec for RierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rier::R`](R) reader structure"]
impl crate::Readable for RierSpec {}
#[doc = "`write(|w| ..)` method takes [`rier::W`](W) writer structure"]
impl crate::Writable for RierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RIER to value 0"]
impl crate::Resettable for RierSpec {}
