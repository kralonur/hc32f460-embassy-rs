#[doc = "Register `BRR` reader"]
pub type R = crate::R<BrrSpec>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `DIV_FRACTION` reader - desc DIV_FRACTION"]
pub type DivFractionR = crate::FieldReader;
#[doc = "Field `DIV_FRACTION` writer - desc DIV_FRACTION"]
pub type DivFractionW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIV_INTEGER` reader - desc DIV_INTEGER"]
pub type DivIntegerR = crate::FieldReader;
#[doc = "Field `DIV_INTEGER` writer - desc DIV_INTEGER"]
pub type DivIntegerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - desc DIV_FRACTION"]
    #[inline(always)]
    pub fn div_fraction(&self) -> DivFractionR {
        DivFractionR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - desc DIV_INTEGER"]
    #[inline(always)]
    pub fn div_integer(&self) -> DivIntegerR {
        DivIntegerR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - desc DIV_FRACTION"]
    #[inline(always)]
    pub fn div_fraction(&mut self) -> DivFractionW<'_, BrrSpec> {
        DivFractionW::new(self, 0)
    }
    #[doc = "Bits 8:15 - desc DIV_INTEGER"]
    #[inline(always)]
    pub fn div_integer(&mut self) -> DivIntegerW<'_, BrrSpec> {
        DivIntegerW::new(self, 8)
    }
}
#[doc = "desc BRR\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BrrSpec {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR to value 0xffff"]
impl crate::Resettable for BrrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
