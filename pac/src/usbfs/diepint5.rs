#[doc = "Register `DIEPINT5` reader"]
pub type R = crate::R<Diepint5Spec>;
#[doc = "Register `DIEPINT5` writer"]
pub type W = crate::W<Diepint5Spec>;
#[doc = "Field `XFRC` reader - desc XFRC"]
pub type XfrcR = crate::BitReader;
#[doc = "Field `XFRC` writer - desc XFRC"]
pub type XfrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISD` reader - desc EPDISD"]
pub type EpdisdR = crate::BitReader;
#[doc = "Field `EPDISD` writer - desc EPDISD"]
pub type EpdisdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOC` reader - desc TOC"]
pub type TocR = crate::BitReader;
#[doc = "Field `TOC` writer - desc TOC"]
pub type TocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTXFE` reader - desc TTXFE"]
pub type TtxfeR = crate::BitReader;
#[doc = "Field `TTXFE` writer - desc TTXFE"]
pub type TtxfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNE` reader - desc INEPNE"]
pub type InepneR = crate::BitReader;
#[doc = "Field `INEPNE` writer - desc INEPNE"]
pub type InepneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFE` reader - desc TXFE"]
pub type TxfeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XfrcR {
        XfrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EPDISD"]
    #[inline(always)]
    pub fn epdisd(&self) -> EpdisdR {
        EpdisdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TOC"]
    #[inline(always)]
    pub fn toc(&self) -> TocR {
        TocR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TTXFE"]
    #[inline(always)]
    pub fn ttxfe(&self) -> TtxfeR {
        TtxfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc INEPNE"]
    #[inline(always)]
    pub fn inepne(&self) -> InepneR {
        InepneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TXFE"]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc XFRC"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XfrcW<'_, Diepint5Spec> {
        XfrcW::new(self, 0)
    }
    #[doc = "Bit 1 - desc EPDISD"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EpdisdW<'_, Diepint5Spec> {
        EpdisdW::new(self, 1)
    }
    #[doc = "Bit 3 - desc TOC"]
    #[inline(always)]
    pub fn toc(&mut self) -> TocW<'_, Diepint5Spec> {
        TocW::new(self, 3)
    }
    #[doc = "Bit 4 - desc TTXFE"]
    #[inline(always)]
    pub fn ttxfe(&mut self) -> TtxfeW<'_, Diepint5Spec> {
        TtxfeW::new(self, 4)
    }
    #[doc = "Bit 6 - desc INEPNE"]
    #[inline(always)]
    pub fn inepne(&mut self) -> InepneW<'_, Diepint5Spec> {
        InepneW::new(self, 6)
    }
}
#[doc = "desc DIEPINT5\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diepint5Spec;
impl crate::RegisterSpec for Diepint5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint5::R`](R) reader structure"]
impl crate::Readable for Diepint5Spec {}
#[doc = "`write(|w| ..)` method takes [`diepint5::W`](W) writer structure"]
impl crate::Writable for Diepint5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPINT5 to value 0x80"]
impl crate::Resettable for Diepint5Spec {
    const RESET_VALUE: u32 = 0x80;
}
