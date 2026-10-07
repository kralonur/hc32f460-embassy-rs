#[doc = "Register `FSR` reader"]
pub type R = crate::R<FsrSpec>;
#[doc = "Field `PEWERR` reader - desc PEWERR"]
pub type PewerrR = crate::BitReader;
#[doc = "Field `PEPRTERR` reader - desc PEPRTERR"]
pub type PeprterrR = crate::BitReader;
#[doc = "Field `PGSZERR` reader - desc PGSZERR"]
pub type PgszerrR = crate::BitReader;
#[doc = "Field `PGMISMTCH` reader - desc PGMISMTCH"]
pub type PgmismtchR = crate::BitReader;
#[doc = "Field `OPTEND` reader - desc OPTEND"]
pub type OptendR = crate::BitReader;
#[doc = "Field `COLERR` reader - desc COLERR"]
pub type ColerrR = crate::BitReader;
#[doc = "Field `RDY` reader - desc RDY"]
pub type RdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc PEWERR"]
    #[inline(always)]
    pub fn pewerr(&self) -> PewerrR {
        PewerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PEPRTERR"]
    #[inline(always)]
    pub fn peprterr(&self) -> PeprterrR {
        PeprterrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PGSZERR"]
    #[inline(always)]
    pub fn pgszerr(&self) -> PgszerrR {
        PgszerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PGMISMTCH"]
    #[inline(always)]
    pub fn pgmismtch(&self) -> PgmismtchR {
        PgmismtchR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc OPTEND"]
    #[inline(always)]
    pub fn optend(&self) -> OptendR {
        OptendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc COLERR"]
    #[inline(always)]
    pub fn colerr(&self) -> ColerrR {
        ColerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RDY"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "desc FSR\n\nYou can [`read`](crate::Reg::read) this register and get [`fsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsrSpec;
impl crate::RegisterSpec for FsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsr::R`](R) reader structure"]
impl crate::Readable for FsrSpec {}
#[doc = "`reset()` method sets FSR to value 0x0100"]
impl crate::Resettable for FsrSpec {
    const RESET_VALUE: u32 = 0x0100;
}
