#[doc = "Register `SBT` reader"]
pub type R = crate::R<SbtSpec>;
#[doc = "Register `SBT` writer"]
pub type W = crate::W<SbtSpec>;
#[doc = "Field `S_SEG_1` reader - desc S_SEG_1"]
pub type SSeg1R = crate::FieldReader;
#[doc = "Field `S_SEG_1` writer - desc S_SEG_1"]
pub type SSeg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `S_SEG_2` reader - desc S_SEG_2"]
pub type SSeg2R = crate::FieldReader;
#[doc = "Field `S_SEG_2` writer - desc S_SEG_2"]
pub type SSeg2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `S_SJW` reader - desc S_SJW"]
pub type SSjwR = crate::FieldReader;
#[doc = "Field `S_SJW` writer - desc S_SJW"]
pub type SSjwW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `S_PRESC` reader - desc S_PRESC"]
pub type SPrescR = crate::FieldReader;
#[doc = "Field `S_PRESC` writer - desc S_PRESC"]
pub type SPrescW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - desc S_SEG_1"]
    #[inline(always)]
    pub fn s_seg_1(&self) -> SSeg1R {
        SSeg1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - desc S_SEG_2"]
    #[inline(always)]
    pub fn s_seg_2(&self) -> SSeg2R {
        SSeg2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - desc S_SJW"]
    #[inline(always)]
    pub fn s_sjw(&self) -> SSjwR {
        SSjwR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - desc S_PRESC"]
    #[inline(always)]
    pub fn s_presc(&self) -> SPrescR {
        SPrescR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc S_SEG_1"]
    #[inline(always)]
    pub fn s_seg_1(&mut self) -> SSeg1W<'_, SbtSpec> {
        SSeg1W::new(self, 0)
    }
    #[doc = "Bits 8:14 - desc S_SEG_2"]
    #[inline(always)]
    pub fn s_seg_2(&mut self) -> SSeg2W<'_, SbtSpec> {
        SSeg2W::new(self, 8)
    }
    #[doc = "Bits 16:22 - desc S_SJW"]
    #[inline(always)]
    pub fn s_sjw(&mut self) -> SSjwW<'_, SbtSpec> {
        SSjwW::new(self, 16)
    }
    #[doc = "Bits 24:31 - desc S_PRESC"]
    #[inline(always)]
    pub fn s_presc(&mut self) -> SPrescW<'_, SbtSpec> {
        SPrescW::new(self, 24)
    }
}
#[doc = "desc SBT\n\nYou can [`read`](crate::Reg::read) this register and get [`sbt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SbtSpec;
impl crate::RegisterSpec for SbtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbt::R`](R) reader structure"]
impl crate::Readable for SbtSpec {}
#[doc = "`write(|w| ..)` method takes [`sbt::W`](W) writer structure"]
impl crate::Writable for SbtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SBT to value 0x0102_0203"]
impl crate::Resettable for SbtSpec {
    const RESET_VALUE: u32 = 0x0102_0203;
}
