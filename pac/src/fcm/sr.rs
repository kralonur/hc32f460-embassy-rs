#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `ERRF` reader - desc ERRF"]
pub type ErrfR = crate::BitReader;
#[doc = "Field `MENDF` reader - desc MENDF"]
pub type MendfR = crate::BitReader;
#[doc = "Field `OVF` reader - desc OVF"]
pub type OvfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc ERRF"]
    #[inline(always)]
    pub fn errf(&self) -> ErrfR {
        ErrfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc MENDF"]
    #[inline(always)]
    pub fn mendf(&self) -> MendfR {
        MendfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc OVF"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "desc SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
