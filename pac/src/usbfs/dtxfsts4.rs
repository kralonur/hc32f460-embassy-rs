#[doc = "Register `DTXFSTS4` reader"]
pub type R = crate::R<Dtxfsts4Spec>;
#[doc = "Field `INEPTFSAV` reader - desc INEPTFSAV"]
pub type IneptfsavR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - desc INEPTFSAV"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> IneptfsavR {
        IneptfsavR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "desc DTXFSTS4\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dtxfsts4Spec;
impl crate::RegisterSpec for Dtxfsts4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts4::R`](R) reader structure"]
impl crate::Readable for Dtxfsts4Spec {}
#[doc = "`reset()` method sets DTXFSTS4 to value 0x0100"]
impl crate::Resettable for Dtxfsts4Spec {
    const RESET_VALUE: u32 = 0x0100;
}
