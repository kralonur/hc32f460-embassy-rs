#[doc = "Register `PERAR` reader"]
pub type R = crate::R<PerarSpec>;
#[doc = "Register `PERAR` writer"]
pub type W = crate::W<PerarSpec>;
#[doc = "Field `PER` reader - desc PER"]
pub type PerR = crate::FieldReader<u16>;
#[doc = "Field `PER` writer - desc PER"]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc PER"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc PER"]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<'_, PerarSpec> {
        PerW::new(self, 0)
    }
}
#[doc = "desc PERAR\n\nYou can [`read`](crate::Reg::read) this register and get [`perar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerarSpec;
impl crate::RegisterSpec for PerarSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`perar::R`](R) reader structure"]
impl crate::Readable for PerarSpec {}
#[doc = "`write(|w| ..)` method takes [`perar::W`](W) writer structure"]
impl crate::Writable for PerarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERAR to value 0xffff"]
impl crate::Resettable for PerarSpec {
    const RESET_VALUE: u16 = 0xffff;
}
