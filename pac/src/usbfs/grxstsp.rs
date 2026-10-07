#[doc = "Register `GRXSTSP` reader"]
pub type R = crate::R<GrxstspSpec>;
#[doc = "Field `CHNUM_EPNUM` reader - desc CHNUM_EPNUM"]
pub type ChnumEpnumR = crate::FieldReader;
#[doc = "Field `BCNT` reader - desc BCNT"]
pub type BcntR = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - desc DPID"]
pub type DpidR = crate::FieldReader;
#[doc = "Field `PKTSTS` reader - desc PKTSTS"]
pub type PktstsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - desc CHNUM_EPNUM"]
    #[inline(always)]
    pub fn chnum_epnum(&self) -> ChnumEpnumR {
        ChnumEpnumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - desc BCNT"]
    #[inline(always)]
    pub fn bcnt(&self) -> BcntR {
        BcntR::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - desc DPID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - desc PKTSTS"]
    #[inline(always)]
    pub fn pktsts(&self) -> PktstsR {
        PktstsR::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
#[doc = "desc GRXSTSP\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrxstspSpec;
impl crate::RegisterSpec for GrxstspSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsp::R`](R) reader structure"]
impl crate::Readable for GrxstspSpec {}
#[doc = "`reset()` method sets GRXSTSP to value 0"]
impl crate::Resettable for GrxstspSpec {}
