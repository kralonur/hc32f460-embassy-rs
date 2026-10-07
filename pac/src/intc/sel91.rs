#[doc = "Register `SEL91` reader"]
pub type R = crate::R<Sel91Spec>;
#[doc = "Register `SEL91` writer"]
pub type W = crate::W<Sel91Spec>;
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
    pub fn intsel(&mut self) -> IntselW<'_, Sel91Spec> {
        IntselW::new(self, 0)
    }
}
#[doc = "desc SEL91\n\nYou can [`read`](crate::Reg::read) this register and get [`sel91::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel91::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sel91Spec;
impl crate::RegisterSpec for Sel91Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel91::R`](R) reader structure"]
impl crate::Readable for Sel91Spec {}
#[doc = "`write(|w| ..)` method takes [`sel91::W`](W) writer structure"]
impl crate::Writable for Sel91Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEL91 to value 0x01ff"]
impl crate::Resettable for Sel91Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
