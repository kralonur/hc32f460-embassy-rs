#[doc = "Register `SOE` reader"]
pub type R = crate::R<SoeSpec>;
#[doc = "Register `SOE` writer"]
pub type W = crate::W<SoeSpec>;
#[doc = "Field `SOE` reader - desc SOE"]
pub type SoeR = crate::BitReader;
#[doc = "Field `SOE` writer - desc SOE"]
pub type SoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SOE"]
    #[inline(always)]
    pub fn soe(&self) -> SoeR {
        SoeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SOE"]
    #[inline(always)]
    pub fn soe(&mut self) -> SoeW<'_, SoeSpec> {
        SoeW::new(self, 0)
    }
}
#[doc = "desc SOE\n\nYou can [`read`](crate::Reg::read) this register and get [`soe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoeSpec;
impl crate::RegisterSpec for SoeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soe::R`](R) reader structure"]
impl crate::Readable for SoeSpec {}
#[doc = "`write(|w| ..)` method takes [`soe::W`](W) writer structure"]
impl crate::Writable for SoeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOE to value 0"]
impl crate::Resettable for SoeSpec {}
