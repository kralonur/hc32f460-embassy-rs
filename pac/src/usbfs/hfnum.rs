#[doc = "Register `HFNUM` reader"]
pub type R = crate::R<HfnumSpec>;
#[doc = "Field `FRNUM` reader - desc FRNUM"]
pub type FrnumR = crate::FieldReader<u16>;
#[doc = "Field `FTREM` reader - desc FTREM"]
pub type FtremR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - desc FRNUM"]
    #[inline(always)]
    pub fn frnum(&self) -> FrnumR {
        FrnumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - desc FTREM"]
    #[inline(always)]
    pub fn ftrem(&self) -> FtremR {
        FtremR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "desc HFNUM\n\nYou can [`read`](crate::Reg::read) this register and get [`hfnum::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfnumSpec;
impl crate::RegisterSpec for HfnumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfnum::R`](R) reader structure"]
impl crate::Readable for HfnumSpec {}
#[doc = "`reset()` method sets HFNUM to value 0x3fff"]
impl crate::Resettable for HfnumSpec {
    const RESET_VALUE: u32 = 0x3fff;
}
