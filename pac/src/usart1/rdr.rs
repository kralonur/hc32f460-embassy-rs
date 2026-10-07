#[doc = "Register `RDR` reader"]
pub type R = crate::R<RdrSpec>;
#[doc = "Field `RDR` reader - desc RDR"]
pub type RdrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - desc RDR"]
    #[inline(always)]
    pub fn rdr(&self) -> RdrR {
        RdrR::new(self.bits & 0x01ff)
    }
}
#[doc = "desc RDR\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdrSpec;
impl crate::RegisterSpec for RdrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RdrSpec {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RdrSpec {}
