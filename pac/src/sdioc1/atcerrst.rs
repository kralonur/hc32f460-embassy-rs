#[doc = "Register `ATCERRST` reader"]
pub type R = crate::R<AtcerrstSpec>;
#[doc = "Field `NE` reader - desc NE"]
pub type NeR = crate::BitReader;
#[doc = "Field `TOE` reader - desc TOE"]
pub type ToeR = crate::BitReader;
#[doc = "Field `CE` reader - desc CE"]
pub type CeR = crate::BitReader;
#[doc = "Field `EBE` reader - desc EBE"]
pub type EbeR = crate::BitReader;
#[doc = "Field `IE` reader - desc IE"]
pub type IeR = crate::BitReader;
#[doc = "Field `CMDE` reader - desc CMDE"]
pub type CmdeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc NE"]
    #[inline(always)]
    pub fn ne(&self) -> NeR {
        NeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TOE"]
    #[inline(always)]
    pub fn toe(&self) -> ToeR {
        ToeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CE"]
    #[inline(always)]
    pub fn ce(&self) -> CeR {
        CeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc EBE"]
    #[inline(always)]
    pub fn ebe(&self) -> EbeR {
        EbeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CMDE"]
    #[inline(always)]
    pub fn cmde(&self) -> CmdeR {
        CmdeR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "desc ATCERRST\n\nYou can [`read`](crate::Reg::read) this register and get [`atcerrst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtcerrstSpec;
impl crate::RegisterSpec for AtcerrstSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`atcerrst::R`](R) reader structure"]
impl crate::Readable for AtcerrstSpec {}
#[doc = "`reset()` method sets ATCERRST to value 0"]
impl crate::Resettable for AtcerrstSpec {}
