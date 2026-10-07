#[doc = "Register `GAHBCFG` reader"]
pub type R = crate::R<GahbcfgSpec>;
#[doc = "Register `GAHBCFG` writer"]
pub type W = crate::W<GahbcfgSpec>;
#[doc = "Field `GINTMSK` reader - desc GINTMSK"]
pub type GintmskR = crate::BitReader;
#[doc = "Field `GINTMSK` writer - desc GINTMSK"]
pub type GintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBSTLEN` reader - desc HBSTLEN"]
pub type HbstlenR = crate::FieldReader;
#[doc = "Field `HBSTLEN` writer - desc HBSTLEN"]
pub type HbstlenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAEN` reader - desc DMAEN"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - desc DMAEN"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFELVL` reader - desc TXFELVL"]
pub type TxfelvlR = crate::BitReader;
#[doc = "Field `TXFELVL` writer - desc TXFELVL"]
pub type TxfelvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFELVL` reader - desc PTXFELVL"]
pub type PtxfelvlR = crate::BitReader;
#[doc = "Field `PTXFELVL` writer - desc PTXFELVL"]
pub type PtxfelvlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc GINTMSK"]
    #[inline(always)]
    pub fn gintmsk(&self) -> GintmskR {
        GintmskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - desc HBSTLEN"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HbstlenR {
        HbstlenR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - desc DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TXFELVL"]
    #[inline(always)]
    pub fn txfelvl(&self) -> TxfelvlR {
        TxfelvlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PTXFELVL"]
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PtxfelvlR {
        PtxfelvlR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc GINTMSK"]
    #[inline(always)]
    pub fn gintmsk(&mut self) -> GintmskW<'_, GahbcfgSpec> {
        GintmskW::new(self, 0)
    }
    #[doc = "Bits 1:4 - desc HBSTLEN"]
    #[inline(always)]
    pub fn hbstlen(&mut self) -> HbstlenW<'_, GahbcfgSpec> {
        HbstlenW::new(self, 1)
    }
    #[doc = "Bit 5 - desc DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, GahbcfgSpec> {
        DmaenW::new(self, 5)
    }
    #[doc = "Bit 7 - desc TXFELVL"]
    #[inline(always)]
    pub fn txfelvl(&mut self) -> TxfelvlW<'_, GahbcfgSpec> {
        TxfelvlW::new(self, 7)
    }
    #[doc = "Bit 8 - desc PTXFELVL"]
    #[inline(always)]
    pub fn ptxfelvl(&mut self) -> PtxfelvlW<'_, GahbcfgSpec> {
        PtxfelvlW::new(self, 8)
    }
}
#[doc = "desc GAHBCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GahbcfgSpec;
impl crate::RegisterSpec for GahbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcfg::R`](R) reader structure"]
impl crate::Readable for GahbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure"]
impl crate::Writable for GahbcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GahbcfgSpec {}
