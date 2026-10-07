#[doc = "Register `PERICKSEL` reader"]
pub type R = crate::R<PerickselSpec>;
#[doc = "Register `PERICKSEL` writer"]
pub type W = crate::W<PerickselSpec>;
#[doc = "Field `PERICKSEL` reader - desc PERICKSEL"]
pub type PerickselR = crate::FieldReader;
#[doc = "Field `PERICKSEL` writer - desc PERICKSEL"]
pub type PerickselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc PERICKSEL"]
    #[inline(always)]
    pub fn pericksel(&self) -> PerickselR {
        PerickselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc PERICKSEL"]
    #[inline(always)]
    pub fn pericksel(&mut self) -> PerickselW<'_, PerickselSpec> {
        PerickselW::new(self, 0)
    }
}
#[doc = "desc PERICKSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`pericksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pericksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerickselSpec;
impl crate::RegisterSpec for PerickselSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pericksel::R`](R) reader structure"]
impl crate::Readable for PerickselSpec {}
#[doc = "`write(|w| ..)` method takes [`pericksel::W`](W) writer structure"]
impl crate::Writable for PerickselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERICKSEL to value 0"]
impl crate::Resettable for PerickselSpec {}
