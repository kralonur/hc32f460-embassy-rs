#[doc = "Register `CHSTAT` reader"]
pub type R = crate::R<ChstatSpec>;
#[doc = "Field `DMAACT` reader - desc DMAACT"]
pub type DmaactR = crate::BitReader;
#[doc = "Field `RCFGACT` reader - desc RCFGACT"]
pub type RcfgactR = crate::BitReader;
#[doc = "Field `CHACT` reader - desc CHACT"]
pub type ChactR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - desc DMAACT"]
    #[inline(always)]
    pub fn dmaact(&self) -> DmaactR {
        DmaactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RCFGACT"]
    #[inline(always)]
    pub fn rcfgact(&self) -> RcfgactR {
        RcfgactR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:19 - desc CHACT"]
    #[inline(always)]
    pub fn chact(&self) -> ChactR {
        ChactR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "desc CHSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`chstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChstatSpec;
impl crate::RegisterSpec for ChstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chstat::R`](R) reader structure"]
impl crate::Readable for ChstatSpec {}
#[doc = "`reset()` method sets CHSTAT to value 0"]
impl crate::Resettable for ChstatSpec {}
