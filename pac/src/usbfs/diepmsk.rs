#[doc = "Register `DIEPMSK` reader"]
pub type R = crate::R<DiepmskSpec>;
#[doc = "Register `DIEPMSK` writer"]
pub type W = crate::W<DiepmskSpec>;
#[doc = "Field `XFRCM` reader - desc XFRCM"]
pub type XfrcmR = crate::BitReader;
#[doc = "Field `XFRCM` writer - desc XFRCM"]
pub type XfrcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDM` reader - desc EPDM"]
pub type EpdmR = crate::BitReader;
#[doc = "Field `EPDM` writer - desc EPDM"]
pub type EpdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOM` reader - desc TOM"]
pub type TomR = crate::BitReader;
#[doc = "Field `TOM` writer - desc TOM"]
pub type TomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTXFEMSK` reader - desc TTXFEMSK"]
pub type TtxfemskR = crate::BitReader;
#[doc = "Field `TTXFEMSK` writer - desc TTXFEMSK"]
pub type TtxfemskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNMM` reader - desc INEPNMM"]
pub type InepnmmR = crate::BitReader;
#[doc = "Field `INEPNMM` writer - desc INEPNMM"]
pub type InepnmmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNEM` reader - desc INEPNEM"]
pub type InepnemR = crate::BitReader;
#[doc = "Field `INEPNEM` writer - desc INEPNEM"]
pub type InepnemW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XfrcmR {
        XfrcmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EPDM"]
    #[inline(always)]
    pub fn epdm(&self) -> EpdmR {
        EpdmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TOM"]
    #[inline(always)]
    pub fn tom(&self) -> TomR {
        TomR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TTXFEMSK"]
    #[inline(always)]
    pub fn ttxfemsk(&self) -> TtxfemskR {
        TtxfemskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc INEPNMM"]
    #[inline(always)]
    pub fn inepnmm(&self) -> InepnmmR {
        InepnmmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc INEPNEM"]
    #[inline(always)]
    pub fn inepnem(&self) -> InepnemR {
        InepnemR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XfrcmW<'_, DiepmskSpec> {
        XfrcmW::new(self, 0)
    }
    #[doc = "Bit 1 - desc EPDM"]
    #[inline(always)]
    pub fn epdm(&mut self) -> EpdmW<'_, DiepmskSpec> {
        EpdmW::new(self, 1)
    }
    #[doc = "Bit 3 - desc TOM"]
    #[inline(always)]
    pub fn tom(&mut self) -> TomW<'_, DiepmskSpec> {
        TomW::new(self, 3)
    }
    #[doc = "Bit 4 - desc TTXFEMSK"]
    #[inline(always)]
    pub fn ttxfemsk(&mut self) -> TtxfemskW<'_, DiepmskSpec> {
        TtxfemskW::new(self, 4)
    }
    #[doc = "Bit 5 - desc INEPNMM"]
    #[inline(always)]
    pub fn inepnmm(&mut self) -> InepnmmW<'_, DiepmskSpec> {
        InepnmmW::new(self, 5)
    }
    #[doc = "Bit 6 - desc INEPNEM"]
    #[inline(always)]
    pub fn inepnem(&mut self) -> InepnemW<'_, DiepmskSpec> {
        InepnemW::new(self, 6)
    }
}
#[doc = "desc DIEPMSK\n\nYou can [`read`](crate::Reg::read) this register and get [`diepmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DiepmskSpec;
impl crate::RegisterSpec for DiepmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepmsk::R`](R) reader structure"]
impl crate::Readable for DiepmskSpec {}
#[doc = "`write(|w| ..)` method takes [`diepmsk::W`](W) writer structure"]
impl crate::Writable for DiepmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPMSK to value 0"]
impl crate::Resettable for DiepmskSpec {}
