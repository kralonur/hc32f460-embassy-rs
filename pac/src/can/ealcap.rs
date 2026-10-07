#[doc = "Register `EALCAP` reader"]
pub type R = crate::R<EalcapSpec>;
#[doc = "Field `ALC` reader - desc ALC"]
pub type AlcR = crate::FieldReader;
#[doc = "Field `KOER` reader - desc KOER"]
pub type KoerR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - desc ALC"]
    #[inline(always)]
    pub fn alc(&self) -> AlcR {
        AlcR::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - desc KOER"]
    #[inline(always)]
    pub fn koer(&self) -> KoerR {
        KoerR::new((self.bits >> 5) & 7)
    }
}
#[doc = "desc EALCAP\n\nYou can [`read`](crate::Reg::read) this register and get [`ealcap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EalcapSpec;
impl crate::RegisterSpec for EalcapSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ealcap::R`](R) reader structure"]
impl crate::Readable for EalcapSpec {}
#[doc = "`reset()` method sets EALCAP to value 0"]
impl crate::Resettable for EalcapSpec {}
