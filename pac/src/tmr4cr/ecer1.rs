#[doc = "Register `ECER1` reader"]
pub type R = crate::R<Ecer1Spec>;
#[doc = "Register `ECER1` writer"]
pub type W = crate::W<Ecer1Spec>;
#[doc = "Field `EMBVAL` reader - desc EMBVAL"]
pub type EmbvalR = crate::FieldReader;
#[doc = "Field `EMBVAL` writer - desc EMBVAL"]
pub type EmbvalW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - desc EMBVAL"]
    #[inline(always)]
    pub fn embval(&self) -> EmbvalR {
        EmbvalR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc EMBVAL"]
    #[inline(always)]
    pub fn embval(&mut self) -> EmbvalW<'_, Ecer1Spec> {
        EmbvalW::new(self, 0)
    }
}
#[doc = "desc ECER1\n\nYou can [`read`](crate::Reg::read) this register and get [`ecer1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecer1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ecer1Spec;
impl crate::RegisterSpec for Ecer1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecer1::R`](R) reader structure"]
impl crate::Readable for Ecer1Spec {}
#[doc = "`write(|w| ..)` method takes [`ecer1::W`](W) writer structure"]
impl crate::Writable for Ecer1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECER1 to value 0"]
impl crate::Resettable for Ecer1Spec {}
