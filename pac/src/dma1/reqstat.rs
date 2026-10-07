#[doc = "Register `REQSTAT` reader"]
pub type R = crate::R<ReqstatSpec>;
#[doc = "Field `CHREQ` reader - desc CHREQ"]
pub type ChreqR = crate::FieldReader;
#[doc = "Field `RCFGREQ` reader - desc RCFGREQ"]
pub type RcfgreqR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - desc CHREQ"]
    #[inline(always)]
    pub fn chreq(&self) -> ChreqR {
        ChreqR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 15 - desc RCFGREQ"]
    #[inline(always)]
    pub fn rcfgreq(&self) -> RcfgreqR {
        RcfgreqR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "desc REQSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`reqstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReqstatSpec;
impl crate::RegisterSpec for ReqstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reqstat::R`](R) reader structure"]
impl crate::Readable for ReqstatSpec {}
#[doc = "`reset()` method sets REQSTAT to value 0"]
impl crate::Resettable for ReqstatSpec {}
