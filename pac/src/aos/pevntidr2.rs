#[doc = "Register `PEVNTIDR2` reader"]
pub type R = crate::R<Pevntidr2Spec>;
#[doc = "Field `PIN` reader - desc PIN"]
pub type PinR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - desc PIN"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "desc PEVNTIDR2\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pevntidr2Spec;
impl crate::RegisterSpec for Pevntidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pevntidr2::R`](R) reader structure"]
impl crate::Readable for Pevntidr2Spec {}
#[doc = "`reset()` method sets PEVNTIDR2 to value 0"]
impl crate::Resettable for Pevntidr2Spec {}
