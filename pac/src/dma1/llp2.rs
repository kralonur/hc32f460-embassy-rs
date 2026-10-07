#[doc = "Register `LLP2` reader"]
pub type R = crate::R<Llp2Spec>;
#[doc = "Register `LLP2` writer"]
pub type W = crate::W<Llp2Spec>;
#[doc = "Field `LLP` reader - desc LLP"]
pub type LlpR = crate::FieldReader<u32>;
#[doc = "Field `LLP` writer - desc LLP"]
pub type LlpW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - desc LLP"]
    #[inline(always)]
    pub fn llp(&self) -> LlpR {
        LlpR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - desc LLP"]
    #[inline(always)]
    pub fn llp(&mut self) -> LlpW<'_, Llp2Spec> {
        LlpW::new(self, 2)
    }
}
#[doc = "desc LLP2\n\nYou can [`read`](crate::Reg::read) this register and get [`llp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Llp2Spec;
impl crate::RegisterSpec for Llp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llp2::R`](R) reader structure"]
impl crate::Readable for Llp2Spec {}
#[doc = "`write(|w| ..)` method takes [`llp2::W`](W) writer structure"]
impl crate::Writable for Llp2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LLP2 to value 0"]
impl crate::Resettable for Llp2Spec {}
