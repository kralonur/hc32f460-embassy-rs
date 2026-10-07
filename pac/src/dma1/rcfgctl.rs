#[doc = "Register `RCFGCTL` reader"]
pub type R = crate::R<RcfgctlSpec>;
#[doc = "Register `RCFGCTL` writer"]
pub type W = crate::W<RcfgctlSpec>;
#[doc = "Field `RCFGEN` reader - desc RCFGEN"]
pub type RcfgenR = crate::BitReader;
#[doc = "Field `RCFGEN` writer - desc RCFGEN"]
pub type RcfgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCFGLLP` reader - desc RCFGLLP"]
pub type RcfgllpR = crate::BitReader;
#[doc = "Field `RCFGLLP` writer - desc RCFGLLP"]
pub type RcfgllpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCFGCHS` reader - desc RCFGCHS"]
pub type RcfgchsR = crate::FieldReader;
#[doc = "Field `RCFGCHS` writer - desc RCFGCHS"]
pub type RcfgchsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SARMD` reader - desc SARMD"]
pub type SarmdR = crate::FieldReader;
#[doc = "Field `SARMD` writer - desc SARMD"]
pub type SarmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DARMD` reader - desc DARMD"]
pub type DarmdR = crate::FieldReader;
#[doc = "Field `DARMD` writer - desc DARMD"]
pub type DarmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNTMD` reader - desc CNTMD"]
pub type CntmdR = crate::FieldReader;
#[doc = "Field `CNTMD` writer - desc CNTMD"]
pub type CntmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc RCFGEN"]
    #[inline(always)]
    pub fn rcfgen(&self) -> RcfgenR {
        RcfgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RCFGLLP"]
    #[inline(always)]
    pub fn rcfgllp(&self) -> RcfgllpR {
        RcfgllpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - desc RCFGCHS"]
    #[inline(always)]
    pub fn rcfgchs(&self) -> RcfgchsR {
        RcfgchsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - desc SARMD"]
    #[inline(always)]
    pub fn sarmd(&self) -> SarmdR {
        SarmdR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - desc DARMD"]
    #[inline(always)]
    pub fn darmd(&self) -> DarmdR {
        DarmdR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - desc CNTMD"]
    #[inline(always)]
    pub fn cntmd(&self) -> CntmdR {
        CntmdR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc RCFGEN"]
    #[inline(always)]
    pub fn rcfgen(&mut self) -> RcfgenW<'_, RcfgctlSpec> {
        RcfgenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RCFGLLP"]
    #[inline(always)]
    pub fn rcfgllp(&mut self) -> RcfgllpW<'_, RcfgctlSpec> {
        RcfgllpW::new(self, 1)
    }
    #[doc = "Bits 8:11 - desc RCFGCHS"]
    #[inline(always)]
    pub fn rcfgchs(&mut self) -> RcfgchsW<'_, RcfgctlSpec> {
        RcfgchsW::new(self, 8)
    }
    #[doc = "Bits 16:17 - desc SARMD"]
    #[inline(always)]
    pub fn sarmd(&mut self) -> SarmdW<'_, RcfgctlSpec> {
        SarmdW::new(self, 16)
    }
    #[doc = "Bits 18:19 - desc DARMD"]
    #[inline(always)]
    pub fn darmd(&mut self) -> DarmdW<'_, RcfgctlSpec> {
        DarmdW::new(self, 18)
    }
    #[doc = "Bits 20:21 - desc CNTMD"]
    #[inline(always)]
    pub fn cntmd(&mut self) -> CntmdW<'_, RcfgctlSpec> {
        CntmdW::new(self, 20)
    }
}
#[doc = "desc RCFGCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`rcfgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcfgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcfgctlSpec;
impl crate::RegisterSpec for RcfgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcfgctl::R`](R) reader structure"]
impl crate::Readable for RcfgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rcfgctl::W`](W) writer structure"]
impl crate::Writable for RcfgctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCFGCTL to value 0"]
impl crate::Resettable for RcfgctlSpec {}
