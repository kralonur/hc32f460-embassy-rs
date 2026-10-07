#[doc = "Register `MONDTCTL3` reader"]
pub type R = crate::R<Mondtctl3Spec>;
#[doc = "Field `BLKSIZE` reader - desc BLKSIZE"]
pub type BlksizeR = crate::FieldReader<u16>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - desc BLKSIZE"]
    #[inline(always)]
    pub fn blksize(&self) -> BlksizeR {
        BlksizeR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "desc MONDTCTL3\n\nYou can [`read`](crate::Reg::read) this register and get [`mondtctl3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mondtctl3Spec;
impl crate::RegisterSpec for Mondtctl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mondtctl3::R`](R) reader structure"]
impl crate::Readable for Mondtctl3Spec {}
#[doc = "`reset()` method sets MONDTCTL3 to value 0x01"]
impl crate::Resettable for Mondtctl3Spec {
    const RESET_VALUE: u32 = 0x01;
}
