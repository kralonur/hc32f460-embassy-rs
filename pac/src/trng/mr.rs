#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `LOAD` reader - desc LOAD"]
pub type LoadR = crate::BitReader;
#[doc = "Field `LOAD` writer - desc LOAD"]
pub type LoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CntR = crate::FieldReader;
#[doc = "Field `CNT` writer - desc CNT"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - desc LOAD"]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc LOAD"]
    #[inline(always)]
    pub fn load(&mut self) -> LoadW<'_, MrSpec> {
        LoadW::new(self, 0)
    }
    #[doc = "Bits 2:4 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, MrSpec> {
        CntW::new(self, 2)
    }
}
#[doc = "desc MR\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MR to value 0x12"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0x12;
}
