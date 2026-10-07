#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `PE` reader - desc PE"]
pub type PeR = crate::BitReader;
#[doc = "Field `FE` reader - desc FE"]
pub type FeR = crate::BitReader;
#[doc = "Field `ORE` reader - desc ORE"]
pub type OreR = crate::BitReader;
#[doc = "Field `RXNE` reader - desc RXNE"]
pub type RxneR = crate::BitReader;
#[doc = "Field `TC` reader - desc TC"]
pub type TcR = crate::BitReader;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `RTOF` reader - desc RTOF"]
pub type RtofR = crate::BitReader;
#[doc = "Field `MPB` reader - desc MPB"]
pub type MpbR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc PE"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FE"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc ORE"]
    #[inline(always)]
    pub fn ore(&self) -> OreR {
        OreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TC"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RTOF"]
    #[inline(always)]
    pub fn rtof(&self) -> RtofR {
        RtofR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - desc MPB"]
    #[inline(always)]
    pub fn mpb(&self) -> MpbR {
        MpbR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "desc SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0xc0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0xc0;
}
