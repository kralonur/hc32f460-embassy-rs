#[doc = "Register `CNTER` reader"]
pub type R = crate::R<CnterSpec>;
#[doc = "Register `CNTER` writer"]
pub type W = crate::W<CnterSpec>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - desc CNT"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, CnterSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "desc CNTER\n\nYou can [`read`](crate::Reg::read) this register and get [`cnter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CnterSpec;
impl crate::RegisterSpec for CnterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnter::R`](R) reader structure"]
impl crate::Readable for CnterSpec {}
#[doc = "`write(|w| ..)` method takes [`cnter::W`](W) writer structure"]
impl crate::Writable for CnterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNTER to value 0"]
impl crate::Resettable for CnterSpec {}
