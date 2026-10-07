#[doc = "Register `PEVNTFALR2` reader"]
pub type R = crate::R<Pevntfalr2Spec>;
#[doc = "Register `PEVNTFALR2` writer"]
pub type W = crate::W<Pevntfalr2Spec>;
#[doc = "Field `FAL` reader - desc FAL"]
pub type FalR = crate::FieldReader<u16>;
#[doc = "Field `FAL` writer - desc FAL"]
pub type FalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc FAL"]
    #[inline(always)]
    pub fn fal(&self) -> FalR {
        FalR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc FAL"]
    #[inline(always)]
    pub fn fal(&mut self) -> FalW<'_, Pevntfalr2Spec> {
        FalW::new(self, 0)
    }
}
#[doc = "desc PEVNTFALR2\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntfalr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntfalr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pevntfalr2Spec;
impl crate::RegisterSpec for Pevntfalr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pevntfalr2::R`](R) reader structure"]
impl crate::Readable for Pevntfalr2Spec {}
#[doc = "`write(|w| ..)` method takes [`pevntfalr2::W`](W) writer structure"]
impl crate::Writable for Pevntfalr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PEVNTFALR2 to value 0"]
impl crate::Resettable for Pevntfalr2Spec {}
