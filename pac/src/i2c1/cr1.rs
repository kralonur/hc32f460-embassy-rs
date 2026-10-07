#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `PE` reader - desc PE"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - desc PE"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBUS` reader - desc SMBUS"]
pub type SmbusR = crate::BitReader;
#[doc = "Field `SMBUS` writer - desc SMBUS"]
pub type SmbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBALRTEN` reader - desc SMBALRTEN"]
pub type SmbalrtenR = crate::BitReader;
#[doc = "Field `SMBALRTEN` writer - desc SMBALRTEN"]
pub type SmbalrtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBDEFAULTEN` reader - desc SMBDEFAULTEN"]
pub type SmbdefaultenR = crate::BitReader;
#[doc = "Field `SMBDEFAULTEN` writer - desc SMBDEFAULTEN"]
pub type SmbdefaultenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBHOSTEN` reader - desc SMBHOSTEN"]
pub type SmbhostenR = crate::BitReader;
#[doc = "Field `SMBHOSTEN` writer - desc SMBHOSTEN"]
pub type SmbhostenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENGC` reader - desc ENGC"]
pub type EngcR = crate::BitReader;
#[doc = "Field `ENGC` writer - desc ENGC"]
pub type EngcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTART` reader - desc RESTART"]
pub type RestartR = crate::BitReader;
#[doc = "Field `RESTART` writer - desc RESTART"]
pub type RestartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - desc START"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - desc START"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - desc STOP"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - desc STOP"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - desc ACK"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - desc ACK"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` reader - desc SWRST"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - desc SWRST"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PE"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SMBUS"]
    #[inline(always)]
    pub fn smbus(&self) -> SmbusR {
        SmbusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SMBALRTEN"]
    #[inline(always)]
    pub fn smbalrten(&self) -> SmbalrtenR {
        SmbalrtenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SMBDEFAULTEN"]
    #[inline(always)]
    pub fn smbdefaulten(&self) -> SmbdefaultenR {
        SmbdefaultenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc SMBHOSTEN"]
    #[inline(always)]
    pub fn smbhosten(&self) -> SmbhostenR {
        SmbhostenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ENGC"]
    #[inline(always)]
    pub fn engc(&self) -> EngcR {
        EngcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RESTART"]
    #[inline(always)]
    pub fn restart(&self) -> RestartR {
        RestartR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ACK"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - desc SWRST"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PE"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, Cr1Spec> {
        PeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SMBUS"]
    #[inline(always)]
    pub fn smbus(&mut self) -> SmbusW<'_, Cr1Spec> {
        SmbusW::new(self, 1)
    }
    #[doc = "Bit 2 - desc SMBALRTEN"]
    #[inline(always)]
    pub fn smbalrten(&mut self) -> SmbalrtenW<'_, Cr1Spec> {
        SmbalrtenW::new(self, 2)
    }
    #[doc = "Bit 3 - desc SMBDEFAULTEN"]
    #[inline(always)]
    pub fn smbdefaulten(&mut self) -> SmbdefaultenW<'_, Cr1Spec> {
        SmbdefaultenW::new(self, 3)
    }
    #[doc = "Bit 4 - desc SMBHOSTEN"]
    #[inline(always)]
    pub fn smbhosten(&mut self) -> SmbhostenW<'_, Cr1Spec> {
        SmbhostenW::new(self, 4)
    }
    #[doc = "Bit 6 - desc ENGC"]
    #[inline(always)]
    pub fn engc(&mut self) -> EngcW<'_, Cr1Spec> {
        EngcW::new(self, 6)
    }
    #[doc = "Bit 7 - desc RESTART"]
    #[inline(always)]
    pub fn restart(&mut self) -> RestartW<'_, Cr1Spec> {
        RestartW::new(self, 7)
    }
    #[doc = "Bit 8 - desc START"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, Cr1Spec> {
        StartW::new(self, 8)
    }
    #[doc = "Bit 9 - desc STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, Cr1Spec> {
        StopW::new(self, 9)
    }
    #[doc = "Bit 10 - desc ACK"]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<'_, Cr1Spec> {
        AckW::new(self, 10)
    }
    #[doc = "Bit 15 - desc SWRST"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, Cr1Spec> {
        SwrstW::new(self, 15)
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
#[doc = "`reset()` method sets CR1 to value 0x40"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0x40;
}
