#[doc = "Register `SEC` reader"]
pub type R = crate::R<SecSpec>;
#[doc = "Register `SEC` writer"]
pub type W = crate::W<SecSpec>;
#[doc = "Field `SECU` reader - desc SECU"]
pub type SecuR = crate::FieldReader;
#[doc = "Field `SECU` writer - desc SECU"]
pub type SecuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SECD` reader - desc SECD"]
pub type SecdR = crate::FieldReader;
#[doc = "Field `SECD` writer - desc SECD"]
pub type SecdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - desc SECU"]
    #[inline(always)]
    pub fn secu(&self) -> SecuR {
        SecuR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - desc SECD"]
    #[inline(always)]
    pub fn secd(&self) -> SecdR {
        SecdR::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc SECU"]
    #[inline(always)]
    pub fn secu(&mut self) -> SecuW<'_, SecSpec> {
        SecuW::new(self, 0)
    }
    #[doc = "Bits 4:6 - desc SECD"]
    #[inline(always)]
    pub fn secd(&mut self) -> SecdW<'_, SecSpec> {
        SecdW::new(self, 4)
    }
}
#[doc = "desc SEC\n\nYou can [`read`](crate::Reg::read) this register and get [`sec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecSpec;
impl crate::RegisterSpec for SecSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sec::R`](R) reader structure"]
impl crate::Readable for SecSpec {}
#[doc = "`write(|w| ..)` method takes [`sec::W`](W) writer structure"]
impl crate::Writable for SecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC to value 0"]
impl crate::Resettable for SecSpec {}
