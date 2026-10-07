#[doc = "Register `ECER2` reader"]
pub type R = crate::R<Ecer2Spec>;
#[doc = "Register `ECER2` writer"]
pub type W = crate::W<Ecer2Spec>;
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
    pub fn embval(&mut self) -> EmbvalW<'_, Ecer2Spec> {
        EmbvalW::new(self, 0)
    }
}
#[doc = "desc ECER2\n\nYou can [`read`](crate::Reg::read) this register and get [`ecer2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecer2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ecer2Spec;
impl crate::RegisterSpec for Ecer2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecer2::R`](R) reader structure"]
impl crate::Readable for Ecer2Spec {}
#[doc = "`write(|w| ..)` method takes [`ecer2::W`](W) writer structure"]
impl crate::Writable for Ecer2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECER2 to value 0"]
impl crate::Resettable for Ecer2Spec {}
