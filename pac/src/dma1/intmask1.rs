#[doc = "Register `INTMASK1` reader"]
pub type R = crate::R<Intmask1Spec>;
#[doc = "Register `INTMASK1` writer"]
pub type W = crate::W<Intmask1Spec>;
#[doc = "Field `MSKTC` reader - desc MSKTC"]
pub type MsktcR = crate::FieldReader;
#[doc = "Field `MSKTC` writer - desc MSKTC"]
pub type MsktcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MSKBTC` reader - desc MSKBTC"]
pub type MskbtcR = crate::FieldReader;
#[doc = "Field `MSKBTC` writer - desc MSKBTC"]
pub type MskbtcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc MSKTC"]
    #[inline(always)]
    pub fn msktc(&self) -> MsktcR {
        MsktcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - desc MSKBTC"]
    #[inline(always)]
    pub fn mskbtc(&self) -> MskbtcR {
        MskbtcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc MSKTC"]
    #[inline(always)]
    pub fn msktc(&mut self) -> MsktcW<'_, Intmask1Spec> {
        MsktcW::new(self, 0)
    }
    #[doc = "Bits 16:19 - desc MSKBTC"]
    #[inline(always)]
    pub fn mskbtc(&mut self) -> MskbtcW<'_, Intmask1Spec> {
        MskbtcW::new(self, 16)
    }
}
#[doc = "desc INTMASK1\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intmask1Spec;
impl crate::RegisterSpec for Intmask1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmask1::R`](R) reader structure"]
impl crate::Readable for Intmask1Spec {}
#[doc = "`write(|w| ..)` method takes [`intmask1::W`](W) writer structure"]
impl crate::Writable for Intmask1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTMASK1 to value 0"]
impl crate::Resettable for Intmask1Spec {}
