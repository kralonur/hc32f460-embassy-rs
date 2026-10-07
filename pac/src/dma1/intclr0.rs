#[doc = "Register `INTCLR0` writer"]
pub type W = crate::W<Intclr0Spec>;
#[doc = "Field `CLRTRNERR` writer - desc CLRTRNERR"]
pub type ClrtrnerrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLRREQERR` writer - desc CLRREQERR"]
pub type ClrreqerrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - desc CLRTRNERR"]
    #[inline(always)]
    pub fn clrtrnerr(&mut self) -> ClrtrnerrW<'_, Intclr0Spec> {
        ClrtrnerrW::new(self, 0)
    }
    #[doc = "Bits 16:19 - desc CLRREQERR"]
    #[inline(always)]
    pub fn clrreqerr(&mut self) -> ClrreqerrW<'_, Intclr0Spec> {
        ClrreqerrW::new(self, 16)
    }
}
#[doc = "desc INTCLR0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intclr0Spec;
impl crate::RegisterSpec for Intclr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intclr0::W`](W) writer structure"]
impl crate::Writable for Intclr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTCLR0 to value 0"]
impl crate::Resettable for Intclr0Spec {}
