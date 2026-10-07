#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DATASIZE` reader - desc DATASIZE"]
pub type DatasizeR = crate::FieldReader;
#[doc = "Field `DATASIZE` writer - desc DATASIZE"]
pub type DatasizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMPTRG` reader - desc COMPTRG"]
pub type ComptrgR = crate::BitReader;
#[doc = "Field `COMPTRG` writer - desc COMPTRG"]
pub type ComptrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN` reader - desc INTEN"]
pub type IntenR = crate::BitReader;
#[doc = "Field `INTEN` writer - desc INTEN"]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - desc DATASIZE"]
    #[inline(always)]
    pub fn datasize(&self) -> DatasizeR {
        DatasizeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 8 - desc COMPTRG"]
    #[inline(always)]
    pub fn comptrg(&self) -> ComptrgR {
        ComptrgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - desc INTEN"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CtlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 3:4 - desc DATASIZE"]
    #[inline(always)]
    pub fn datasize(&mut self) -> DatasizeW<'_, CtlSpec> {
        DatasizeW::new(self, 3)
    }
    #[doc = "Bit 8 - desc COMPTRG"]
    #[inline(always)]
    pub fn comptrg(&mut self) -> ComptrgW<'_, CtlSpec> {
        ComptrgW::new(self, 8)
    }
    #[doc = "Bit 31 - desc INTEN"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<'_, CtlSpec> {
        IntenW::new(self, 31)
    }
}
#[doc = "desc CTL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL to value 0x8000_0000"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
