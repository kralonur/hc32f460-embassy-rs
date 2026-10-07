#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Field `PORTINF` reader - desc PORTINF"]
pub type PortinfR = crate::BitReader;
#[doc = "Field `PWMSF` reader - desc PWMSF"]
pub type PwmsfR = crate::BitReader;
#[doc = "Field `CMPF` reader - desc CMPF"]
pub type CmpfR = crate::BitReader;
#[doc = "Field `OSF` reader - desc OSF"]
pub type OsfR = crate::BitReader;
#[doc = "Field `PORTINST` reader - desc PORTINST"]
pub type PortinstR = crate::BitReader;
#[doc = "Field `PWMST` reader - desc PWMST"]
pub type PwmstR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc PORTINF"]
    #[inline(always)]
    pub fn portinf(&self) -> PortinfR {
        PortinfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PWMSF"]
    #[inline(always)]
    pub fn pwmsf(&self) -> PwmsfR {
        PwmsfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CMPF"]
    #[inline(always)]
    pub fn cmpf(&self) -> CmpfR {
        CmpfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc OSF"]
    #[inline(always)]
    pub fn osf(&self) -> OsfR {
        OsfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PORTINST"]
    #[inline(always)]
    pub fn portinst(&self) -> PortinstR {
        PortinstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PWMST"]
    #[inline(always)]
    pub fn pwmst(&self) -> PwmstR {
        PwmstR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "desc STAT\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
