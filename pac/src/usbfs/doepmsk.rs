#[doc = "Register `DOEPMSK` reader"]
pub type R = crate::R<DoepmskSpec>;
#[doc = "Register `DOEPMSK` writer"]
pub type W = crate::W<DoepmskSpec>;
#[doc = "Field `XFRCM` reader - desc XFRCM"]
pub type XfrcmR = crate::BitReader;
#[doc = "Field `XFRCM` writer - desc XFRCM"]
pub type XfrcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDM` reader - desc EPDM"]
pub type EpdmR = crate::BitReader;
#[doc = "Field `EPDM` writer - desc EPDM"]
pub type EpdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUPM` reader - desc STUPM"]
pub type StupmR = crate::BitReader;
#[doc = "Field `STUPM` writer - desc STUPM"]
pub type StupmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTEPDM` reader - desc OTEPDM"]
pub type OtepdmR = crate::BitReader;
#[doc = "Field `OTEPDM` writer - desc OTEPDM"]
pub type OtepdmW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - desc STUPM"]
    #[inline(always)]
    pub fn stupm(&self) -> StupmR {
        StupmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc OTEPDM"]
    #[inline(always)]
    pub fn otepdm(&self) -> OtepdmR {
        OtepdmR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XfrcmW<'_, DoepmskSpec> {
        XfrcmW::new(self, 0)
    }
    #[doc = "Bit 1 - desc EPDM"]
    #[inline(always)]
    pub fn epdm(&mut self) -> EpdmW<'_, DoepmskSpec> {
        EpdmW::new(self, 1)
    }
    #[doc = "Bit 3 - desc STUPM"]
    #[inline(always)]
    pub fn stupm(&mut self) -> StupmW<'_, DoepmskSpec> {
        StupmW::new(self, 3)
    }
    #[doc = "Bit 4 - desc OTEPDM"]
    #[inline(always)]
    pub fn otepdm(&mut self) -> OtepdmW<'_, DoepmskSpec> {
        OtepdmW::new(self, 4)
    }
}
#[doc = "desc DOEPMSK\n\nYou can [`read`](crate::Reg::read) this register and get [`doepmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoepmskSpec;
impl crate::RegisterSpec for DoepmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepmsk::R`](R) reader structure"]
impl crate::Readable for DoepmskSpec {}
#[doc = "`write(|w| ..)` method takes [`doepmsk::W`](W) writer structure"]
impl crate::Writable for DoepmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPMSK to value 0"]
impl crate::Resettable for DoepmskSpec {}
