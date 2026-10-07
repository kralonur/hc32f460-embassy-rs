#[doc = "Register `ECSR` reader"]
pub type R = crate::R<EcsrSpec>;
#[doc = "Register `ECSR` writer"]
pub type W = crate::W<EcsrSpec>;
#[doc = "Field `HOLD` reader - desc HOLD"]
pub type HoldR = crate::BitReader;
#[doc = "Field `HOLD` writer - desc HOLD"]
pub type HoldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - desc HOLD"]
    #[inline(always)]
    pub fn hold(&self) -> HoldR {
        HoldR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - desc HOLD"]
    #[inline(always)]
    pub fn hold(&mut self) -> HoldW<'_, EcsrSpec> {
        HoldW::new(self, 7)
    }
}
#[doc = "desc ECSR\n\nYou can [`read`](crate::Reg::read) this register and get [`ecsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcsrSpec;
impl crate::RegisterSpec for EcsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ecsr::R`](R) reader structure"]
impl crate::Readable for EcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ecsr::W`](W) writer structure"]
impl crate::Writable for EcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECSR to value 0"]
impl crate::Resettable for EcsrSpec {}
