#[doc = "Register `HNPTXSTS` reader"]
pub type R = crate::R<HnptxstsSpec>;
#[doc = "Field `NPTXFSAV` reader - desc NPTXFSAV"]
pub type NptxfsavR = crate::FieldReader<u16>;
#[doc = "Field `NPTQXSAV` reader - desc NPTQXSAV"]
pub type NptqxsavR = crate::FieldReader;
#[doc = "Field `NPTXQTOP` reader - desc NPTXQTOP"]
pub type NptxqtopR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - desc NPTXFSAV"]
    #[inline(always)]
    pub fn nptxfsav(&self) -> NptxfsavR {
        NptxfsavR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - desc NPTQXSAV"]
    #[inline(always)]
    pub fn nptqxsav(&self) -> NptqxsavR {
        NptqxsavR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - desc NPTXQTOP"]
    #[inline(always)]
    pub fn nptxqtop(&self) -> NptxqtopR {
        NptxqtopR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "desc HNPTXSTS\n\nYou can [`read`](crate::Reg::read) this register and get [`hnptxsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HnptxstsSpec;
impl crate::RegisterSpec for HnptxstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hnptxsts::R`](R) reader structure"]
impl crate::Readable for HnptxstsSpec {}
#[doc = "`reset()` method sets HNPTXSTS to value 0x0008_0100"]
impl crate::Resettable for HnptxstsSpec {
    const RESET_VALUE: u32 = 0x0008_0100;
}
