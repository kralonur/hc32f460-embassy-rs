#[doc = "Register `MON` reader"]
pub type R = crate::R<MonSpec>;
#[doc = "Register `MON` writer"]
pub type W = crate::W<MonSpec>;
#[doc = "Field `MON` reader - desc MON"]
pub type MonR = crate::FieldReader;
#[doc = "Field `MON` writer - desc MON"]
pub type MonW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - desc MON"]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - desc MON"]
    #[inline(always)]
    pub fn mon(&mut self) -> MonW<'_, MonSpec> {
        MonW::new(self, 0)
    }
}
#[doc = "desc MON\n\nYou can [`read`](crate::Reg::read) this register and get [`mon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MonSpec;
impl crate::RegisterSpec for MonSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mon::R`](R) reader structure"]
impl crate::Readable for MonSpec {}
#[doc = "`write(|w| ..)` method takes [`mon::W`](W) writer structure"]
impl crate::Writable for MonSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MON to value 0"]
impl crate::Resettable for MonSpec {}
