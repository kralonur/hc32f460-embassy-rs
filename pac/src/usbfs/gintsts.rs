#[doc = "Register `GINTSTS` reader"]
pub type R = crate::R<GintstsSpec>;
#[doc = "Register `GINTSTS` writer"]
pub type W = crate::W<GintstsSpec>;
#[doc = "Field `CMOD` reader - desc CMOD"]
pub type CmodR = crate::BitReader;
#[doc = "Field `MMIS` reader - desc MMIS"]
pub type MmisR = crate::BitReader;
#[doc = "Field `MMIS` writer - desc MMIS"]
pub type MmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF` reader - desc SOF"]
pub type SofR = crate::BitReader;
#[doc = "Field `SOF` writer - desc SOF"]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFNE` reader - desc RXFNE"]
pub type RxfneR = crate::BitReader;
#[doc = "Field `NPTXFE` reader - desc NPTXFE"]
pub type NptxfeR = crate::BitReader;
#[doc = "Field `GINAKEFF` reader - desc GINAKEFF"]
pub type GinakeffR = crate::BitReader;
#[doc = "Field `GONAKEFF` reader - desc GONAKEFF"]
pub type GonakeffR = crate::BitReader;
#[doc = "Field `ESUSP` reader - desc ESUSP"]
pub type EsuspR = crate::BitReader;
#[doc = "Field `ESUSP` writer - desc ESUSP"]
pub type EsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSP` reader - desc USBSUSP"]
pub type UsbsuspR = crate::BitReader;
#[doc = "Field `USBSUSP` writer - desc USBSUSP"]
pub type UsbsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - desc USBRST"]
pub type UsbrstR = crate::BitReader;
#[doc = "Field `USBRST` writer - desc USBRST"]
pub type UsbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDNE` reader - desc ENUMDNE"]
pub type EnumdneR = crate::BitReader;
#[doc = "Field `ENUMDNE` writer - desc ENUMDNE"]
pub type EnumdneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOODRP` reader - desc ISOODRP"]
pub type IsoodrpR = crate::BitReader;
#[doc = "Field `ISOODRP` writer - desc ISOODRP"]
pub type IsoodrpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPF` reader - desc EOPF"]
pub type EopfR = crate::BitReader;
#[doc = "Field `EOPF` writer - desc EOPF"]
pub type EopfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPINT` reader - desc IEPINT"]
pub type IepintR = crate::BitReader;
#[doc = "Field `OEPINT` reader - desc OEPINT"]
pub type OepintR = crate::BitReader;
#[doc = "Field `IISOIXFR` reader - desc IISOIXFR"]
pub type IisoixfrR = crate::BitReader;
#[doc = "Field `IISOIXFR` writer - desc IISOIXFR"]
pub type IisoixfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPXFR_INCOMPISOOUT` reader - desc IPXFR_INCOMPISOOUT"]
pub type IpxfrIncompisooutR = crate::BitReader;
#[doc = "Field `IPXFR_INCOMPISOOUT` writer - desc IPXFR_INCOMPISOOUT"]
pub type IpxfrIncompisooutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAFSUSP` reader - desc DATAFSUSP"]
pub type DatafsuspR = crate::BitReader;
#[doc = "Field `DATAFSUSP` writer - desc DATAFSUSP"]
pub type DatafsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPRTINT` reader - desc HPRTINT"]
pub type HprtintR = crate::BitReader;
#[doc = "Field `HCINT` reader - desc HCINT"]
pub type HcintR = crate::BitReader;
#[doc = "Field `PTXFE` reader - desc PTXFE"]
pub type PtxfeR = crate::BitReader;
#[doc = "Field `CIDSCHG` reader - desc CIDSCHG"]
pub type CidschgR = crate::BitReader;
#[doc = "Field `CIDSCHG` writer - desc CIDSCHG"]
pub type CidschgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCINT` reader - desc DISCINT"]
pub type DiscintR = crate::BitReader;
#[doc = "Field `DISCINT` writer - desc DISCINT"]
pub type DiscintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSVINT` reader - desc VBUSVINT"]
pub type VbusvintR = crate::BitReader;
#[doc = "Field `VBUSVINT` writer - desc VBUSVINT"]
pub type VbusvintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUINT` reader - desc WKUINT"]
pub type WkuintR = crate::BitReader;
#[doc = "Field `WKUINT` writer - desc WKUINT"]
pub type WkuintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CMOD"]
    #[inline(always)]
    pub fn cmod(&self) -> CmodR {
        CmodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc MMIS"]
    #[inline(always)]
    pub fn mmis(&self) -> MmisR {
        MmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RXFNE"]
    #[inline(always)]
    pub fn rxfne(&self) -> RxfneR {
        RxfneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc NPTXFE"]
    #[inline(always)]
    pub fn nptxfe(&self) -> NptxfeR {
        NptxfeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc GINAKEFF"]
    #[inline(always)]
    pub fn ginakeff(&self) -> GinakeffR {
        GinakeffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc GONAKEFF"]
    #[inline(always)]
    pub fn gonakeff(&self) -> GonakeffR {
        GonakeffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ESUSP"]
    #[inline(always)]
    pub fn esusp(&self) -> EsuspR {
        EsuspR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc USBSUSP"]
    #[inline(always)]
    pub fn usbsusp(&self) -> UsbsuspR {
        UsbsuspR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc USBRST"]
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        UsbrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc ENUMDNE"]
    #[inline(always)]
    pub fn enumdne(&self) -> EnumdneR {
        EnumdneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc ISOODRP"]
    #[inline(always)]
    pub fn isoodrp(&self) -> IsoodrpR {
        IsoodrpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EOPF"]
    #[inline(always)]
    pub fn eopf(&self) -> EopfR {
        EopfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - desc IEPINT"]
    #[inline(always)]
    pub fn iepint(&self) -> IepintR {
        IepintR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc OEPINT"]
    #[inline(always)]
    pub fn oepint(&self) -> OepintR {
        OepintR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc IISOIXFR"]
    #[inline(always)]
    pub fn iisoixfr(&self) -> IisoixfrR {
        IisoixfrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc IPXFR_INCOMPISOOUT"]
    #[inline(always)]
    pub fn ipxfr_incompisoout(&self) -> IpxfrIncompisooutR {
        IpxfrIncompisooutR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc DATAFSUSP"]
    #[inline(always)]
    pub fn datafsusp(&self) -> DatafsuspR {
        DatafsuspR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - desc HPRTINT"]
    #[inline(always)]
    pub fn hprtint(&self) -> HprtintR {
        HprtintR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc HCINT"]
    #[inline(always)]
    pub fn hcint(&self) -> HcintR {
        HcintR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc PTXFE"]
    #[inline(always)]
    pub fn ptxfe(&self) -> PtxfeR {
        PtxfeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - desc CIDSCHG"]
    #[inline(always)]
    pub fn cidschg(&self) -> CidschgR {
        CidschgR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc DISCINT"]
    #[inline(always)]
    pub fn discint(&self) -> DiscintR {
        DiscintR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc VBUSVINT"]
    #[inline(always)]
    pub fn vbusvint(&self) -> VbusvintR {
        VbusvintR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc WKUINT"]
    #[inline(always)]
    pub fn wkuint(&self) -> WkuintR {
        WkuintR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc MMIS"]
    #[inline(always)]
    pub fn mmis(&mut self) -> MmisW<'_, GintstsSpec> {
        MmisW::new(self, 1)
    }
    #[doc = "Bit 3 - desc SOF"]
    #[inline(always)]
    pub fn sof(&mut self) -> SofW<'_, GintstsSpec> {
        SofW::new(self, 3)
    }
    #[doc = "Bit 10 - desc ESUSP"]
    #[inline(always)]
    pub fn esusp(&mut self) -> EsuspW<'_, GintstsSpec> {
        EsuspW::new(self, 10)
    }
    #[doc = "Bit 11 - desc USBSUSP"]
    #[inline(always)]
    pub fn usbsusp(&mut self) -> UsbsuspW<'_, GintstsSpec> {
        UsbsuspW::new(self, 11)
    }
    #[doc = "Bit 12 - desc USBRST"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> UsbrstW<'_, GintstsSpec> {
        UsbrstW::new(self, 12)
    }
    #[doc = "Bit 13 - desc ENUMDNE"]
    #[inline(always)]
    pub fn enumdne(&mut self) -> EnumdneW<'_, GintstsSpec> {
        EnumdneW::new(self, 13)
    }
    #[doc = "Bit 14 - desc ISOODRP"]
    #[inline(always)]
    pub fn isoodrp(&mut self) -> IsoodrpW<'_, GintstsSpec> {
        IsoodrpW::new(self, 14)
    }
    #[doc = "Bit 15 - desc EOPF"]
    #[inline(always)]
    pub fn eopf(&mut self) -> EopfW<'_, GintstsSpec> {
        EopfW::new(self, 15)
    }
    #[doc = "Bit 20 - desc IISOIXFR"]
    #[inline(always)]
    pub fn iisoixfr(&mut self) -> IisoixfrW<'_, GintstsSpec> {
        IisoixfrW::new(self, 20)
    }
    #[doc = "Bit 21 - desc IPXFR_INCOMPISOOUT"]
    #[inline(always)]
    pub fn ipxfr_incompisoout(&mut self) -> IpxfrIncompisooutW<'_, GintstsSpec> {
        IpxfrIncompisooutW::new(self, 21)
    }
    #[doc = "Bit 22 - desc DATAFSUSP"]
    #[inline(always)]
    pub fn datafsusp(&mut self) -> DatafsuspW<'_, GintstsSpec> {
        DatafsuspW::new(self, 22)
    }
    #[doc = "Bit 28 - desc CIDSCHG"]
    #[inline(always)]
    pub fn cidschg(&mut self) -> CidschgW<'_, GintstsSpec> {
        CidschgW::new(self, 28)
    }
    #[doc = "Bit 29 - desc DISCINT"]
    #[inline(always)]
    pub fn discint(&mut self) -> DiscintW<'_, GintstsSpec> {
        DiscintW::new(self, 29)
    }
    #[doc = "Bit 30 - desc VBUSVINT"]
    #[inline(always)]
    pub fn vbusvint(&mut self) -> VbusvintW<'_, GintstsSpec> {
        VbusvintW::new(self, 30)
    }
    #[doc = "Bit 31 - desc WKUINT"]
    #[inline(always)]
    pub fn wkuint(&mut self) -> WkuintW<'_, GintstsSpec> {
        WkuintW::new(self, 31)
    }
}
#[doc = "desc GINTSTS\n\nYou can [`read`](crate::Reg::read) this register and get [`gintsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GintstsSpec;
impl crate::RegisterSpec for GintstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintsts::R`](R) reader structure"]
impl crate::Readable for GintstsSpec {}
#[doc = "`write(|w| ..)` method takes [`gintsts::W`](W) writer structure"]
impl crate::Writable for GintstsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GINTSTS to value 0x1400_0020"]
impl crate::Resettable for GintstsSpec {
    const RESET_VALUE: u32 = 0x1400_0020;
}
