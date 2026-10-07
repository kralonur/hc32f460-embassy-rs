#[doc = "Register `PEVNTORR3` reader"]
pub type R = crate::R<Pevntorr3Spec>;
#[doc = "Register `PEVNTORR3` writer"]
pub type W = crate::W<Pevntorr3Spec>;
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
    pub fn por(&mut self) -> PorW<'_, Pevntorr3Spec> {
        PorW::new(self, 0)
    }
}
#[doc = "desc PEVNTORR3\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntorr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntorr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pevntorr3Spec;
impl crate::RegisterSpec for Pevntorr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pevntorr3::R`](R) reader structure"]
impl crate::Readable for Pevntorr3Spec {}
#[doc = "`write(|w| ..)` method takes [`pevntorr3::W`](W) writer structure"]
impl crate::Writable for Pevntorr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PEVNTORR3 to value 0"]
impl crate::Resettable for Pevntorr3Spec {}
