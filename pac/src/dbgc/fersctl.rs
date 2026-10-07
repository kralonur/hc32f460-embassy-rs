#[doc = "Register `FERSCTL` reader"]
pub type R = crate::R<FersctlSpec>;
#[doc = "Register `FERSCTL` writer"]
pub type W = crate::W<FersctlSpec>;
#[doc = "Field `ERASEREQ` reader - desc ERASEREQ"]
pub type ErasereqR = crate::BitReader;
#[doc = "Field `ERASEREQ` writer - desc ERASEREQ"]
pub type ErasereqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASEACK` reader - desc ERASEACK"]
pub type EraseackR = crate::BitReader;
#[doc = "Field `ERASEACK` writer - desc ERASEACK"]
pub type EraseackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASEERR` reader - desc ERASEERR"]
pub type EraseerrR = crate::BitReader;
#[doc = "Field `ERASEERR` writer - desc ERASEERR"]
pub type EraseerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc ERASEREQ"]
    #[inline(always)]
    pub fn erasereq(&self) -> ErasereqR {
        ErasereqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ERASEACK"]
    #[inline(always)]
    pub fn eraseack(&self) -> EraseackR {
        EraseackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ERASEERR"]
    #[inline(always)]
    pub fn eraseerr(&self) -> EraseerrR {
        EraseerrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ERASEREQ"]
    #[inline(always)]
    pub fn erasereq(&mut self) -> ErasereqW<'_, FersctlSpec> {
        ErasereqW::new(self, 0)
    }
    #[doc = "Bit 1 - desc ERASEACK"]
    #[inline(always)]
    pub fn eraseack(&mut self) -> EraseackW<'_, FersctlSpec> {
        EraseackW::new(self, 1)
    }
    #[doc = "Bit 2 - desc ERASEERR"]
    #[inline(always)]
    pub fn eraseerr(&mut self) -> EraseerrW<'_, FersctlSpec> {
        EraseerrW::new(self, 2)
    }
}
#[doc = "desc FERSCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`fersctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fersctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FersctlSpec;
impl crate::RegisterSpec for FersctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fersctl::R`](R) reader structure"]
impl crate::Readable for FersctlSpec {}
#[doc = "`write(|w| ..)` method takes [`fersctl::W`](W) writer structure"]
impl crate::Writable for FersctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FERSCTL to value 0"]
impl crate::Resettable for FersctlSpec {}
