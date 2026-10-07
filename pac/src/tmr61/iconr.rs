#[doc = "Register `ICONR` reader"]
pub type R = crate::R<IconrSpec>;
#[doc = "Register `ICONR` writer"]
pub type W = crate::W<IconrSpec>;
#[doc = "Field `INTENA` reader - desc INTENA"]
pub type IntenaR = crate::BitReader;
#[doc = "Field `INTENA` writer - desc INTENA"]
pub type IntenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENB` reader - desc INTENB"]
pub type IntenbR = crate::BitReader;
#[doc = "Field `INTENB` writer - desc INTENB"]
pub type IntenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENC` reader - desc INTENC"]
pub type IntencR = crate::BitReader;
#[doc = "Field `INTENC` writer - desc INTENC"]
pub type IntencW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEND` reader - desc INTEND"]
pub type IntendR = crate::BitReader;
#[doc = "Field `INTEND` writer - desc INTEND"]
pub type IntendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENE` reader - desc INTENE"]
pub type InteneR = crate::BitReader;
#[doc = "Field `INTENE` writer - desc INTENE"]
pub type InteneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENF` reader - desc INTENF"]
pub type IntenfR = crate::BitReader;
#[doc = "Field `INTENF` writer - desc INTENF"]
pub type IntenfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENOVF` reader - desc INTENOVF"]
pub type IntenovfR = crate::BitReader;
#[doc = "Field `INTENOVF` writer - desc INTENOVF"]
pub type IntenovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENUDF` reader - desc INTENUDF"]
pub type IntenudfR = crate::BitReader;
#[doc = "Field `INTENUDF` writer - desc INTENUDF"]
pub type IntenudfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENDTE` reader - desc INTENDTE"]
pub type IntendteR = crate::BitReader;
#[doc = "Field `INTENDTE` writer - desc INTENDTE"]
pub type IntendteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENSAU` reader - desc INTENSAU"]
pub type IntensauR = crate::BitReader;
#[doc = "Field `INTENSAU` writer - desc INTENSAU"]
pub type IntensauW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENSAD` reader - desc INTENSAD"]
pub type IntensadR = crate::BitReader;
#[doc = "Field `INTENSAD` writer - desc INTENSAD"]
pub type IntensadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENSBU` reader - desc INTENSBU"]
pub type IntensbuR = crate::BitReader;
#[doc = "Field `INTENSBU` writer - desc INTENSBU"]
pub type IntensbuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTENSBD` reader - desc INTENSBD"]
pub type IntensbdR = crate::BitReader;
#[doc = "Field `INTENSBD` writer - desc INTENSBD"]
pub type IntensbdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc INTENA"]
    #[inline(always)]
    pub fn intena(&self) -> IntenaR {
        IntenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc INTENB"]
    #[inline(always)]
    pub fn intenb(&self) -> IntenbR {
        IntenbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc INTENC"]
    #[inline(always)]
    pub fn intenc(&self) -> IntencR {
        IntencR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc INTEND"]
    #[inline(always)]
    pub fn intend(&self) -> IntendR {
        IntendR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc INTENE"]
    #[inline(always)]
    pub fn intene(&self) -> InteneR {
        InteneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc INTENF"]
    #[inline(always)]
    pub fn intenf(&self) -> IntenfR {
        IntenfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc INTENOVF"]
    #[inline(always)]
    pub fn intenovf(&self) -> IntenovfR {
        IntenovfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc INTENUDF"]
    #[inline(always)]
    pub fn intenudf(&self) -> IntenudfR {
        IntenudfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc INTENDTE"]
    #[inline(always)]
    pub fn intendte(&self) -> IntendteR {
        IntendteR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - desc INTENSAU"]
    #[inline(always)]
    pub fn intensau(&self) -> IntensauR {
        IntensauR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc INTENSAD"]
    #[inline(always)]
    pub fn intensad(&self) -> IntensadR {
        IntensadR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc INTENSBU"]
    #[inline(always)]
    pub fn intensbu(&self) -> IntensbuR {
        IntensbuR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc INTENSBD"]
    #[inline(always)]
    pub fn intensbd(&self) -> IntensbdR {
        IntensbdR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc INTENA"]
    #[inline(always)]
    pub fn intena(&mut self) -> IntenaW<'_, IconrSpec> {
        IntenaW::new(self, 0)
    }
    #[doc = "Bit 1 - desc INTENB"]
    #[inline(always)]
    pub fn intenb(&mut self) -> IntenbW<'_, IconrSpec> {
        IntenbW::new(self, 1)
    }
    #[doc = "Bit 2 - desc INTENC"]
    #[inline(always)]
    pub fn intenc(&mut self) -> IntencW<'_, IconrSpec> {
        IntencW::new(self, 2)
    }
    #[doc = "Bit 3 - desc INTEND"]
    #[inline(always)]
    pub fn intend(&mut self) -> IntendW<'_, IconrSpec> {
        IntendW::new(self, 3)
    }
    #[doc = "Bit 4 - desc INTENE"]
    #[inline(always)]
    pub fn intene(&mut self) -> InteneW<'_, IconrSpec> {
        InteneW::new(self, 4)
    }
    #[doc = "Bit 5 - desc INTENF"]
    #[inline(always)]
    pub fn intenf(&mut self) -> IntenfW<'_, IconrSpec> {
        IntenfW::new(self, 5)
    }
    #[doc = "Bit 6 - desc INTENOVF"]
    #[inline(always)]
    pub fn intenovf(&mut self) -> IntenovfW<'_, IconrSpec> {
        IntenovfW::new(self, 6)
    }
    #[doc = "Bit 7 - desc INTENUDF"]
    #[inline(always)]
    pub fn intenudf(&mut self) -> IntenudfW<'_, IconrSpec> {
        IntenudfW::new(self, 7)
    }
    #[doc = "Bit 8 - desc INTENDTE"]
    #[inline(always)]
    pub fn intendte(&mut self) -> IntendteW<'_, IconrSpec> {
        IntendteW::new(self, 8)
    }
    #[doc = "Bit 16 - desc INTENSAU"]
    #[inline(always)]
    pub fn intensau(&mut self) -> IntensauW<'_, IconrSpec> {
        IntensauW::new(self, 16)
    }
    #[doc = "Bit 17 - desc INTENSAD"]
    #[inline(always)]
    pub fn intensad(&mut self) -> IntensadW<'_, IconrSpec> {
        IntensadW::new(self, 17)
    }
    #[doc = "Bit 18 - desc INTENSBU"]
    #[inline(always)]
    pub fn intensbu(&mut self) -> IntensbuW<'_, IconrSpec> {
        IntensbuW::new(self, 18)
    }
    #[doc = "Bit 19 - desc INTENSBD"]
    #[inline(always)]
    pub fn intensbd(&mut self) -> IntensbdW<'_, IconrSpec> {
        IntensbdW::new(self, 19)
    }
}
#[doc = "desc ICONR\n\nYou can [`read`](crate::Reg::read) this register and get [`iconr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iconr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IconrSpec;
impl crate::RegisterSpec for IconrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iconr::R`](R) reader structure"]
impl crate::Readable for IconrSpec {}
#[doc = "`write(|w| ..)` method takes [`iconr::W`](W) writer structure"]
impl crate::Writable for IconrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICONR to value 0"]
impl crate::Resettable for IconrSpec {}
