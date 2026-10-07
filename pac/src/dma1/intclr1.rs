#[doc = "Register `INTCLR1` writer"]
pub type W = crate::W<Intclr1Spec>;
#[doc = "Field `CLRTC` writer - desc CLRTC"]
pub type ClrtcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLRBTC` writer - desc CLRBTC"]
pub type ClrbtcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - desc CLRTC"]
    #[inline(always)]
    pub fn clrtc(&mut self) -> ClrtcW<'_, Intclr1Spec> {
        ClrtcW::new(self, 0)
    }
    #[doc = "Bits 16:19 - desc CLRBTC"]
    #[inline(always)]
    pub fn clrbtc(&mut self) -> ClrbtcW<'_, Intclr1Spec> {
        ClrbtcW::new(self, 16)
    }
}
#[doc = "desc INTCLR1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intclr1Spec;
impl crate::RegisterSpec for Intclr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intclr1::W`](W) writer structure"]
impl crate::Writable for Intclr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTCLR1 to value 0"]
impl crate::Resettable for Intclr1Spec {}
