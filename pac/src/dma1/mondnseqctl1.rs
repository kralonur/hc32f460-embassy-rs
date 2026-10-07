#[doc = "Register `MONDNSEQCTL1` reader"]
pub type R = crate::R<Mondnseqctl1Spec>;
#[doc = "Field `DOFFSET` reader - desc DOFFSET"]
pub type DoffsetR = crate::FieldReader<u32>;
#[doc = "Field `DNSCNT` reader - desc DNSCNT"]
pub type DnscntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:19 - desc DOFFSET"]
    #[inline(always)]
    pub fn doffset(&self) -> DoffsetR {
        DoffsetR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - desc DNSCNT"]
    #[inline(always)]
    pub fn dnscnt(&self) -> DnscntR {
        DnscntR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "desc MONDNSEQCTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`mondnseqctl1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mondnseqctl1Spec;
impl crate::RegisterSpec for Mondnseqctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mondnseqctl1::R`](R) reader structure"]
impl crate::Readable for Mondnseqctl1Spec {}
#[doc = "`reset()` method sets MONDNSEQCTL1 to value 0"]
impl crate::Resettable for Mondnseqctl1Spec {}
