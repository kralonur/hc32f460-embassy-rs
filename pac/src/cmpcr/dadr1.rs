#[doc = "Register `DADR1` reader"]
pub type R = crate::R<Dadr1Spec>;
#[doc = "Register `DADR1` writer"]
pub type W = crate::W<Dadr1Spec>;
#[doc = "Field `DATA` reader - desc DATA"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - desc DATA"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - desc DATA"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc DATA"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Dadr1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "desc DADR1\n\nYou can [`read`](crate::Reg::read) this register and get [`dadr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dadr1Spec;
impl crate::RegisterSpec for Dadr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dadr1::R`](R) reader structure"]
impl crate::Readable for Dadr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dadr1::W`](W) writer structure"]
impl crate::Writable for Dadr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DADR1 to value 0"]
impl crate::Resettable for Dadr1Spec {}
