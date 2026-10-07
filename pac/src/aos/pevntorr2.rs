#[doc = "Register `PEVNTORR2` reader"]
pub type R = crate::R<Pevntorr2Spec>;
#[doc = "Register `PEVNTORR2` writer"]
pub type W = crate::W<Pevntorr2Spec>;
#[doc = "Field `POR` reader - desc POR"]
pub type PorR = crate::FieldReader<u16>;
#[doc = "Field `POR` writer - desc POR"]
pub type PorW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc POR"]
    #[inline(always)]
    pub fn por(&self) -> PorR {
        PorR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc POR"]
    #[inline(always)]
    pub fn por(&mut self) -> PorW<'_, Pevntorr2Spec> {
        PorW::new(self, 0)
    }
}
#[doc = "desc PEVNTORR2\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntorr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntorr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pevntorr2Spec;
impl crate::RegisterSpec for Pevntorr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pevntorr2::R`](R) reader structure"]
impl crate::Readable for Pevntorr2Spec {}
#[doc = "`write(|w| ..)` method takes [`pevntorr2::W`](W) writer structure"]
impl crate::Writable for Pevntorr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PEVNTORR2 to value 0"]
impl crate::Resettable for Pevntorr2Spec {}
