#[doc = "Register `DTXFSTS5` reader"]
pub type R = crate::R<Dtxfsts5Spec>;
#[doc = "Field `INEPTFSAV` reader - desc INEPTFSAV"]
pub type IneptfsavR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - desc INEPTFSAV"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> IneptfsavR {
        IneptfsavR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "desc DTXFSTS5\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dtxfsts5Spec;
impl crate::RegisterSpec for Dtxfsts5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts5::R`](R) reader structure"]
impl crate::Readable for Dtxfsts5Spec {}
#[doc = "`reset()` method sets DTXFSTS5 to value 0x0100"]
impl crate::Resettable for Dtxfsts5Spec {
    const RESET_VALUE: u32 = 0x0100;
}
