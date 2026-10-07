#[doc = "Register `MONSNSEQCTL0` reader"]
pub type R = crate::R<Monsnseqctl0Spec>;
#[doc = "Field `SOFFSET` reader - desc SOFFSET"]
pub type SoffsetR = crate::FieldReader<u32>;
#[doc = "Field `SNSCNT` reader - desc SNSCNT"]
pub type SnscntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:19 - desc SOFFSET"]
    #[inline(always)]
    pub fn soffset(&self) -> SoffsetR {
        SoffsetR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - desc SNSCNT"]
    #[inline(always)]
    pub fn snscnt(&self) -> SnscntR {
        SnscntR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "desc MONSNSEQCTL0\n\nYou can [`read`](crate::Reg::read) this register and get [`monsnseqctl0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Monsnseqctl0Spec;
impl crate::RegisterSpec for Monsnseqctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`monsnseqctl0::R`](R) reader structure"]
impl crate::Readable for Monsnseqctl0Spec {}
#[doc = "`reset()` method sets MONSNSEQCTL0 to value 0"]
impl crate::Resettable for Monsnseqctl0Spec {}
