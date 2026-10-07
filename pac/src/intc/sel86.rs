#[doc = "Register `SEL86` reader"]
pub type R = crate::R<Sel86Spec>;
#[doc = "Register `SEL86` writer"]
pub type W = crate::W<Sel86Spec>;
#[doc = "Field `INTSEL` reader - desc INTSEL"]
pub type IntselR = crate::FieldReader<u16>;
#[doc = "Field `INTSEL` writer - desc INTSEL"]
pub type IntselW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - desc INTSEL"]
    #[inline(always)]
    pub fn intsel(&self) -> IntselR {
        IntselR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - desc INTSEL"]
    #[inline(always)]
    pub fn intsel(&mut self) -> IntselW<'_, Sel86Spec> {
        IntselW::new(self, 0)
    }
}
#[doc = "desc SEL86\n\nYou can [`read`](crate::Reg::read) this register and get [`sel86::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel86::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sel86Spec;
impl crate::RegisterSpec for Sel86Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel86::R`](R) reader structure"]
impl crate::Readable for Sel86Spec {}
#[doc = "`write(|w| ..)` method takes [`sel86::W`](W) writer structure"]
impl crate::Writable for Sel86Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEL86 to value 0x01ff"]
impl crate::Resettable for Sel86Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
