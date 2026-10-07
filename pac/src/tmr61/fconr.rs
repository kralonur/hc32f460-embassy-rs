#[doc = "Register `FCONR` reader"]
pub type R = crate::R<FconrSpec>;
#[doc = "Register `FCONR` writer"]
pub type W = crate::W<FconrSpec>;
#[doc = "Field `NOFIENGA` reader - desc NOFIENGA"]
pub type NofiengaR = crate::BitReader;
#[doc = "Field `NOFIENGA` writer - desc NOFIENGA"]
pub type NofiengaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOFICKGA` reader - desc NOFICKGA"]
pub type NofickgaR = crate::FieldReader;
#[doc = "Field `NOFICKGA` writer - desc NOFICKGA"]
pub type NofickgaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOFIENGB` reader - desc NOFIENGB"]
pub type NofiengbR = crate::BitReader;
#[doc = "Field `NOFIENGB` writer - desc NOFIENGB"]
pub type NofiengbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOFICKGB` reader - desc NOFICKGB"]
pub type NofickgbR = crate::FieldReader;
#[doc = "Field `NOFICKGB` writer - desc NOFICKGB"]
pub type NofickgbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOFIENTA` reader - desc NOFIENTA"]
pub type NofientaR = crate::BitReader;
#[doc = "Field `NOFIENTA` writer - desc NOFIENTA"]
pub type NofientaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOFICKTA` reader - desc NOFICKTA"]
pub type NoficktaR = crate::FieldReader;
#[doc = "Field `NOFICKTA` writer - desc NOFICKTA"]
pub type NoficktaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOFIENTB` reader - desc NOFIENTB"]
pub type NofientbR = crate::BitReader;
#[doc = "Field `NOFIENTB` writer - desc NOFIENTB"]
pub type NofientbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOFICKTB` reader - desc NOFICKTB"]
pub type NoficktbR = crate::FieldReader;
#[doc = "Field `NOFICKTB` writer - desc NOFICKTB"]
pub type NoficktbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc NOFIENGA"]
    #[inline(always)]
    pub fn nofienga(&self) -> NofiengaR {
        NofiengaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc NOFICKGA"]
    #[inline(always)]
    pub fn nofickga(&self) -> NofickgaR {
        NofickgaR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - desc NOFIENGB"]
    #[inline(always)]
    pub fn nofiengb(&self) -> NofiengbR {
        NofiengbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - desc NOFICKGB"]
    #[inline(always)]
    pub fn nofickgb(&self) -> NofickgbR {
        NofickgbR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 16 - desc NOFIENTA"]
    #[inline(always)]
    pub fn nofienta(&self) -> NofientaR {
        NofientaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - desc NOFICKTA"]
    #[inline(always)]
    pub fn nofickta(&self) -> NoficktaR {
        NoficktaR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 20 - desc NOFIENTB"]
    #[inline(always)]
    pub fn nofientb(&self) -> NofientbR {
        NofientbR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - desc NOFICKTB"]
    #[inline(always)]
    pub fn noficktb(&self) -> NoficktbR {
        NoficktbR::new(((self.bits >> 21) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc NOFIENGA"]
    #[inline(always)]
    pub fn nofienga(&mut self) -> NofiengaW<'_, FconrSpec> {
        NofiengaW::new(self, 0)
    }
    #[doc = "Bits 1:2 - desc NOFICKGA"]
    #[inline(always)]
    pub fn nofickga(&mut self) -> NofickgaW<'_, FconrSpec> {
        NofickgaW::new(self, 1)
    }
    #[doc = "Bit 4 - desc NOFIENGB"]
    #[inline(always)]
    pub fn nofiengb(&mut self) -> NofiengbW<'_, FconrSpec> {
        NofiengbW::new(self, 4)
    }
    #[doc = "Bits 5:6 - desc NOFICKGB"]
    #[inline(always)]
    pub fn nofickgb(&mut self) -> NofickgbW<'_, FconrSpec> {
        NofickgbW::new(self, 5)
    }
    #[doc = "Bit 16 - desc NOFIENTA"]
    #[inline(always)]
    pub fn nofienta(&mut self) -> NofientaW<'_, FconrSpec> {
        NofientaW::new(self, 16)
    }
    #[doc = "Bits 17:18 - desc NOFICKTA"]
    #[inline(always)]
    pub fn nofickta(&mut self) -> NoficktaW<'_, FconrSpec> {
        NoficktaW::new(self, 17)
    }
    #[doc = "Bit 20 - desc NOFIENTB"]
    #[inline(always)]
    pub fn nofientb(&mut self) -> NofientbW<'_, FconrSpec> {
        NofientbW::new(self, 20)
    }
    #[doc = "Bits 21:22 - desc NOFICKTB"]
    #[inline(always)]
    pub fn noficktb(&mut self) -> NoficktbW<'_, FconrSpec> {
        NoficktbW::new(self, 21)
    }
}
#[doc = "desc FCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`fconr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fconr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FconrSpec;
impl crate::RegisterSpec for FconrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fconr::R`](R) reader structure"]
impl crate::Readable for FconrSpec {}
#[doc = "`write(|w| ..)` method takes [`fconr::W`](W) writer structure"]
impl crate::Writable for FconrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCONR to value 0"]
impl crate::Resettable for FconrSpec {}
