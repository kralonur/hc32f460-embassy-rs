#[doc = "Register `TMR0_TRGSEL` reader"]
pub type R = crate::R<Tmr0TrgselSpec>;
#[doc = "Register `TMR0_TRGSEL` writer"]
pub type W = crate::W<Tmr0TrgselSpec>;
#[doc = "Field `TRGSEL` reader - desc TRGSEL"]
pub type TrgselR = crate::FieldReader<u16>;
#[doc = "Field `TRGSEL` writer - desc TRGSEL"]
pub type TrgselW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `COMEN` reader - desc COMEN"]
pub type ComenR = crate::FieldReader;
#[doc = "Field `COMEN` writer - desc COMEN"]
pub type ComenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:8 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 30:31 - desc COMEN"]
    #[inline(always)]
    pub fn comen(&self) -> ComenR {
        ComenR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TrgselW<'_, Tmr0TrgselSpec> {
        TrgselW::new(self, 0)
    }
    #[doc = "Bits 30:31 - desc COMEN"]
    #[inline(always)]
    pub fn comen(&mut self) -> ComenW<'_, Tmr0TrgselSpec> {
        ComenW::new(self, 30)
    }
}
#[doc = "desc TMR0_TRGSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr0_trgsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr0_trgsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr0TrgselSpec;
impl crate::RegisterSpec for Tmr0TrgselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr0_trgsel::R`](R) reader structure"]
impl crate::Readable for Tmr0TrgselSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr0_trgsel::W`](W) writer structure"]
impl crate::Writable for Tmr0TrgselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMR0_TRGSEL to value 0x01ff"]
impl crate::Resettable for Tmr0TrgselSpec {
    const RESET_VALUE: u32 = 0x01ff;
}
