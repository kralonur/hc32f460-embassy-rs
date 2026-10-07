#[doc = "Register `CCSR` reader"]
pub type R = crate::R<CcsrSpec>;
#[doc = "Register `CCSR` writer"]
pub type W = crate::W<CcsrSpec>;
#[doc = "Field `CKDIV` reader - desc CKDIV"]
pub type CkdivR = crate::FieldReader;
#[doc = "Field `CKDIV` writer - desc CKDIV"]
pub type CkdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLEAR` reader - desc CLEAR"]
pub type ClearR = crate::BitReader;
#[doc = "Field `CLEAR` writer - desc CLEAR"]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - desc STOP"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - desc STOP"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFEN` reader - desc BUFEN"]
pub type BufenR = crate::BitReader;
#[doc = "Field `BUFEN` writer - desc BUFEN"]
pub type BufenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQPEN` reader - desc IRQPEN"]
pub type IrqpenR = crate::BitReader;
#[doc = "Field `IRQPEN` writer - desc IRQPEN"]
pub type IrqpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQPF` reader - desc IRQPF"]
pub type IrqpfR = crate::BitReader;
#[doc = "Field `IRQPF` writer - desc IRQPF"]
pub type IrqpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQZEN` reader - desc IRQZEN"]
pub type IrqzenR = crate::BitReader;
#[doc = "Field `IRQZEN` writer - desc IRQZEN"]
pub type IrqzenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQZF` reader - desc IRQZF"]
pub type IrqzfR = crate::BitReader;
#[doc = "Field `IRQZF` writer - desc IRQZF"]
pub type IrqzfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECKEN` reader - desc ECKEN"]
pub type EckenR = crate::BitReader;
#[doc = "Field `ECKEN` writer - desc ECKEN"]
pub type EckenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - desc CKDIV"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CkdivR {
        CkdivR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - desc CLEAR"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BUFEN"]
    #[inline(always)]
    pub fn bufen(&self) -> BufenR {
        BufenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc IRQPEN"]
    #[inline(always)]
    pub fn irqpen(&self) -> IrqpenR {
        IrqpenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc IRQPF"]
    #[inline(always)]
    pub fn irqpf(&self) -> IrqpfR {
        IrqpfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - desc IRQZEN"]
    #[inline(always)]
    pub fn irqzen(&self) -> IrqzenR {
        IrqzenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc IRQZF"]
    #[inline(always)]
    pub fn irqzf(&self) -> IrqzfR {
        IrqzfR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc ECKEN"]
    #[inline(always)]
    pub fn ecken(&self) -> EckenR {
        EckenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc CKDIV"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CkdivW<'_, CcsrSpec> {
        CkdivW::new(self, 0)
    }
    #[doc = "Bit 4 - desc CLEAR"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<'_, CcsrSpec> {
        ClearW::new(self, 4)
    }
    #[doc = "Bit 5 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CcsrSpec> {
        ModeW::new(self, 5)
    }
    #[doc = "Bit 6 - desc STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, CcsrSpec> {
        StopW::new(self, 6)
    }
    #[doc = "Bit 7 - desc BUFEN"]
    #[inline(always)]
    pub fn bufen(&mut self) -> BufenW<'_, CcsrSpec> {
        BufenW::new(self, 7)
    }
    #[doc = "Bit 8 - desc IRQPEN"]
    #[inline(always)]
    pub fn irqpen(&mut self) -> IrqpenW<'_, CcsrSpec> {
        IrqpenW::new(self, 8)
    }
    #[doc = "Bit 9 - desc IRQPF"]
    #[inline(always)]
    pub fn irqpf(&mut self) -> IrqpfW<'_, CcsrSpec> {
        IrqpfW::new(self, 9)
    }
    #[doc = "Bit 13 - desc IRQZEN"]
    #[inline(always)]
    pub fn irqzen(&mut self) -> IrqzenW<'_, CcsrSpec> {
        IrqzenW::new(self, 13)
    }
    #[doc = "Bit 14 - desc IRQZF"]
    #[inline(always)]
    pub fn irqzf(&mut self) -> IrqzfW<'_, CcsrSpec> {
        IrqzfW::new(self, 14)
    }
    #[doc = "Bit 15 - desc ECKEN"]
    #[inline(always)]
    pub fn ecken(&mut self) -> EckenW<'_, CcsrSpec> {
        EckenW::new(self, 15)
    }
}
#[doc = "desc CCSR\n\nYou can [`read`](crate::Reg::read) this register and get [`ccsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcsrSpec;
impl crate::RegisterSpec for CcsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ccsr::R`](R) reader structure"]
impl crate::Readable for CcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccsr::W`](W) writer structure"]
impl crate::Writable for CcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCSR to value 0x40"]
impl crate::Resettable for CcsrSpec {
    const RESET_VALUE: u16 = 0x40;
}
