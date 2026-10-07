#[doc = "Register `PEVNTIDR1` reader"]
pub type R = crate::R<Pevntidr1Spec>;
#[doc = "Field `PIN` reader - desc PIN"]
pub type PinR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - desc PIN"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "desc PEVNTIDR1\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pevntidr1Spec;
impl crate::RegisterSpec for Pevntidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pevntidr1::R`](R) reader structure"]
impl crate::Readable for Pevntidr1Spec {}
#[doc = "`reset()` method sets PEVNTIDR1 to value 0"]
impl crate::Resettable for Pevntidr1Spec {}
