#[doc = "Register `HPTXSTS` reader"]
pub type R = crate::R<HptxstsSpec>;
#[doc = "Field `PTXFSAVL` reader - desc PTXFSAVL"]
pub type PtxfsavlR = crate::FieldReader<u16>;
#[doc = "Field `PTXQSAV` reader - desc PTXQSAV"]
pub type PtxqsavR = crate::FieldReader;
#[doc = "Field `PTXQTOP` reader - desc PTXQTOP"]
pub type PtxqtopR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - desc PTXFSAVL"]
    #[inline(always)]
    pub fn ptxfsavl(&self) -> PtxfsavlR {
        PtxfsavlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - desc PTXQSAV"]
    #[inline(always)]
    pub fn ptxqsav(&self) -> PtxqsavR {
        PtxqsavR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - desc PTXQTOP"]
    #[inline(always)]
    pub fn ptxqtop(&self) -> PtxqtopR {
        PtxqtopR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "desc HPTXSTS\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HptxstsSpec;
impl crate::RegisterSpec for HptxstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxsts::R`](R) reader structure"]
impl crate::Readable for HptxstsSpec {}
#[doc = "`reset()` method sets HPTXSTS to value 0x0008_0100"]
impl crate::Resettable for HptxstsSpec {
    const RESET_VALUE: u32 = 0x0008_0100;
}
