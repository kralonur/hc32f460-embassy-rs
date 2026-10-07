#[doc = "Register `FLG` reader"]
pub type R = crate::R<FlgSpec>;
#[doc = "Field `CRCFLAG_32` reader - desc CRCFLAG_32"]
pub type Crcflag32R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc CRCFLAG_32"]
    #[inline(always)]
    pub fn crcflag_32(&self) -> Crcflag32R {
        Crcflag32R::new((self.bits & 1) != 0)
    }
}
#[doc = "desc FLG\n\nYou can [`read`](crate::Reg::read) this register and get [`flg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlgSpec;
impl crate::RegisterSpec for FlgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flg::R`](R) reader structure"]
impl crate::Readable for FlgSpec {}
#[doc = "`reset()` method sets FLG to value 0x01"]
impl crate::Resettable for FlgSpec {
    const RESET_VALUE: u32 = 0x01;
}
