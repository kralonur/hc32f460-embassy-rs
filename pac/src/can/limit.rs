#[doc = "Register `LIMIT` reader"]
pub type R = crate::R<LimitSpec>;
#[doc = "Register `LIMIT` writer"]
pub type W = crate::W<LimitSpec>;
#[doc = "Field `EWL` reader - desc EWL"]
pub type EwlR = crate::FieldReader;
#[doc = "Field `EWL` writer - desc EWL"]
pub type EwlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFWL` reader - desc AFWL"]
pub type AfwlR = crate::FieldReader;
#[doc = "Field `AFWL` writer - desc AFWL"]
pub type AfwlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc EWL"]
    #[inline(always)]
    pub fn ewl(&self) -> EwlR {
        EwlR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - desc AFWL"]
    #[inline(always)]
    pub fn afwl(&self) -> AfwlR {
        AfwlR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc EWL"]
    #[inline(always)]
    pub fn ewl(&mut self) -> EwlW<'_, LimitSpec> {
        EwlW::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc AFWL"]
    #[inline(always)]
    pub fn afwl(&mut self) -> AfwlW<'_, LimitSpec> {
        AfwlW::new(self, 4)
    }
}
#[doc = "desc LIMIT\n\nYou can [`read`](crate::Reg::read) this register and get [`limit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LimitSpec;
impl crate::RegisterSpec for LimitSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`limit::R`](R) reader structure"]
impl crate::Readable for LimitSpec {}
#[doc = "`write(|w| ..)` method takes [`limit::W`](W) writer structure"]
impl crate::Writable for LimitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LIMIT to value 0x1b"]
impl crate::Resettable for LimitSpec {
    const RESET_VALUE: u8 = 0x1b;
}
