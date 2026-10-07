#[doc = "Register `PEVNTRISR3` reader"]
pub type R = crate::R<Pevntrisr3Spec>;
#[doc = "Register `PEVNTRISR3` writer"]
pub type W = crate::W<Pevntrisr3Spec>;
#[doc = "Field `RIS` reader - desc RIS"]
pub type RisR = crate::FieldReader<u16>;
#[doc = "Field `RIS` writer - desc RIS"]
pub type RisW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc RIS"]
    #[inline(always)]
    pub fn ris(&self) -> RisR {
        RisR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc RIS"]
    #[inline(always)]
    pub fn ris(&mut self) -> RisW<'_, Pevntrisr3Spec> {
        RisW::new(self, 0)
    }
}
#[doc = "desc PEVNTRISR3\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntrisr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntrisr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pevntrisr3Spec;
impl crate::RegisterSpec for Pevntrisr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pevntrisr3::R`](R) reader structure"]
impl crate::Readable for Pevntrisr3Spec {}
#[doc = "`write(|w| ..)` method takes [`pevntrisr3::W`](W) writer structure"]
impl crate::Writable for Pevntrisr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PEVNTRISR3 to value 0"]
impl crate::Resettable for Pevntrisr3Spec {}
