#[doc = "Register `FCG1` reader"]
pub type R = crate::R<Fcg1Spec>;
#[doc = "Register `FCG1` writer"]
pub type W = crate::W<Fcg1Spec>;
#[doc = "Field `CAN` reader - desc CAN"]
pub type CanR = crate::BitReader;
#[doc = "Field `CAN` writer - desc CAN"]
pub type CanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI` reader - desc QSPI"]
pub type QspiR = crate::BitReader;
#[doc = "Field `QSPI` writer - desc QSPI"]
pub type QspiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - desc I2C1"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C1` writer - desc I2C1"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2` reader - desc I2C2"]
pub type I2c2R = crate::BitReader;
#[doc = "Field `I2C2` writer - desc I2C2"]
pub type I2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3` reader - desc I2C3"]
pub type I2c3R = crate::BitReader;
#[doc = "Field `I2C3` writer - desc I2C3"]
pub type I2c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBFS` reader - desc USBFS"]
pub type UsbfsR = crate::BitReader;
#[doc = "Field `USBFS` writer - desc USBFS"]
pub type UsbfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOC1` reader - desc SDIOC1"]
pub type Sdioc1R = crate::BitReader;
#[doc = "Field `SDIOC1` writer - desc SDIOC1"]
pub type Sdioc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOC2` reader - desc SDIOC2"]
pub type Sdioc2R = crate::BitReader;
#[doc = "Field `SDIOC2` writer - desc SDIOC2"]
pub type Sdioc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1` reader - desc I2S1"]
pub type I2s1R = crate::BitReader;
#[doc = "Field `I2S1` writer - desc I2S1"]
pub type I2s1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2` reader - desc I2S2"]
pub type I2s2R = crate::BitReader;
#[doc = "Field `I2S2` writer - desc I2S2"]
pub type I2s2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S3` reader - desc I2S3"]
pub type I2s3R = crate::BitReader;
#[doc = "Field `I2S3` writer - desc I2S3"]
pub type I2s3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S4` reader - desc I2S4"]
pub type I2s4R = crate::BitReader;
#[doc = "Field `I2S4` writer - desc I2S4"]
pub type I2s4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - desc SPI1"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI1` writer - desc SPI1"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2` reader - desc SPI2"]
pub type Spi2R = crate::BitReader;
#[doc = "Field `SPI2` writer - desc SPI2"]
pub type Spi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3` reader - desc SPI3"]
pub type Spi3R = crate::BitReader;
#[doc = "Field `SPI3` writer - desc SPI3"]
pub type Spi3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4` reader - desc SPI4"]
pub type Spi4R = crate::BitReader;
#[doc = "Field `SPI4` writer - desc SPI4"]
pub type Spi4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1` reader - desc USART1"]
pub type Usart1R = crate::BitReader;
#[doc = "Field `USART1` writer - desc USART1"]
pub type Usart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2` reader - desc USART2"]
pub type Usart2R = crate::BitReader;
#[doc = "Field `USART2` writer - desc USART2"]
pub type Usart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3` reader - desc USART3"]
pub type Usart3R = crate::BitReader;
#[doc = "Field `USART3` writer - desc USART3"]
pub type Usart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART4` reader - desc USART4"]
pub type Usart4R = crate::BitReader;
#[doc = "Field `USART4` writer - desc USART4"]
pub type Usart4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CAN"]
    #[inline(always)]
    pub fn can(&self) -> CanR {
        CanR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - desc QSPI"]
    #[inline(always)]
    pub fn qspi(&self) -> QspiR {
        QspiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc I2C1"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc I2C2"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2c2R {
        I2c2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc I2C3"]
    #[inline(always)]
    pub fn i2c3(&self) -> I2c3R {
        I2c3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc USBFS"]
    #[inline(always)]
    pub fn usbfs(&self) -> UsbfsR {
        UsbfsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - desc SDIOC1"]
    #[inline(always)]
    pub fn sdioc1(&self) -> Sdioc1R {
        Sdioc1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc SDIOC2"]
    #[inline(always)]
    pub fn sdioc2(&self) -> Sdioc2R {
        Sdioc2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc I2S1"]
    #[inline(always)]
    pub fn i2s1(&self) -> I2s1R {
        I2s1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc I2S2"]
    #[inline(always)]
    pub fn i2s2(&self) -> I2s2R {
        I2s2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc I2S3"]
    #[inline(always)]
    pub fn i2s3(&self) -> I2s3R {
        I2s3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc I2S4"]
    #[inline(always)]
    pub fn i2s4(&self) -> I2s4R {
        I2s4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> Spi2R {
        Spi2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc SPI3"]
    #[inline(always)]
    pub fn spi3(&self) -> Spi3R {
        Spi3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc SPI4"]
    #[inline(always)]
    pub fn spi4(&self) -> Spi4R {
        Spi4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - desc USART1"]
    #[inline(always)]
    pub fn usart1(&self) -> Usart1R {
        Usart1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc USART2"]
    #[inline(always)]
    pub fn usart2(&self) -> Usart2R {
        Usart2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc USART3"]
    #[inline(always)]
    pub fn usart3(&self) -> Usart3R {
        Usart3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc USART4"]
    #[inline(always)]
    pub fn usart4(&self) -> Usart4R {
        Usart4R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CAN"]
    #[inline(always)]
    pub fn can(&mut self) -> CanW<'_, Fcg1Spec> {
        CanW::new(self, 0)
    }
    #[doc = "Bit 3 - desc QSPI"]
    #[inline(always)]
    pub fn qspi(&mut self) -> QspiW<'_, Fcg1Spec> {
        QspiW::new(self, 3)
    }
    #[doc = "Bit 4 - desc I2C1"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<'_, Fcg1Spec> {
        I2c1W::new(self, 4)
    }
    #[doc = "Bit 5 - desc I2C2"]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2c2W<'_, Fcg1Spec> {
        I2c2W::new(self, 5)
    }
    #[doc = "Bit 6 - desc I2C3"]
    #[inline(always)]
    pub fn i2c3(&mut self) -> I2c3W<'_, Fcg1Spec> {
        I2c3W::new(self, 6)
    }
    #[doc = "Bit 8 - desc USBFS"]
    #[inline(always)]
    pub fn usbfs(&mut self) -> UsbfsW<'_, Fcg1Spec> {
        UsbfsW::new(self, 8)
    }
    #[doc = "Bit 10 - desc SDIOC1"]
    #[inline(always)]
    pub fn sdioc1(&mut self) -> Sdioc1W<'_, Fcg1Spec> {
        Sdioc1W::new(self, 10)
    }
    #[doc = "Bit 11 - desc SDIOC2"]
    #[inline(always)]
    pub fn sdioc2(&mut self) -> Sdioc2W<'_, Fcg1Spec> {
        Sdioc2W::new(self, 11)
    }
    #[doc = "Bit 12 - desc I2S1"]
    #[inline(always)]
    pub fn i2s1(&mut self) -> I2s1W<'_, Fcg1Spec> {
        I2s1W::new(self, 12)
    }
    #[doc = "Bit 13 - desc I2S2"]
    #[inline(always)]
    pub fn i2s2(&mut self) -> I2s2W<'_, Fcg1Spec> {
        I2s2W::new(self, 13)
    }
    #[doc = "Bit 14 - desc I2S3"]
    #[inline(always)]
    pub fn i2s3(&mut self) -> I2s3W<'_, Fcg1Spec> {
        I2s3W::new(self, 14)
    }
    #[doc = "Bit 15 - desc I2S4"]
    #[inline(always)]
    pub fn i2s4(&mut self) -> I2s4W<'_, Fcg1Spec> {
        I2s4W::new(self, 15)
    }
    #[doc = "Bit 16 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<'_, Fcg1Spec> {
        Spi1W::new(self, 16)
    }
    #[doc = "Bit 17 - desc SPI2"]
    #[inline(always)]
    pub fn spi2(&mut self) -> Spi2W<'_, Fcg1Spec> {
        Spi2W::new(self, 17)
    }
    #[doc = "Bit 18 - desc SPI3"]
    #[inline(always)]
    pub fn spi3(&mut self) -> Spi3W<'_, Fcg1Spec> {
        Spi3W::new(self, 18)
    }
    #[doc = "Bit 19 - desc SPI4"]
    #[inline(always)]
    pub fn spi4(&mut self) -> Spi4W<'_, Fcg1Spec> {
        Spi4W::new(self, 19)
    }
    #[doc = "Bit 24 - desc USART1"]
    #[inline(always)]
    pub fn usart1(&mut self) -> Usart1W<'_, Fcg1Spec> {
        Usart1W::new(self, 24)
    }
    #[doc = "Bit 25 - desc USART2"]
    #[inline(always)]
    pub fn usart2(&mut self) -> Usart2W<'_, Fcg1Spec> {
        Usart2W::new(self, 25)
    }
    #[doc = "Bit 26 - desc USART3"]
    #[inline(always)]
    pub fn usart3(&mut self) -> Usart3W<'_, Fcg1Spec> {
        Usart3W::new(self, 26)
    }
    #[doc = "Bit 27 - desc USART4"]
    #[inline(always)]
    pub fn usart4(&mut self) -> Usart4W<'_, Fcg1Spec> {
        Usart4W::new(self, 27)
    }
}
#[doc = "desc FCG1\n\nYou can [`read`](crate::Reg::read) this register and get [`fcg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fcg1Spec;
impl crate::RegisterSpec for Fcg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcg1::R`](R) reader structure"]
impl crate::Readable for Fcg1Spec {}
#[doc = "`write(|w| ..)` method takes [`fcg1::W`](W) writer structure"]
impl crate::Writable for Fcg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCG1 to value 0xffff_ffff"]
impl crate::Resettable for Fcg1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
