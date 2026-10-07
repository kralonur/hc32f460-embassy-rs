#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - desc TXE"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIE` reader - desc TXIE"]
pub type TxieR = crate::BitReader;
#[doc = "Field `TXIE` writer - desc TXIE"]
pub type TxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXE` reader - desc RXE"]
pub type RxeR = crate::BitReader;
#[doc = "Field `RXE` writer - desc RXE"]
pub type RxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIE` reader - desc RXIE"]
pub type RxieR = crate::BitReader;
#[doc = "Field `RXIE` writer - desc RXIE"]
pub type RxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIE` reader - desc EIE"]
pub type EieR = crate::BitReader;
#[doc = "Field `EIE` writer - desc EIE"]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WMS` reader - desc WMS"]
pub type WmsR = crate::BitReader;
#[doc = "Field `WMS` writer - desc WMS"]
pub type WmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODD` reader - desc ODD"]
pub type OddR = crate::BitReader;
#[doc = "Field `ODD` writer - desc ODD"]
pub type OddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKOE` reader - desc MCKOE"]
pub type MckoeR = crate::BitReader;
#[doc = "Field `MCKOE` writer - desc MCKOE"]
pub type MckoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBIRQWL` reader - desc TXBIRQWL"]
pub type TxbirqwlR = crate::FieldReader;
#[doc = "Field `TXBIRQWL` writer - desc TXBIRQWL"]
pub type TxbirqwlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXBIRQWL` reader - desc RXBIRQWL"]
pub type RxbirqwlR = crate::FieldReader;
#[doc = "Field `RXBIRQWL` writer - desc RXBIRQWL"]
pub type RxbirqwlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FIFOR` reader - desc FIFOR"]
pub type FiforR = crate::BitReader;
#[doc = "Field `FIFOR` writer - desc FIFOR"]
pub type FiforW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODECRC` reader - desc CODECRC"]
pub type CodecrcR = crate::BitReader;
#[doc = "Field `CODECRC` writer - desc CODECRC"]
pub type CodecrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SPLLSEL` reader - desc I2SPLLSEL"]
pub type I2spllselR = crate::BitReader;
#[doc = "Field `I2SPLLSEL` writer - desc I2SPLLSEL"]
pub type I2spllselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDOE` reader - desc SDOE"]
pub type SdoeR = crate::BitReader;
#[doc = "Field `SDOE` writer - desc SDOE"]
pub type SdoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LRCKOE` reader - desc LRCKOE"]
pub type LrckoeR = crate::BitReader;
#[doc = "Field `LRCKOE` writer - desc LRCKOE"]
pub type LrckoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKOE` reader - desc CKOE"]
pub type CkoeR = crate::BitReader;
#[doc = "Field `CKOE` writer - desc CKOE"]
pub type CkoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUPLEX` reader - desc DUPLEX"]
pub type DuplexR = crate::BitReader;
#[doc = "Field `DUPLEX` writer - desc DUPLEX"]
pub type DuplexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSEL` reader - desc CLKSEL"]
pub type ClkselR = crate::BitReader;
#[doc = "Field `CLKSEL` writer - desc CLKSEL"]
pub type ClkselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TXIE"]
    #[inline(always)]
    pub fn txie(&self) -> TxieR {
        TxieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc RXE"]
    #[inline(always)]
    pub fn rxe(&self) -> RxeR {
        RxeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc RXIE"]
    #[inline(always)]
    pub fn rxie(&self) -> RxieR {
        RxieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc EIE"]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc WMS"]
    #[inline(always)]
    pub fn wms(&self) -> WmsR {
        WmsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ODD"]
    #[inline(always)]
    pub fn odd(&self) -> OddR {
        OddR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc MCKOE"]
    #[inline(always)]
    pub fn mckoe(&self) -> MckoeR {
        MckoeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - desc TXBIRQWL"]
    #[inline(always)]
    pub fn txbirqwl(&self) -> TxbirqwlR {
        TxbirqwlR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc RXBIRQWL"]
    #[inline(always)]
    pub fn rxbirqwl(&self) -> RxbirqwlR {
        RxbirqwlR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - desc FIFOR"]
    #[inline(always)]
    pub fn fifor(&self) -> FiforR {
        FiforR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc CODECRC"]
    #[inline(always)]
    pub fn codecrc(&self) -> CodecrcR {
        CodecrcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc I2SPLLSEL"]
    #[inline(always)]
    pub fn i2spllsel(&self) -> I2spllselR {
        I2spllselR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc SDOE"]
    #[inline(always)]
    pub fn sdoe(&self) -> SdoeR {
        SdoeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc LRCKOE"]
    #[inline(always)]
    pub fn lrckoe(&self) -> LrckoeR {
        LrckoeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc CKOE"]
    #[inline(always)]
    pub fn ckoe(&self) -> CkoeR {
        CkoeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc DUPLEX"]
    #[inline(always)]
    pub fn duplex(&self) -> DuplexR {
        DuplexR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc CLKSEL"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TXE"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, CtrlSpec> {
        TxeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc TXIE"]
    #[inline(always)]
    pub fn txie(&mut self) -> TxieW<'_, CtrlSpec> {
        TxieW::new(self, 1)
    }
    #[doc = "Bit 2 - desc RXE"]
    #[inline(always)]
    pub fn rxe(&mut self) -> RxeW<'_, CtrlSpec> {
        RxeW::new(self, 2)
    }
    #[doc = "Bit 3 - desc RXIE"]
    #[inline(always)]
    pub fn rxie(&mut self) -> RxieW<'_, CtrlSpec> {
        RxieW::new(self, 3)
    }
    #[doc = "Bit 4 - desc EIE"]
    #[inline(always)]
    pub fn eie(&mut self) -> EieW<'_, CtrlSpec> {
        EieW::new(self, 4)
    }
    #[doc = "Bit 5 - desc WMS"]
    #[inline(always)]
    pub fn wms(&mut self) -> WmsW<'_, CtrlSpec> {
        WmsW::new(self, 5)
    }
    #[doc = "Bit 6 - desc ODD"]
    #[inline(always)]
    pub fn odd(&mut self) -> OddW<'_, CtrlSpec> {
        OddW::new(self, 6)
    }
    #[doc = "Bit 7 - desc MCKOE"]
    #[inline(always)]
    pub fn mckoe(&mut self) -> MckoeW<'_, CtrlSpec> {
        MckoeW::new(self, 7)
    }
    #[doc = "Bits 8:10 - desc TXBIRQWL"]
    #[inline(always)]
    pub fn txbirqwl(&mut self) -> TxbirqwlW<'_, CtrlSpec> {
        TxbirqwlW::new(self, 8)
    }
    #[doc = "Bits 12:14 - desc RXBIRQWL"]
    #[inline(always)]
    pub fn rxbirqwl(&mut self) -> RxbirqwlW<'_, CtrlSpec> {
        RxbirqwlW::new(self, 12)
    }
    #[doc = "Bit 16 - desc FIFOR"]
    #[inline(always)]
    pub fn fifor(&mut self) -> FiforW<'_, CtrlSpec> {
        FiforW::new(self, 16)
    }
    #[doc = "Bit 17 - desc CODECRC"]
    #[inline(always)]
    pub fn codecrc(&mut self) -> CodecrcW<'_, CtrlSpec> {
        CodecrcW::new(self, 17)
    }
    #[doc = "Bit 18 - desc I2SPLLSEL"]
    #[inline(always)]
    pub fn i2spllsel(&mut self) -> I2spllselW<'_, CtrlSpec> {
        I2spllselW::new(self, 18)
    }
    #[doc = "Bit 19 - desc SDOE"]
    #[inline(always)]
    pub fn sdoe(&mut self) -> SdoeW<'_, CtrlSpec> {
        SdoeW::new(self, 19)
    }
    #[doc = "Bit 20 - desc LRCKOE"]
    #[inline(always)]
    pub fn lrckoe(&mut self) -> LrckoeW<'_, CtrlSpec> {
        LrckoeW::new(self, 20)
    }
    #[doc = "Bit 21 - desc CKOE"]
    #[inline(always)]
    pub fn ckoe(&mut self) -> CkoeW<'_, CtrlSpec> {
        CkoeW::new(self, 21)
    }
    #[doc = "Bit 22 - desc DUPLEX"]
    #[inline(always)]
    pub fn duplex(&mut self) -> DuplexW<'_, CtrlSpec> {
        DuplexW::new(self, 22)
    }
    #[doc = "Bit 23 - desc CLKSEL"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<'_, CtrlSpec> {
        ClkselW::new(self, 23)
    }
}
#[doc = "desc CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x2200"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x2200;
}
