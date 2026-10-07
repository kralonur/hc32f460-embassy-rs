#[doc = "Register `INTSTAT0` reader"]
pub type R = crate::R<Intstat0Spec>;
#[doc = "Field `TRNERR` reader - desc TRNERR"]
pub type TrnerrR = crate::FieldReader;
#[doc = "Field `REQERR` reader - desc REQERR"]
pub type ReqerrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - desc TRNERR"]
    #[inline(always)]
    pub fn trnerr(&self) -> TrnerrR {
        TrnerrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - desc REQERR"]
    #[inline(always)]
    pub fn reqerr(&self) -> ReqerrR {
        ReqerrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "desc INTSTAT0\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intstat0Spec;
impl crate::RegisterSpec for Intstat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat0::R`](R) reader structure"]
impl crate::Readable for Intstat0Spec {}
#[doc = "`reset()` method sets INTSTAT0 to value 0"]
impl crate::Resettable for Intstat0Spec {}
