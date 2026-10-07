#[doc = "Register `PEVNTRISR4` reader"]
pub type R = crate::R<Pevntrisr4Spec>;
#[doc = "Register `PEVNTRISR4` writer"]
pub type W = crate::W<Pevntrisr4Spec>;
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
    pub fn ris(&mut self) -> RisW<'_, Pevntrisr4Spec> {
        RisW::new(self, 0)
    }
}
#[doc = "desc PEVNTRISR4\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntrisr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntrisr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pevntrisr4Spec;
impl crate::RegisterSpec for Pevntrisr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pevntrisr4::R`](R) reader structure"]
impl crate::Readable for Pevntrisr4Spec {}
#[doc = "`write(|w| ..)` method takes [`pevntrisr4::W`](W) writer structure"]
impl crate::Writable for Pevntrisr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PEVNTRISR4 to value 0"]
impl crate::Resettable for Pevntrisr4Spec {}
