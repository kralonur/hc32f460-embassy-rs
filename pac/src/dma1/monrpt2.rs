#[doc = "Register `MONRPT2` reader"]
pub type R = crate::R<Monrpt2Spec>;
#[doc = "Field `SRPT` reader - desc SRPT"]
pub type SrptR = crate::FieldReader<u16>;
#[doc = "Field `DRPT` reader - desc DRPT"]
pub type DrptR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - desc SRPT"]
    #[inline(always)]
    pub fn srpt(&self) -> SrptR {
        SrptR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - desc DRPT"]
    #[inline(always)]
    pub fn drpt(&self) -> DrptR {
        DrptR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "desc MONRPT2\n\nYou can [`read`](crate::Reg::read) this register and get [`monrpt2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Monrpt2Spec;
impl crate::RegisterSpec for Monrpt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`monrpt2::R`](R) reader structure"]
impl crate::Readable for Monrpt2Spec {}
#[doc = "`reset()` method sets MONRPT2 to value 0"]
impl crate::Resettable for Monrpt2Spec {}
