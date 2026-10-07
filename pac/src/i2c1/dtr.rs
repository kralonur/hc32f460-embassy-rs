#[doc = "Register `DTR` writer"]
pub type W = crate::W<DtrSpec>;
#[doc = "Field `DT` writer - desc DT"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - desc DT"]
    #[inline(always)]
    pub fn dt(&mut self) -> DtW<'_, DtrSpec> {
        DtW::new(self, 0)
    }
}
#[doc = "desc DTR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtrSpec;
impl crate::RegisterSpec for DtrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`dtr::W`](W) writer structure"]
impl crate::Writable for DtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTR to value 0xff"]
impl crate::Resettable for DtrSpec {
    const RESET_VALUE: u8 = 0xff;
}
