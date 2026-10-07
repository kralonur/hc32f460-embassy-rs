#[doc = "Register `GINTMSK` reader"]
pub type R = crate::R<GintmskSpec>;
#[doc = "Register `GINTMSK` writer"]
pub type W = crate::W<GintmskSpec>;
#[doc = "Field `MMISM` reader - desc MMISM"]
pub type MmismR = crate::BitReader;
#[doc = "Field `MMISM` writer - desc MMISM"]
pub type MmismW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFM` reader - desc SOFM"]
pub type SofmR = crate::BitReader;
#[doc = "Field `SOFM` writer - desc SOFM"]
pub type SofmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFNEM` reader - desc RXFNEM"]
pub type RxfnemR = crate::BitReader;
#[doc = "Field `RXFNEM` writer - desc RXFNEM"]
pub type RxfnemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPTXFEM` reader - desc NPTXFEM"]
pub type NptxfemR = crate::BitReader;
#[doc = "Field `NPTXFEM` writer - desc NPTXFEM"]
pub type NptxfemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINAKEFFM` reader - desc GINAKEFFM"]
pub type GinakeffmR = crate::BitReader;
#[doc = "Field `GINAKEFFM` writer - desc GINAKEFFM"]
pub type GinakeffmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GONAKEFFM` reader - desc GONAKEFFM"]
pub type GonakeffmR = crate::BitReader;
#[doc = "Field `GONAKEFFM` writer - desc GONAKEFFM"]
pub type GonakeffmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESUSPM` reader - desc ESUSPM"]
pub type EsuspmR = crate::BitReader;
#[doc = "Field `ESUSPM` writer - desc ESUSPM"]
pub type EsuspmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSPM` reader - desc USBSUSPM"]
pub type UsbsuspmR = crate::BitReader;
#[doc = "Field `USBSUSPM` writer - desc USBSUSPM"]
pub type UsbsuspmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRSTM` reader - desc USBRSTM"]
pub type UsbrstmR = crate::BitReader;
#[doc = "Field `USBRSTM` writer - desc USBRSTM"]
pub type UsbrstmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDNEM` reader - desc ENUMDNEM"]
pub type EnumdnemR = crate::BitReader;
#[doc = "Field `ENUMDNEM` writer - desc ENUMDNEM"]
pub type EnumdnemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOODRPM` reader - desc ISOODRPM"]
pub type IsoodrpmR = crate::BitReader;
#[doc = "Field `ISOODRPM` writer - desc ISOODRPM"]
pub type IsoodrpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPFM` reader - desc EOPFM"]
pub type EopfmR = crate::BitReader;
#[doc = "Field `EOPFM` writer - desc EOPFM"]
pub type EopfmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPIM` reader - desc IEPIM"]
pub type IepimR = crate::BitReader;
#[doc = "Field `IEPIM` writer - desc IEPIM"]
pub type IepimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEPIM` reader - desc OEPIM"]
pub type OepimR = crate::BitReader;
#[doc = "Field `OEPIM` writer - desc OEPIM"]
pub type OepimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IISOIXFRM` reader - desc IISOIXFRM"]
pub type IisoixfrmR = crate::BitReader;
#[doc = "Field `IISOIXFRM` writer - desc IISOIXFRM"]
pub type IisoixfrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPXFRM_INCOMPISOOUTM` reader - desc IPXFRM_INCOMPISOOUTM"]
pub type IpxfrmIncompisooutmR = crate::BitReader;
#[doc = "Field `IPXFRM_INCOMPISOOUTM` writer - desc IPXFRM_INCOMPISOOUTM"]
pub type IpxfrmIncompisooutmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAFSUSPM` reader - desc DATAFSUSPM"]
pub type DatafsuspmR = crate::BitReader;
#[doc = "Field `DATAFSUSPM` writer - desc DATAFSUSPM"]
pub type DatafsuspmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPRTIM` reader - desc HPRTIM"]
pub type HprtimR = crate::BitReader;
#[doc = "Field `HPRTIM` writer - desc HPRTIM"]
pub type HprtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCIM` reader - desc HCIM"]
pub type HcimR = crate::BitReader;
#[doc = "Field `HCIM` writer - desc HCIM"]
pub type HcimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFEM` reader - desc PTXFEM"]
pub type PtxfemR = crate::BitReader;
#[doc = "Field `PTXFEM` writer - desc PTXFEM"]
pub type PtxfemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIDSCHGM` reader - desc CIDSCHGM"]
pub type CidschgmR = crate::BitReader;
#[doc = "Field `CIDSCHGM` writer - desc CIDSCHGM"]
pub type CidschgmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCIM` reader - desc DISCIM"]
pub type DiscimR = crate::BitReader;
#[doc = "Field `DISCIM` writer - desc DISCIM"]
pub type DiscimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSVIM` reader - desc VBUSVIM"]
pub type VbusvimR = crate::BitReader;
#[doc = "Field `VBUSVIM` writer - desc VBUSVIM"]
pub type VbusvimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUIM` reader - desc WKUIM"]
pub type WkuimR = crate::BitReader;
#[doc = "Field `WKUIM` writer - desc WKUIM"]
pub type WkuimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - desc MMISM"]
    #[inline(always)]
    pub fn mmism(&self) -> MmismR {
        MmismR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SOFM"]
    #[inline(always)]
    pub fn sofm(&self) -> SofmR {
        SofmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RXFNEM"]
    #[inline(always)]
    pub fn rxfnem(&self) -> RxfnemR {
        RxfnemR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc NPTXFEM"]
    #[inline(always)]
    pub fn nptxfem(&self) -> NptxfemR {
        NptxfemR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc GINAKEFFM"]
    #[inline(always)]
    pub fn ginakeffm(&self) -> GinakeffmR {
        GinakeffmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc GONAKEFFM"]
    #[inline(always)]
    pub fn gonakeffm(&self) -> GonakeffmR {
        GonakeffmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ESUSPM"]
    #[inline(always)]
    pub fn esuspm(&self) -> EsuspmR {
        EsuspmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc USBSUSPM"]
    #[inline(always)]
    pub fn usbsuspm(&self) -> UsbsuspmR {
        UsbsuspmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc USBRSTM"]
    #[inline(always)]
    pub fn usbrstm(&self) -> UsbrstmR {
        UsbrstmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc ENUMDNEM"]
    #[inline(always)]
    pub fn enumdnem(&self) -> EnumdnemR {
        EnumdnemR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc ISOODRPM"]
    #[inline(always)]
    pub fn isoodrpm(&self) -> IsoodrpmR {
        IsoodrpmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EOPFM"]
    #[inline(always)]
    pub fn eopfm(&self) -> EopfmR {
        EopfmR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - desc IEPIM"]
    #[inline(always)]
    pub fn iepim(&self) -> IepimR {
        IepimR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc OEPIM"]
    #[inline(always)]
    pub fn oepim(&self) -> OepimR {
        OepimR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc IISOIXFRM"]
    #[inline(always)]
    pub fn iisoixfrm(&self) -> IisoixfrmR {
        IisoixfrmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc IPXFRM_INCOMPISOOUTM"]
    #[inline(always)]
    pub fn ipxfrm_incompisooutm(&self) -> IpxfrmIncompisooutmR {
        IpxfrmIncompisooutmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc DATAFSUSPM"]
    #[inline(always)]
    pub fn datafsuspm(&self) -> DatafsuspmR {
        DatafsuspmR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - desc HPRTIM"]
    #[inline(always)]
    pub fn hprtim(&self) -> HprtimR {
        HprtimR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc HCIM"]
    #[inline(always)]
    pub fn hcim(&self) -> HcimR {
        HcimR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc PTXFEM"]
    #[inline(always)]
    pub fn ptxfem(&self) -> PtxfemR {
        PtxfemR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - desc CIDSCHGM"]
    #[inline(always)]
    pub fn cidschgm(&self) -> CidschgmR {
        CidschgmR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc DISCIM"]
    #[inline(always)]
    pub fn discim(&self) -> DiscimR {
        DiscimR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc VBUSVIM"]
    #[inline(always)]
    pub fn vbusvim(&self) -> VbusvimR {
        VbusvimR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc WKUIM"]
    #[inline(always)]
    pub fn wkuim(&self) -> WkuimR {
        WkuimR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc MMISM"]
    #[inline(always)]
    pub fn mmism(&mut self) -> MmismW<'_, GintmskSpec> {
        MmismW::new(self, 1)
    }
    #[doc = "Bit 3 - desc SOFM"]
    #[inline(always)]
    pub fn sofm(&mut self) -> SofmW<'_, GintmskSpec> {
        SofmW::new(self, 3)
    }
    #[doc = "Bit 4 - desc RXFNEM"]
    #[inline(always)]
    pub fn rxfnem(&mut self) -> RxfnemW<'_, GintmskSpec> {
        RxfnemW::new(self, 4)
    }
    #[doc = "Bit 5 - desc NPTXFEM"]
    #[inline(always)]
    pub fn nptxfem(&mut self) -> NptxfemW<'_, GintmskSpec> {
        NptxfemW::new(self, 5)
    }
    #[doc = "Bit 6 - desc GINAKEFFM"]
    #[inline(always)]
    pub fn ginakeffm(&mut self) -> GinakeffmW<'_, GintmskSpec> {
        GinakeffmW::new(self, 6)
    }
    #[doc = "Bit 7 - desc GONAKEFFM"]
    #[inline(always)]
    pub fn gonakeffm(&mut self) -> GonakeffmW<'_, GintmskSpec> {
        GonakeffmW::new(self, 7)
    }
    #[doc = "Bit 10 - desc ESUSPM"]
    #[inline(always)]
    pub fn esuspm(&mut self) -> EsuspmW<'_, GintmskSpec> {
        EsuspmW::new(self, 10)
    }
    #[doc = "Bit 11 - desc USBSUSPM"]
    #[inline(always)]
    pub fn usbsuspm(&mut self) -> UsbsuspmW<'_, GintmskSpec> {
        UsbsuspmW::new(self, 11)
    }
    #[doc = "Bit 12 - desc USBRSTM"]
    #[inline(always)]
    pub fn usbrstm(&mut self) -> UsbrstmW<'_, GintmskSpec> {
        UsbrstmW::new(self, 12)
    }
    #[doc = "Bit 13 - desc ENUMDNEM"]
    #[inline(always)]
    pub fn enumdnem(&mut self) -> EnumdnemW<'_, GintmskSpec> {
        EnumdnemW::new(self, 13)
    }
    #[doc = "Bit 14 - desc ISOODRPM"]
    #[inline(always)]
    pub fn isoodrpm(&mut self) -> IsoodrpmW<'_, GintmskSpec> {
        IsoodrpmW::new(self, 14)
    }
    #[doc = "Bit 15 - desc EOPFM"]
    #[inline(always)]
    pub fn eopfm(&mut self) -> EopfmW<'_, GintmskSpec> {
        EopfmW::new(self, 15)
    }
    #[doc = "Bit 18 - desc IEPIM"]
    #[inline(always)]
    pub fn iepim(&mut self) -> IepimW<'_, GintmskSpec> {
        IepimW::new(self, 18)
    }
    #[doc = "Bit 19 - desc OEPIM"]
    #[inline(always)]
    pub fn oepim(&mut self) -> OepimW<'_, GintmskSpec> {
        OepimW::new(self, 19)
    }
    #[doc = "Bit 20 - desc IISOIXFRM"]
    #[inline(always)]
    pub fn iisoixfrm(&mut self) -> IisoixfrmW<'_, GintmskSpec> {
        IisoixfrmW::new(self, 20)
    }
    #[doc = "Bit 21 - desc IPXFRM_INCOMPISOOUTM"]
    #[inline(always)]
    pub fn ipxfrm_incompisooutm(&mut self) -> IpxfrmIncompisooutmW<'_, GintmskSpec> {
        IpxfrmIncompisooutmW::new(self, 21)
    }
    #[doc = "Bit 22 - desc DATAFSUSPM"]
    #[inline(always)]
    pub fn datafsuspm(&mut self) -> DatafsuspmW<'_, GintmskSpec> {
        DatafsuspmW::new(self, 22)
    }
    #[doc = "Bit 24 - desc HPRTIM"]
    #[inline(always)]
    pub fn hprtim(&mut self) -> HprtimW<'_, GintmskSpec> {
        HprtimW::new(self, 24)
    }
    #[doc = "Bit 25 - desc HCIM"]
    #[inline(always)]
    pub fn hcim(&mut self) -> HcimW<'_, GintmskSpec> {
        HcimW::new(self, 25)
    }
    #[doc = "Bit 26 - desc PTXFEM"]
    #[inline(always)]
    pub fn ptxfem(&mut self) -> PtxfemW<'_, GintmskSpec> {
        PtxfemW::new(self, 26)
    }
    #[doc = "Bit 28 - desc CIDSCHGM"]
    #[inline(always)]
    pub fn cidschgm(&mut self) -> CidschgmW<'_, GintmskSpec> {
        CidschgmW::new(self, 28)
    }
    #[doc = "Bit 29 - desc DISCIM"]
    #[inline(always)]
    pub fn discim(&mut self) -> DiscimW<'_, GintmskSpec> {
        DiscimW::new(self, 29)
    }
    #[doc = "Bit 30 - desc VBUSVIM"]
    #[inline(always)]
    pub fn vbusvim(&mut self) -> VbusvimW<'_, GintmskSpec> {
        VbusvimW::new(self, 30)
    }
    #[doc = "Bit 31 - desc WKUIM"]
    #[inline(always)]
    pub fn wkuim(&mut self) -> WkuimW<'_, GintmskSpec> {
        WkuimW::new(self, 31)
    }
}
#[doc = "desc GINTMSK\n\nYou can [`read`](crate::Reg::read) this register and get [`gintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GintmskSpec;
impl crate::RegisterSpec for GintmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintmsk::R`](R) reader structure"]
impl crate::Readable for GintmskSpec {}
#[doc = "`write(|w| ..)` method takes [`gintmsk::W`](W) writer structure"]
impl crate::Writable for GintmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GINTMSK to value 0"]
impl crate::Resettable for GintmskSpec {}
