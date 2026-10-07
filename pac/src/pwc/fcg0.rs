#[doc = "Register `FCG0` reader"]
pub type R = crate::R<Fcg0Spec>;
#[doc = "Register `FCG0` writer"]
pub type W = crate::W<Fcg0Spec>;
#[doc = "Field `SRAMH` reader - desc SRAMH"]
pub type SramhR = crate::BitReader;
#[doc = "Field `SRAMH` writer - desc SRAMH"]
pub type SramhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM12` reader - desc SRAM12"]
pub type Sram12R = crate::BitReader;
#[doc = "Field `SRAM12` writer - desc SRAM12"]
pub type Sram12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3` reader - desc SRAM3"]
pub type Sram3R = crate::BitReader;
#[doc = "Field `SRAM3` writer - desc SRAM3"]
pub type Sram3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMRET` reader - desc SRAMRET"]
pub type SramretR = crate::BitReader;
#[doc = "Field `SRAMRET` writer - desc SRAMRET"]
pub type SramretW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1` reader - desc DMA1"]
pub type Dma1R = crate::BitReader;
#[doc = "Field `DMA1` writer - desc DMA1"]
pub type Dma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2` reader - desc DMA2"]
pub type Dma2R = crate::BitReader;
#[doc = "Field `DMA2` writer - desc DMA2"]
pub type Dma2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCM` reader - desc FCM"]
pub type FcmR = crate::BitReader;
#[doc = "Field `FCM` writer - desc FCM"]
pub type FcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AOS` reader - desc AOS"]
pub type AosR = crate::BitReader;
#[doc = "Field `AOS` writer - desc AOS"]
pub type AosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES` reader - desc AES"]
pub type AesR = crate::BitReader;
#[doc = "Field `AES` writer - desc AES"]
pub type AesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH` reader - desc HASH"]
pub type HashR = crate::BitReader;
#[doc = "Field `HASH` writer - desc HASH"]
pub type HashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG` reader - desc TRNG"]
pub type TrngR = crate::BitReader;
#[doc = "Field `TRNG` writer - desc TRNG"]
pub type TrngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - desc CRC"]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - desc CRC"]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCU1` reader - desc DCU1"]
pub type Dcu1R = crate::BitReader;
#[doc = "Field `DCU1` writer - desc DCU1"]
pub type Dcu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCU2` reader - desc DCU2"]
pub type Dcu2R = crate::BitReader;
#[doc = "Field `DCU2` writer - desc DCU2"]
pub type Dcu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCU3` reader - desc DCU3"]
pub type Dcu3R = crate::BitReader;
#[doc = "Field `DCU3` writer - desc DCU3"]
pub type Dcu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCU4` reader - desc DCU4"]
pub type Dcu4R = crate::BitReader;
#[doc = "Field `DCU4` writer - desc DCU4"]
pub type Dcu4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` reader - desc KEY"]
pub type KeyR = crate::BitReader;
#[doc = "Field `KEY` writer - desc KEY"]
pub type KeyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SRAMH"]
    #[inline(always)]
    pub fn sramh(&self) -> SramhR {
        SramhR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - desc SRAM12"]
    #[inline(always)]
    pub fn sram12(&self) -> Sram12R {
        Sram12R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - desc SRAM3"]
    #[inline(always)]
    pub fn sram3(&self) -> Sram3R {
        Sram3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - desc SRAMRET"]
    #[inline(always)]
    pub fn sramret(&self) -> SramretR {
        SramretR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - desc DMA1"]
    #[inline(always)]
    pub fn dma1(&self) -> Dma1R {
        Dma1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc DMA2"]
    #[inline(always)]
    pub fn dma2(&self) -> Dma2R {
        Dma2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc FCM"]
    #[inline(always)]
    pub fn fcm(&self) -> FcmR {
        FcmR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc AOS"]
    #[inline(always)]
    pub fn aos(&self) -> AosR {
        AosR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - desc AES"]
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc HASH"]
    #[inline(always)]
    pub fn hash(&self) -> HashR {
        HashR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc TRNG"]
    #[inline(always)]
    pub fn trng(&self) -> TrngR {
        TrngR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc CRC"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc DCU1"]
    #[inline(always)]
    pub fn dcu1(&self) -> Dcu1R {
        Dcu1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc DCU2"]
    #[inline(always)]
    pub fn dcu2(&self) -> Dcu2R {
        Dcu2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc DCU3"]
    #[inline(always)]
    pub fn dcu3(&self) -> Dcu3R {
        Dcu3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc DCU4"]
    #[inline(always)]
    pub fn dcu4(&self) -> Dcu4R {
        Dcu4R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - desc KEY"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SRAMH"]
    #[inline(always)]
    pub fn sramh(&mut self) -> SramhW<'_, Fcg0Spec> {
        SramhW::new(self, 0)
    }
    #[doc = "Bit 4 - desc SRAM12"]
    #[inline(always)]
    pub fn sram12(&mut self) -> Sram12W<'_, Fcg0Spec> {
        Sram12W::new(self, 4)
    }
    #[doc = "Bit 8 - desc SRAM3"]
    #[inline(always)]
    pub fn sram3(&mut self) -> Sram3W<'_, Fcg0Spec> {
        Sram3W::new(self, 8)
    }
    #[doc = "Bit 10 - desc SRAMRET"]
    #[inline(always)]
    pub fn sramret(&mut self) -> SramretW<'_, Fcg0Spec> {
        SramretW::new(self, 10)
    }
    #[doc = "Bit 14 - desc DMA1"]
    #[inline(always)]
    pub fn dma1(&mut self) -> Dma1W<'_, Fcg0Spec> {
        Dma1W::new(self, 14)
    }
    #[doc = "Bit 15 - desc DMA2"]
    #[inline(always)]
    pub fn dma2(&mut self) -> Dma2W<'_, Fcg0Spec> {
        Dma2W::new(self, 15)
    }
    #[doc = "Bit 16 - desc FCM"]
    #[inline(always)]
    pub fn fcm(&mut self) -> FcmW<'_, Fcg0Spec> {
        FcmW::new(self, 16)
    }
    #[doc = "Bit 17 - desc AOS"]
    #[inline(always)]
    pub fn aos(&mut self) -> AosW<'_, Fcg0Spec> {
        AosW::new(self, 17)
    }
    #[doc = "Bit 20 - desc AES"]
    #[inline(always)]
    pub fn aes(&mut self) -> AesW<'_, Fcg0Spec> {
        AesW::new(self, 20)
    }
    #[doc = "Bit 21 - desc HASH"]
    #[inline(always)]
    pub fn hash(&mut self) -> HashW<'_, Fcg0Spec> {
        HashW::new(self, 21)
    }
    #[doc = "Bit 22 - desc TRNG"]
    #[inline(always)]
    pub fn trng(&mut self) -> TrngW<'_, Fcg0Spec> {
        TrngW::new(self, 22)
    }
    #[doc = "Bit 23 - desc CRC"]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<'_, Fcg0Spec> {
        CrcW::new(self, 23)
    }
    #[doc = "Bit 24 - desc DCU1"]
    #[inline(always)]
    pub fn dcu1(&mut self) -> Dcu1W<'_, Fcg0Spec> {
        Dcu1W::new(self, 24)
    }
    #[doc = "Bit 25 - desc DCU2"]
    #[inline(always)]
    pub fn dcu2(&mut self) -> Dcu2W<'_, Fcg0Spec> {
        Dcu2W::new(self, 25)
    }
    #[doc = "Bit 26 - desc DCU3"]
    #[inline(always)]
    pub fn dcu3(&mut self) -> Dcu3W<'_, Fcg0Spec> {
        Dcu3W::new(self, 26)
    }
    #[doc = "Bit 27 - desc DCU4"]
    #[inline(always)]
    pub fn dcu4(&mut self) -> Dcu4W<'_, Fcg0Spec> {
        Dcu4W::new(self, 27)
    }
    #[doc = "Bit 31 - desc KEY"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, Fcg0Spec> {
        KeyW::new(self, 31)
    }
}
#[doc = "desc FCG0\n\nYou can [`read`](crate::Reg::read) this register and get [`fcg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fcg0Spec;
impl crate::RegisterSpec for Fcg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcg0::R`](R) reader structure"]
impl crate::Readable for Fcg0Spec {}
#[doc = "`write(|w| ..)` method takes [`fcg0::W`](W) writer structure"]
impl crate::Writable for Fcg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCG0 to value 0xffff_faee"]
impl crate::Resettable for Fcg0Spec {
    const RESET_VALUE: u32 = 0xffff_faee;
}
