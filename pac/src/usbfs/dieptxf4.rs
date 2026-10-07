#[doc = "Register `DIEPTXF4` reader"]
pub type R = crate::R<Dieptxf4Spec>;
#[doc = "Register `DIEPTXF4` writer"]
pub type W = crate::W<Dieptxf4Spec>;
#[doc = "Field `INEPTXSA` reader - desc INEPTXSA"]
pub type IneptxsaR = crate::FieldReader<u16>;
#[doc = "Field `INEPTXSA` writer - desc INEPTXSA"]
pub type IneptxsaW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `INEPTXFD` reader - desc INEPTXFD"]
pub type IneptxfdR = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFD` writer - desc INEPTXFD"]
pub type IneptxfdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:11 - desc INEPTXSA"]
    #[inline(always)]
    pub fn ineptxsa(&self) -> IneptxsaR {
        IneptxsaR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25 - desc INEPTXFD"]
    #[inline(always)]
    pub fn ineptxfd(&self) -> IneptxfdR {
        IneptxfdR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc INEPTXSA"]
    #[inline(always)]
    pub fn ineptxsa(&mut self) -> IneptxsaW<'_, Dieptxf4Spec> {
        IneptxsaW::new(self, 0)
    }
    #[doc = "Bits 16:25 - desc INEPTXFD"]
    #[inline(always)]
    pub fn ineptxfd(&mut self) -> IneptxfdW<'_, Dieptxf4Spec> {
        IneptxfdW::new(self, 16)
    }
}
#[doc = "desc DIEPTXF4\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dieptxf4Spec;
impl crate::RegisterSpec for Dieptxf4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf4::R`](R) reader structure"]
impl crate::Readable for Dieptxf4Spec {}
#[doc = "`write(|w| ..)` method takes [`dieptxf4::W`](W) writer structure"]
impl crate::Writable for Dieptxf4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPTXF4 to value 0x0100_0540"]
impl crate::Resettable for Dieptxf4Spec {
    const RESET_VALUE: u32 = 0x0100_0540;
}
