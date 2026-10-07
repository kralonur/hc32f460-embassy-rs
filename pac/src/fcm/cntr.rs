#[doc = "Register `CNTR` reader"]
pub type R = crate::R<CntrSpec>;
#[doc = "Field `CNTR` reader - desc CNTR"]
pub type CntrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - desc CNTR"]
    #[inline(always)]
    pub fn cntr(&self) -> CntrR {
        CntrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "desc CNTR\n\nYou can [`read`](crate::Reg::read) this register and get [`cntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntrSpec;
impl crate::RegisterSpec for CntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntr::R`](R) reader structure"]
impl crate::Readable for CntrSpec {}
#[doc = "`reset()` method sets CNTR to value 0"]
impl crate::Resettable for CntrSpec {}
