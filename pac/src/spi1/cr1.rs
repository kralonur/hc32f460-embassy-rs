#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `SPIMDS` reader - desc SPIMDS"]
pub type SpimdsR = crate::BitReader;
#[doc = "Field `SPIMDS` writer - desc SPIMDS"]
pub type SpimdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMDS` reader - desc TXMDS"]
pub type TxmdsR = crate::BitReader;
#[doc = "Field `TXMDS` writer - desc TXMDS"]
pub type TxmdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTR` reader - desc MSTR"]
pub type MstrR = crate::BitReader;
#[doc = "Field `MSTR` writer - desc MSTR"]
pub type MstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLPBK` reader - desc SPLPBK"]
pub type SplpbkR = crate::BitReader;
#[doc = "Field `SPLPBK` writer - desc SPLPBK"]
pub type SplpbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLPBK2` reader - desc SPLPBK2"]
pub type Splpbk2R = crate::BitReader;
#[doc = "Field `SPLPBK2` writer - desc SPLPBK2"]
pub type Splpbk2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPE` reader - desc SPE"]
pub type SpeR = crate::BitReader;
#[doc = "Field `SPE` writer - desc SPE"]
pub type SpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSUSPE` reader - desc CSUSPE"]
pub type CsuspeR = crate::BitReader;
#[doc = "Field `CSUSPE` writer - desc CSUSPE"]
pub type CsuspeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIE` reader - desc EIE"]
pub type EieR = crate::BitReader;
#[doc = "Field `EIE` writer - desc EIE"]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIE` reader - desc TXIE"]
pub type TxieR = crate::BitReader;
#[doc = "Field `TXIE` writer - desc TXIE"]
pub type TxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIE` reader - desc RXIE"]
pub type RxieR = crate::BitReader;
#[doc = "Field `RXIE` writer - desc RXIE"]
pub type RxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDIE` reader - desc IDIE"]
pub type IdieR = crate::BitReader;
#[doc = "Field `IDIE` writer - desc IDIE"]
pub type IdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODFE` reader - desc MODFE"]
pub type ModfeR = crate::BitReader;
#[doc = "Field `MODFE` writer - desc MODFE"]
pub type ModfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATE` reader - desc PATE"]
pub type PateR = crate::BitReader;
#[doc = "Field `PATE` writer - desc PATE"]
pub type PateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAOE` reader - desc PAOE"]
pub type PaoeR = crate::BitReader;
#[doc = "Field `PAOE` writer - desc PAOE"]
pub type PaoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAE` reader - desc PAE"]
pub type PaeR = crate::BitReader;
#[doc = "Field `PAE` writer - desc PAE"]
pub type PaeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SPIMDS"]
    #[inline(always)]
    pub fn spimds(&self) -> SpimdsR {
        SpimdsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TXMDS"]
    #[inline(always)]
    pub fn txmds(&self) -> TxmdsR {
        TxmdsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc MSTR"]
    #[inline(always)]
    pub fn mstr(&self) -> MstrR {
        MstrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc SPLPBK"]
    #[inline(always)]
    pub fn splpbk(&self) -> SplpbkR {
        SplpbkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc SPLPBK2"]
    #[inline(always)]
    pub fn splpbk2(&self) -> Splpbk2R {
        Splpbk2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SPE"]
    #[inline(always)]
    pub fn spe(&self) -> SpeR {
        SpeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CSUSPE"]
    #[inline(always)]
    pub fn csuspe(&self) -> CsuspeR {
        CsuspeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc EIE"]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc TXIE"]
    #[inline(always)]
    pub fn txie(&self) -> TxieR {
        TxieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc RXIE"]
    #[inline(always)]
    pub fn rxie(&self) -> RxieR {
        RxieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc IDIE"]
    #[inline(always)]
    pub fn idie(&self) -> IdieR {
        IdieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc MODFE"]
    #[inline(always)]
    pub fn modfe(&self) -> ModfeR {
        ModfeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PATE"]
    #[inline(always)]
    pub fn pate(&self) -> PateR {
        PateR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PAOE"]
    #[inline(always)]
    pub fn paoe(&self) -> PaoeR {
        PaoeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PAE"]
    #[inline(always)]
    pub fn pae(&self) -> PaeR {
        PaeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SPIMDS"]
    #[inline(always)]
    pub fn spimds(&mut self) -> SpimdsW<'_, Cr1Spec> {
        SpimdsW::new(self, 0)
    }
    #[doc = "Bit 1 - desc TXMDS"]
    #[inline(always)]
    pub fn txmds(&mut self) -> TxmdsW<'_, Cr1Spec> {
        TxmdsW::new(self, 1)
    }
    #[doc = "Bit 3 - desc MSTR"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MstrW<'_, Cr1Spec> {
        MstrW::new(self, 3)
    }
    #[doc = "Bit 4 - desc SPLPBK"]
    #[inline(always)]
    pub fn splpbk(&mut self) -> SplpbkW<'_, Cr1Spec> {
        SplpbkW::new(self, 4)
    }
    #[doc = "Bit 5 - desc SPLPBK2"]
    #[inline(always)]
    pub fn splpbk2(&mut self) -> Splpbk2W<'_, Cr1Spec> {
        Splpbk2W::new(self, 5)
    }
    #[doc = "Bit 6 - desc SPE"]
    #[inline(always)]
    pub fn spe(&mut self) -> SpeW<'_, Cr1Spec> {
        SpeW::new(self, 6)
    }
    #[doc = "Bit 7 - desc CSUSPE"]
    #[inline(always)]
    pub fn csuspe(&mut self) -> CsuspeW<'_, Cr1Spec> {
        CsuspeW::new(self, 7)
    }
    #[doc = "Bit 8 - desc EIE"]
    #[inline(always)]
    pub fn eie(&mut self) -> EieW<'_, Cr1Spec> {
        EieW::new(self, 8)
    }
    #[doc = "Bit 9 - desc TXIE"]
    #[inline(always)]
    pub fn txie(&mut self) -> TxieW<'_, Cr1Spec> {
        TxieW::new(self, 9)
    }
    #[doc = "Bit 10 - desc RXIE"]
    #[inline(always)]
    pub fn rxie(&mut self) -> RxieW<'_, Cr1Spec> {
        RxieW::new(self, 10)
    }
    #[doc = "Bit 11 - desc IDIE"]
    #[inline(always)]
    pub fn idie(&mut self) -> IdieW<'_, Cr1Spec> {
        IdieW::new(self, 11)
    }
    #[doc = "Bit 12 - desc MODFE"]
    #[inline(always)]
    pub fn modfe(&mut self) -> ModfeW<'_, Cr1Spec> {
        ModfeW::new(self, 12)
    }
    #[doc = "Bit 13 - desc PATE"]
    #[inline(always)]
    pub fn pate(&mut self) -> PateW<'_, Cr1Spec> {
        PateW::new(self, 13)
    }
    #[doc = "Bit 14 - desc PAOE"]
    #[inline(always)]
    pub fn paoe(&mut self) -> PaoeW<'_, Cr1Spec> {
        PaoeW::new(self, 14)
    }
    #[doc = "Bit 15 - desc PAE"]
    #[inline(always)]
    pub fn pae(&mut self) -> PaeW<'_, Cr1Spec> {
        PaeW::new(self, 15)
    }
}
#[doc = "desc CR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
