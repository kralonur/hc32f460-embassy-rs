#[doc = "Register `GRXSTSR` reader"]
pub type R = crate::R<GrxstsrSpec>;
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
#[doc = "desc GRXSTSR\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrxstsrSpec;
impl crate::RegisterSpec for GrxstsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsr::R`](R) reader structure"]
impl crate::Readable for GrxstsrSpec {}
#[doc = "`reset()` method sets GRXSTSR to value 0"]
impl crate::Resettable for GrxstsrSpec {}
