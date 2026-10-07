#[doc = "Register `COMTRG1` reader"]
pub type R = crate::R<Comtrg1Spec>;
#[doc = "Register `COMTRG1` writer"]
pub type W = crate::W<Comtrg1Spec>;
#[doc = "Field `COMTRG` reader - desc COMTRG"]
pub type ComtrgR = crate::FieldReader<u16>;
#[doc = "Field `COMTRG` writer - desc COMTRG"]
pub type ComtrgW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - desc COMTRG"]
    #[inline(always)]
    pub fn comtrg(&self) -> ComtrgR {
        ComtrgR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - desc COMTRG"]
    #[inline(always)]
    pub fn comtrg(&mut self) -> ComtrgW<'_, Comtrg1Spec> {
        ComtrgW::new(self, 0)
    }
}
#[doc = "desc COMTRG1\n\nYou can [`read`](crate::Reg::read) this register and get [`comtrg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comtrg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comtrg1Spec;
impl crate::RegisterSpec for Comtrg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comtrg1::R`](R) reader structure"]
impl crate::Readable for Comtrg1Spec {}
#[doc = "`write(|w| ..)` method takes [`comtrg1::W`](W) writer structure"]
impl crate::Writable for Comtrg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMTRG1 to value 0x01ff"]
impl crate::Resettable for Comtrg1Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
