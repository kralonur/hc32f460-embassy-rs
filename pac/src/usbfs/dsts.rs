#[doc = "Register `DSTS` reader"]
pub type R = crate::R<DstsSpec>;
#[doc = "Field `SUSPSTS` reader - desc SUSPSTS"]
pub type SuspstsR = crate::BitReader;
#[doc = "Field `ENUMSPD` reader - desc ENUMSPD"]
pub type EnumspdR = crate::FieldReader;
#[doc = "Field `EERR` reader - desc EERR"]
pub type EerrR = crate::BitReader;
#[doc = "Field `FNSOF` reader - desc FNSOF"]
pub type FnsofR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - desc SUSPSTS"]
    #[inline(always)]
    pub fn suspsts(&self) -> SuspstsR {
        SuspstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc ENUMSPD"]
    #[inline(always)]
    pub fn enumspd(&self) -> EnumspdR {
        EnumspdR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - desc EERR"]
    #[inline(always)]
    pub fn eerr(&self) -> EerrR {
        EerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21 - desc FNSOF"]
    #[inline(always)]
    pub fn fnsof(&self) -> FnsofR {
        FnsofR::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
#[doc = "desc DSTS\n\nYou can [`read`](crate::Reg::read) this register and get [`dsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstsSpec;
impl crate::RegisterSpec for DstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsts::R`](R) reader structure"]
impl crate::Readable for DstsSpec {}
#[doc = "`reset()` method sets DSTS to value 0x02"]
impl crate::Resettable for DstsSpec {
    const RESET_VALUE: u32 = 0x02;
}
