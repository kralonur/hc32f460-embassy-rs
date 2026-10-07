#[doc = "Register `DTUBR` reader"]
pub type R = crate::R<DtubrSpec>;
#[doc = "Register `DTUBR` writer"]
pub type W = crate::W<DtubrSpec>;
#[doc = "Field `DTUB` reader - desc DTUB"]
pub type DtubR = crate::FieldReader<u16>;
#[doc = "Field `DTUB` writer - desc DTUB"]
pub type DtubW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc DTUB"]
    #[inline(always)]
    pub fn dtub(&self) -> DtubR {
        DtubR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc DTUB"]
    #[inline(always)]
    pub fn dtub(&mut self) -> DtubW<'_, DtubrSpec> {
        DtubW::new(self, 0)
    }
}
#[doc = "desc DTUBR\n\nYou can [`read`](crate::Reg::read) this register and get [`dtubr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtubr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtubrSpec;
impl crate::RegisterSpec for DtubrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtubr::R`](R) reader structure"]
impl crate::Readable for DtubrSpec {}
#[doc = "`write(|w| ..)` method takes [`dtubr::W`](W) writer structure"]
impl crate::Writable for DtubrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTUBR to value 0xffff"]
impl crate::Resettable for DtubrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
