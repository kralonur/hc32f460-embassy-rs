#[doc = "Register `SCR` reader"]
pub type R = crate::R<ScrSpec>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `KEYINSEL` reader - desc KEYINSEL"]
pub type KeyinselR = crate::FieldReader<u16>;
#[doc = "Field `KEYINSEL` writer - desc KEYINSEL"]
pub type KeyinselW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `KEYOUTSEL` reader - desc KEYOUTSEL"]
pub type KeyoutselR = crate::FieldReader;
#[doc = "Field `KEYOUTSEL` writer - desc KEYOUTSEL"]
pub type KeyoutselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKSEL` reader - desc CKSEL"]
pub type CkselR = crate::FieldReader;
#[doc = "Field `CKSEL` writer - desc CKSEL"]
pub type CkselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `T_LLEVEL` reader - desc T_LLEVEL"]
pub type TLlevelR = crate::FieldReader;
#[doc = "Field `T_LLEVEL` writer - desc T_LLEVEL"]
pub type TLlevelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `T_HIZ` reader - desc T_HIZ"]
pub type THizR = crate::FieldReader;
#[doc = "Field `T_HIZ` writer - desc T_HIZ"]
pub type THizW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - desc KEYINSEL"]
    #[inline(always)]
    pub fn keyinsel(&self) -> KeyinselR {
        KeyinselR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - desc KEYOUTSEL"]
    #[inline(always)]
    pub fn keyoutsel(&self) -> KeyoutselR {
        KeyoutselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - desc CKSEL"]
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:28 - desc T_LLEVEL"]
    #[inline(always)]
    pub fn t_llevel(&self) -> TLlevelR {
        TLlevelR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - desc T_HIZ"]
    #[inline(always)]
    pub fn t_hiz(&self) -> THizR {
        THizR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc KEYINSEL"]
    #[inline(always)]
    pub fn keyinsel(&mut self) -> KeyinselW<'_, ScrSpec> {
        KeyinselW::new(self, 0)
    }
    #[doc = "Bits 16:18 - desc KEYOUTSEL"]
    #[inline(always)]
    pub fn keyoutsel(&mut self) -> KeyoutselW<'_, ScrSpec> {
        KeyoutselW::new(self, 16)
    }
    #[doc = "Bits 20:21 - desc CKSEL"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CkselW<'_, ScrSpec> {
        CkselW::new(self, 20)
    }
    #[doc = "Bits 24:28 - desc T_LLEVEL"]
    #[inline(always)]
    pub fn t_llevel(&mut self) -> TLlevelW<'_, ScrSpec> {
        TLlevelW::new(self, 24)
    }
    #[doc = "Bits 29:31 - desc T_HIZ"]
    #[inline(always)]
    pub fn t_hiz(&mut self) -> THizW<'_, ScrSpec> {
        THizW::new(self, 29)
    }
}
#[doc = "desc SCR\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for ScrSpec {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {}
