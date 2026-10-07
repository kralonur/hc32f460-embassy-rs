#[doc = "Register `RCTRL` reader"]
pub type R = crate::R<RctrlSpec>;
#[doc = "Register `RCTRL` writer"]
pub type W = crate::W<RctrlSpec>;
#[doc = "Field `RSTAT` reader - desc RSTAT"]
pub type RstatR = crate::FieldReader;
#[doc = "Field `RBALL` reader - desc RBALL"]
pub type RballR = crate::BitReader;
#[doc = "Field `RBALL` writer - desc RBALL"]
pub type RballW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RREL` reader - desc RREL"]
pub type RrelR = crate::BitReader;
#[doc = "Field `RREL` writer - desc RREL"]
pub type RrelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROV` reader - desc ROV"]
pub type RovR = crate::BitReader;
#[doc = "Field `ROM` reader - desc ROM"]
pub type RomR = crate::BitReader;
#[doc = "Field `ROM` writer - desc ROM"]
pub type RomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SACK` reader - desc SACK"]
pub type SackR = crate::BitReader;
#[doc = "Field `SACK` writer - desc SACK"]
pub type SackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc RSTAT"]
    #[inline(always)]
    pub fn rstat(&self) -> RstatR {
        RstatR::new(self.bits & 3)
    }
    #[doc = "Bit 3 - desc RBALL"]
    #[inline(always)]
    pub fn rball(&self) -> RballR {
        RballR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RREL"]
    #[inline(always)]
    pub fn rrel(&self) -> RrelR {
        RrelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ROV"]
    #[inline(always)]
    pub fn rov(&self) -> RovR {
        RovR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ROM"]
    #[inline(always)]
    pub fn rom(&self) -> RomR {
        RomR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc SACK"]
    #[inline(always)]
    pub fn sack(&self) -> SackR {
        SackR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - desc RBALL"]
    #[inline(always)]
    pub fn rball(&mut self) -> RballW<'_, RctrlSpec> {
        RballW::new(self, 3)
    }
    #[doc = "Bit 4 - desc RREL"]
    #[inline(always)]
    pub fn rrel(&mut self) -> RrelW<'_, RctrlSpec> {
        RrelW::new(self, 4)
    }
    #[doc = "Bit 6 - desc ROM"]
    #[inline(always)]
    pub fn rom(&mut self) -> RomW<'_, RctrlSpec> {
        RomW::new(self, 6)
    }
    #[doc = "Bit 7 - desc SACK"]
    #[inline(always)]
    pub fn sack(&mut self) -> SackW<'_, RctrlSpec> {
        SackW::new(self, 7)
    }
}
#[doc = "desc RCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`rctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RctrlSpec;
impl crate::RegisterSpec for RctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rctrl::R`](R) reader structure"]
impl crate::Readable for RctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rctrl::W`](W) writer structure"]
impl crate::Writable for RctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCTRL to value 0"]
impl crate::Resettable for RctrlSpec {}
