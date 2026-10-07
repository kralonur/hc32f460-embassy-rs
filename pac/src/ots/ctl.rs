#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `OTSST` reader - desc OTSST"]
pub type OtsstR = crate::BitReader;
#[doc = "Field `OTSST` writer - desc OTSST"]
pub type OtsstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTSCK` reader - desc OTSCK"]
pub type OtsckR = crate::BitReader;
#[doc = "Field `OTSCK` writer - desc OTSCK"]
pub type OtsckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTSIE` reader - desc OTSIE"]
pub type OtsieR = crate::BitReader;
#[doc = "Field `OTSIE` writer - desc OTSIE"]
pub type OtsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSTP` reader - desc TSSTP"]
pub type TsstpR = crate::BitReader;
#[doc = "Field `TSSTP` writer - desc TSSTP"]
pub type TsstpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc OTSST"]
    #[inline(always)]
    pub fn otsst(&self) -> OtsstR {
        OtsstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc OTSCK"]
    #[inline(always)]
    pub fn otsck(&self) -> OtsckR {
        OtsckR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc OTSIE"]
    #[inline(always)]
    pub fn otsie(&self) -> OtsieR {
        OtsieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TSSTP"]
    #[inline(always)]
    pub fn tsstp(&self) -> TsstpR {
        TsstpR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc OTSST"]
    #[inline(always)]
    pub fn otsst(&mut self) -> OtsstW<'_, CtlSpec> {
        OtsstW::new(self, 0)
    }
    #[doc = "Bit 1 - desc OTSCK"]
    #[inline(always)]
    pub fn otsck(&mut self) -> OtsckW<'_, CtlSpec> {
        OtsckW::new(self, 1)
    }
    #[doc = "Bit 2 - desc OTSIE"]
    #[inline(always)]
    pub fn otsie(&mut self) -> OtsieW<'_, CtlSpec> {
        OtsieW::new(self, 2)
    }
    #[doc = "Bit 3 - desc TSSTP"]
    #[inline(always)]
    pub fn tsstp(&mut self) -> TsstpW<'_, CtlSpec> {
        TsstpW::new(self, 3)
    }
}
#[doc = "desc CTL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {}
