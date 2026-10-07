#[doc = "Register `OUTMON` reader"]
pub type R = crate::R<OutmonSpec>;
#[doc = "Field `OMON` reader - desc OMON"]
pub type OmonR = crate::BitReader;
#[doc = "Field `CVST` reader - desc CVST"]
pub type CvstR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - desc OMON"]
    #[inline(always)]
    pub fn omon(&self) -> OmonR {
        OmonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - desc CVST"]
    #[inline(always)]
    pub fn cvst(&self) -> CvstR {
        CvstR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "desc OUTMON\n\nYou can [`read`](crate::Reg::read) this register and get [`outmon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutmonSpec;
impl crate::RegisterSpec for OutmonSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`outmon::R`](R) reader structure"]
impl crate::Readable for OutmonSpec {}
#[doc = "`reset()` method sets OUTMON to value 0"]
impl crate::Resettable for OutmonSpec {}
