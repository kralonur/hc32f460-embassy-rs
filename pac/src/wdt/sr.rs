#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `UDF` reader - desc UDF"]
pub type UdfR = crate::BitReader;
#[doc = "Field `UDF` writer - desc UDF"]
pub type UdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF` reader - desc REF"]
pub type RefR = crate::BitReader;
#[doc = "Field `REF` writer - desc REF"]
pub type RefW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - desc UDF"]
    #[inline(always)]
    pub fn udf(&self) -> UdfR {
        UdfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc REF"]
    #[inline(always)]
    pub fn ref_(&self) -> RefR {
        RefR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - desc UDF"]
    #[inline(always)]
    pub fn udf(&mut self) -> UdfW<'_, SrSpec> {
        UdfW::new(self, 16)
    }
    #[doc = "Bit 17 - desc REF"]
    #[inline(always)]
    pub fn ref_(&mut self) -> RefW<'_, SrSpec> {
        RefW::new(self, 17)
    }
}
#[doc = "desc SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
