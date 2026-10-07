#[doc = "Register `PSTAT` reader"]
pub type R = crate::R<PstatSpec>;
#[doc = "Field `CIC` reader - desc CIC"]
pub type CicR = crate::BitReader;
#[doc = "Field `CID` reader - desc CID"]
pub type CidR = crate::BitReader;
#[doc = "Field `DA` reader - desc DA"]
pub type DaR = crate::BitReader;
#[doc = "Field `WTA` reader - desc WTA"]
pub type WtaR = crate::BitReader;
#[doc = "Field `RTA` reader - desc RTA"]
pub type RtaR = crate::BitReader;
#[doc = "Field `BWE` reader - desc BWE"]
pub type BweR = crate::BitReader;
#[doc = "Field `BRE` reader - desc BRE"]
pub type BreR = crate::BitReader;
#[doc = "Field `CIN` reader - desc CIN"]
pub type CinR = crate::BitReader;
#[doc = "Field `CSS` reader - desc CSS"]
pub type CssR = crate::BitReader;
#[doc = "Field `CDL` reader - desc CDL"]
pub type CdlR = crate::BitReader;
#[doc = "Field `WPL` reader - desc WPL"]
pub type WplR = crate::BitReader;
#[doc = "Field `DATL` reader - desc DATL"]
pub type DatlR = crate::FieldReader;
#[doc = "Field `CMDL` reader - desc CMDL"]
pub type CmdlR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc CIC"]
    #[inline(always)]
    pub fn cic(&self) -> CicR {
        CicR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CID"]
    #[inline(always)]
    pub fn cid(&self) -> CidR {
        CidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc DA"]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - desc WTA"]
    #[inline(always)]
    pub fn wta(&self) -> WtaR {
        WtaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc RTA"]
    #[inline(always)]
    pub fn rta(&self) -> RtaR {
        RtaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BWE"]
    #[inline(always)]
    pub fn bwe(&self) -> BweR {
        BweR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc BRE"]
    #[inline(always)]
    pub fn bre(&self) -> BreR {
        BreR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - desc CIN"]
    #[inline(always)]
    pub fn cin(&self) -> CinR {
        CinR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc CSS"]
    #[inline(always)]
    pub fn css(&self) -> CssR {
        CssR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc CDL"]
    #[inline(always)]
    pub fn cdl(&self) -> CdlR {
        CdlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc WPL"]
    #[inline(always)]
    pub fn wpl(&self) -> WplR {
        WplR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - desc DATL"]
    #[inline(always)]
    pub fn datl(&self) -> DatlR {
        DatlR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - desc CMDL"]
    #[inline(always)]
    pub fn cmdl(&self) -> CmdlR {
        CmdlR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "desc PSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`pstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PstatSpec;
impl crate::RegisterSpec for PstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pstat::R`](R) reader structure"]
impl crate::Readable for PstatSpec {}
#[doc = "`reset()` method sets PSTAT to value 0"]
impl crate::Resettable for PstatSpec {}
