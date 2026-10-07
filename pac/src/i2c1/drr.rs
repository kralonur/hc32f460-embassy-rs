#[doc = "Register `DRR` reader"]
pub type R = crate::R<DrrSpec>;
#[doc = "Field `DR` reader - desc DR"]
pub type DrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - desc DR"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new(self.bits)
    }
}
#[doc = "desc DRR\n\nYou can [`read`](crate::Reg::read) this register and get [`drr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrrSpec;
impl crate::RegisterSpec for DrrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`drr::R`](R) reader structure"]
impl crate::Readable for DrrSpec {}
#[doc = "`reset()` method sets DRR to value 0"]
impl crate::Resettable for DrrSpec {}
