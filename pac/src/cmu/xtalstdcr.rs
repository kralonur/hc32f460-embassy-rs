#[doc = "Register `XTALSTDCR` reader"]
pub type R = crate::R<XtalstdcrSpec>;
#[doc = "Register `XTALSTDCR` writer"]
pub type W = crate::W<XtalstdcrSpec>;
#[doc = "Field `XTALSTDIE` reader - desc XTALSTDIE"]
pub type XtalstdieR = crate::BitReader;
#[doc = "Field `XTALSTDIE` writer - desc XTALSTDIE"]
pub type XtalstdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALSTDRE` reader - desc XTALSTDRE"]
pub type XtalstdreR = crate::BitReader;
#[doc = "Field `XTALSTDRE` writer - desc XTALSTDRE"]
pub type XtalstdreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALSTDRIS` reader - desc XTALSTDRIS"]
pub type XtalstdrisR = crate::BitReader;
#[doc = "Field `XTALSTDRIS` writer - desc XTALSTDRIS"]
pub type XtalstdrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALSTDE` reader - desc XTALSTDE"]
pub type XtalstdeR = crate::BitReader;
#[doc = "Field `XTALSTDE` writer - desc XTALSTDE"]
pub type XtalstdeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc XTALSTDIE"]
    #[inline(always)]
    pub fn xtalstdie(&self) -> XtalstdieR {
        XtalstdieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc XTALSTDRE"]
    #[inline(always)]
    pub fn xtalstdre(&self) -> XtalstdreR {
        XtalstdreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc XTALSTDRIS"]
    #[inline(always)]
    pub fn xtalstdris(&self) -> XtalstdrisR {
        XtalstdrisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - desc XTALSTDE"]
    #[inline(always)]
    pub fn xtalstde(&self) -> XtalstdeR {
        XtalstdeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc XTALSTDIE"]
    #[inline(always)]
    pub fn xtalstdie(&mut self) -> XtalstdieW<'_, XtalstdcrSpec> {
        XtalstdieW::new(self, 0)
    }
    #[doc = "Bit 1 - desc XTALSTDRE"]
    #[inline(always)]
    pub fn xtalstdre(&mut self) -> XtalstdreW<'_, XtalstdcrSpec> {
        XtalstdreW::new(self, 1)
    }
    #[doc = "Bit 2 - desc XTALSTDRIS"]
    #[inline(always)]
    pub fn xtalstdris(&mut self) -> XtalstdrisW<'_, XtalstdcrSpec> {
        XtalstdrisW::new(self, 2)
    }
    #[doc = "Bit 7 - desc XTALSTDE"]
    #[inline(always)]
    pub fn xtalstde(&mut self) -> XtalstdeW<'_, XtalstdcrSpec> {
        XtalstdeW::new(self, 7)
    }
}
#[doc = "desc XTALSTDCR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalstdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalstdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalstdcrSpec;
impl crate::RegisterSpec for XtalstdcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xtalstdcr::R`](R) reader structure"]
impl crate::Readable for XtalstdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`xtalstdcr::W`](W) writer structure"]
impl crate::Writable for XtalstdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTALSTDCR to value 0"]
impl crate::Resettable for XtalstdcrSpec {}
