#[doc = "Register `I2SCKSEL` reader"]
pub type R = crate::R<I2sckselSpec>;
#[doc = "Register `I2SCKSEL` writer"]
pub type W = crate::W<I2sckselSpec>;
#[doc = "Field `I2S1CKSEL` reader - desc I2S1CKSEL"]
pub type I2s1ckselR = crate::FieldReader;
#[doc = "Field `I2S1CKSEL` writer - desc I2S1CKSEL"]
pub type I2s1ckselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `I2S2CKSEL` reader - desc I2S2CKSEL"]
pub type I2s2ckselR = crate::FieldReader;
#[doc = "Field `I2S2CKSEL` writer - desc I2S2CKSEL"]
pub type I2s2ckselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `I2S3CKSEL` reader - desc I2S3CKSEL"]
pub type I2s3ckselR = crate::FieldReader;
#[doc = "Field `I2S3CKSEL` writer - desc I2S3CKSEL"]
pub type I2s3ckselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `I2S4CKSEL` reader - desc I2S4CKSEL"]
pub type I2s4ckselR = crate::FieldReader;
#[doc = "Field `I2S4CKSEL` writer - desc I2S4CKSEL"]
pub type I2s4ckselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc I2S1CKSEL"]
    #[inline(always)]
    pub fn i2s1cksel(&self) -> I2s1ckselR {
        I2s1ckselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc I2S2CKSEL"]
    #[inline(always)]
    pub fn i2s2cksel(&self) -> I2s2ckselR {
        I2s2ckselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc I2S3CKSEL"]
    #[inline(always)]
    pub fn i2s3cksel(&self) -> I2s3ckselR {
        I2s3ckselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc I2S4CKSEL"]
    #[inline(always)]
    pub fn i2s4cksel(&self) -> I2s4ckselR {
        I2s4ckselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc I2S1CKSEL"]
    #[inline(always)]
    pub fn i2s1cksel(&mut self) -> I2s1ckselW<'_, I2sckselSpec> {
        I2s1ckselW::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc I2S2CKSEL"]
    #[inline(always)]
    pub fn i2s2cksel(&mut self) -> I2s2ckselW<'_, I2sckselSpec> {
        I2s2ckselW::new(self, 4)
    }
    #[doc = "Bits 8:11 - desc I2S3CKSEL"]
    #[inline(always)]
    pub fn i2s3cksel(&mut self) -> I2s3ckselW<'_, I2sckselSpec> {
        I2s3ckselW::new(self, 8)
    }
    #[doc = "Bits 12:15 - desc I2S4CKSEL"]
    #[inline(always)]
    pub fn i2s4cksel(&mut self) -> I2s4ckselW<'_, I2sckselSpec> {
        I2s4ckselW::new(self, 12)
    }
}
#[doc = "desc I2SCKSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`i2scksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2scksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sckselSpec;
impl crate::RegisterSpec for I2sckselSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`i2scksel::R`](R) reader structure"]
impl crate::Readable for I2sckselSpec {}
#[doc = "`write(|w| ..)` method takes [`i2scksel::W`](W) writer structure"]
impl crate::Writable for I2sckselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2SCKSEL to value 0xbbbb"]
impl crate::Resettable for I2sckselSpec {
    const RESET_VALUE: u16 = 0xbbbb;
}
