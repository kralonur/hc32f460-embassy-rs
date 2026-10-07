#[doc = "Register `OSCSTBSR` reader"]
pub type R = crate::R<OscstbsrSpec>;
#[doc = "Register `OSCSTBSR` writer"]
pub type W = crate::W<OscstbsrSpec>;
#[doc = "Field `HRCSTBF` reader - desc HRCSTBF"]
pub type HrcstbfR = crate::BitReader;
#[doc = "Field `HRCSTBF` writer - desc HRCSTBF"]
pub type HrcstbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALSTBF` reader - desc XTALSTBF"]
pub type XtalstbfR = crate::BitReader;
#[doc = "Field `XTALSTBF` writer - desc XTALSTBF"]
pub type XtalstbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPLLSTBF` reader - desc MPLLSTBF"]
pub type MpllstbfR = crate::BitReader;
#[doc = "Field `MPLLSTBF` writer - desc MPLLSTBF"]
pub type MpllstbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPLLSTBF` reader - desc UPLLSTBF"]
pub type UpllstbfR = crate::BitReader;
#[doc = "Field `UPLLSTBF` writer - desc UPLLSTBF"]
pub type UpllstbfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HRCSTBF"]
    #[inline(always)]
    pub fn hrcstbf(&self) -> HrcstbfR {
        HrcstbfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - desc XTALSTBF"]
    #[inline(always)]
    pub fn xtalstbf(&self) -> XtalstbfR {
        XtalstbfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - desc MPLLSTBF"]
    #[inline(always)]
    pub fn mpllstbf(&self) -> MpllstbfR {
        MpllstbfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc UPLLSTBF"]
    #[inline(always)]
    pub fn upllstbf(&self) -> UpllstbfR {
        UpllstbfR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HRCSTBF"]
    #[inline(always)]
    pub fn hrcstbf(&mut self) -> HrcstbfW<'_, OscstbsrSpec> {
        HrcstbfW::new(self, 0)
    }
    #[doc = "Bit 3 - desc XTALSTBF"]
    #[inline(always)]
    pub fn xtalstbf(&mut self) -> XtalstbfW<'_, OscstbsrSpec> {
        XtalstbfW::new(self, 3)
    }
    #[doc = "Bit 5 - desc MPLLSTBF"]
    #[inline(always)]
    pub fn mpllstbf(&mut self) -> MpllstbfW<'_, OscstbsrSpec> {
        MpllstbfW::new(self, 5)
    }
    #[doc = "Bit 6 - desc UPLLSTBF"]
    #[inline(always)]
    pub fn upllstbf(&mut self) -> UpllstbfW<'_, OscstbsrSpec> {
        UpllstbfW::new(self, 6)
    }
}
#[doc = "desc OSCSTBSR\n\nYou can [`read`](crate::Reg::read) this register and get [`oscstbsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscstbsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscstbsrSpec;
impl crate::RegisterSpec for OscstbsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`oscstbsr::R`](R) reader structure"]
impl crate::Readable for OscstbsrSpec {}
#[doc = "`write(|w| ..)` method takes [`oscstbsr::W`](W) writer structure"]
impl crate::Writable for OscstbsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSCSTBSR to value 0"]
impl crate::Resettable for OscstbsrSpec {}
