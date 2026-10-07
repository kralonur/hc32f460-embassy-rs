#[doc = "Register `LVR` reader"]
pub type R = crate::R<LvrSpec>;
#[doc = "Register `LVR` writer"]
pub type W = crate::W<LvrSpec>;
#[doc = "Field `LVR` reader - desc LVR"]
pub type LvrR = crate::FieldReader<u16>;
#[doc = "Field `LVR` writer - desc LVR"]
pub type LvrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc LVR"]
    #[inline(always)]
    pub fn lvr(&self) -> LvrR {
        LvrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc LVR"]
    #[inline(always)]
    pub fn lvr(&mut self) -> LvrW<'_, LvrSpec> {
        LvrW::new(self, 0)
    }
}
#[doc = "desc LVR\n\nYou can [`read`](crate::Reg::read) this register and get [`lvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvrSpec;
impl crate::RegisterSpec for LvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lvr::R`](R) reader structure"]
impl crate::Readable for LvrSpec {}
#[doc = "`write(|w| ..)` method takes [`lvr::W`](W) writer structure"]
impl crate::Writable for LvrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LVR to value 0"]
impl crate::Resettable for LvrSpec {}
