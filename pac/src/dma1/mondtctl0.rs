#[doc = "Register `MONDTCTL0` reader"]
pub type R = crate::R<Mondtctl0Spec>;
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
#[doc = "desc MONDTCTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`mondtctl0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mondtctl0Spec;
impl crate::RegisterSpec for Mondtctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mondtctl0::R`](R) reader structure"]
impl crate::Readable for Mondtctl0Spec {}
#[doc = "`reset()` method sets MONDTCTL0 to value 0x01"]
impl crate::Resettable for Mondtctl0Spec {
    const RESET_VALUE: u32 = 0x01;
}
