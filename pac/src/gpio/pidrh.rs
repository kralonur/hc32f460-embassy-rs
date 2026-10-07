#[doc = "Register `PIDRH` reader"]
pub type R = crate::R<PidrhSpec>;
#[doc = "Field `PIN00` reader - desc PIN00"]
pub type Pin00R = crate::BitReader;
#[doc = "Field `PIN01` reader - desc PIN01"]
pub type Pin01R = crate::BitReader;
#[doc = "Field `PIN02` reader - desc PIN02"]
pub type Pin02R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc PIN00"]
    #[inline(always)]
    pub fn pin00(&self) -> Pin00R {
        Pin00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PIN01"]
    #[inline(always)]
    pub fn pin01(&self) -> Pin01R {
        Pin01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PIN02"]
    #[inline(always)]
    pub fn pin02(&self) -> Pin02R {
        Pin02R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "desc PIDRH\n\nYou can [`read`](crate::Reg::read) this register and get [`pidrh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PidrhSpec;
impl crate::RegisterSpec for PidrhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pidrh::R`](R) reader structure"]
impl crate::Readable for PidrhSpec {}
#[doc = "`reset()` method sets PIDRH to value 0"]
impl crate::Resettable for PidrhSpec {}
