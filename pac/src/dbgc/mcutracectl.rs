#[doc = "Register `MCUTRACECTL` reader"]
pub type R = crate::R<McutracectlSpec>;
#[doc = "Register `MCUTRACECTL` writer"]
pub type W = crate::W<McutracectlSpec>;
#[doc = "Field `TRACEMODE` reader - desc TRACEMODE"]
pub type TracemodeR = crate::FieldReader;
#[doc = "Field `TRACEMODE` writer - desc TRACEMODE"]
pub type TracemodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRACEIOEN` reader - desc TRACEIOEN"]
pub type TraceioenR = crate::BitReader;
#[doc = "Field `TRACEIOEN` writer - desc TRACEIOEN"]
pub type TraceioenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc TRACEMODE"]
    #[inline(always)]
    pub fn tracemode(&self) -> TracemodeR {
        TracemodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - desc TRACEIOEN"]
    #[inline(always)]
    pub fn traceioen(&self) -> TraceioenR {
        TraceioenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc TRACEMODE"]
    #[inline(always)]
    pub fn tracemode(&mut self) -> TracemodeW<'_, McutracectlSpec> {
        TracemodeW::new(self, 0)
    }
    #[doc = "Bit 2 - desc TRACEIOEN"]
    #[inline(always)]
    pub fn traceioen(&mut self) -> TraceioenW<'_, McutracectlSpec> {
        TraceioenW::new(self, 2)
    }
}
#[doc = "desc MCUTRACECTL\n\nYou can [`read`](crate::Reg::read) this register and get [`mcutracectl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcutracectl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McutracectlSpec;
impl crate::RegisterSpec for McutracectlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcutracectl::R`](R) reader structure"]
impl crate::Readable for McutracectlSpec {}
#[doc = "`write(|w| ..)` method takes [`mcutracectl::W`](W) writer structure"]
impl crate::Writable for McutracectlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCUTRACECTL to value 0"]
impl crate::Resettable for McutracectlSpec {}
