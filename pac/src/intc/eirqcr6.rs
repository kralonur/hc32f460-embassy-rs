#[doc = "Register `EIRQCR6` reader"]
pub type R = crate::R<Eirqcr6Spec>;
#[doc = "Register `EIRQCR6` writer"]
pub type W = crate::W<Eirqcr6Spec>;
#[doc = "Field `EIRQTRG` reader - desc EIRQTRG"]
pub type EirqtrgR = crate::FieldReader;
#[doc = "Field `EIRQTRG` writer - desc EIRQTRG"]
pub type EirqtrgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EISMPCLK` reader - desc EISMPCLK"]
pub type EismpclkR = crate::FieldReader;
#[doc = "Field `EISMPCLK` writer - desc EISMPCLK"]
pub type EismpclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EFEN` reader - desc EFEN"]
pub type EfenR = crate::BitReader;
#[doc = "Field `EFEN` writer - desc EFEN"]
pub type EfenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc EIRQTRG"]
    #[inline(always)]
    pub fn eirqtrg(&self) -> EirqtrgR {
        EirqtrgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc EISMPCLK"]
    #[inline(always)]
    pub fn eismpclk(&self) -> EismpclkR {
        EismpclkR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - desc EFEN"]
    #[inline(always)]
    pub fn efen(&self) -> EfenR {
        EfenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc EIRQTRG"]
    #[inline(always)]
    pub fn eirqtrg(&mut self) -> EirqtrgW<'_, Eirqcr6Spec> {
        EirqtrgW::new(self, 0)
    }
    #[doc = "Bits 4:5 - desc EISMPCLK"]
    #[inline(always)]
    pub fn eismpclk(&mut self) -> EismpclkW<'_, Eirqcr6Spec> {
        EismpclkW::new(self, 4)
    }
    #[doc = "Bit 7 - desc EFEN"]
    #[inline(always)]
    pub fn efen(&mut self) -> EfenW<'_, Eirqcr6Spec> {
        EfenW::new(self, 7)
    }
}
#[doc = "desc EIRQCR6\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eirqcr6Spec;
impl crate::RegisterSpec for Eirqcr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eirqcr6::R`](R) reader structure"]
impl crate::Readable for Eirqcr6Spec {}
#[doc = "`write(|w| ..)` method takes [`eirqcr6::W`](W) writer structure"]
impl crate::Writable for Eirqcr6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EIRQCR6 to value 0"]
impl crate::Resettable for Eirqcr6Spec {}
