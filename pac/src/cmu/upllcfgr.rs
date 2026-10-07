#[doc = "Register `UPLLCFGR` reader"]
pub type R = crate::R<UpllcfgrSpec>;
#[doc = "Register `UPLLCFGR` writer"]
pub type W = crate::W<UpllcfgrSpec>;
#[doc = "Field `UPLLM` reader - desc UPLLM"]
pub type UpllmR = crate::FieldReader;
#[doc = "Field `UPLLM` writer - desc UPLLM"]
pub type UpllmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `UPLLN` reader - desc UPLLN"]
pub type UpllnR = crate::FieldReader<u16>;
#[doc = "Field `UPLLN` writer - desc UPLLN"]
pub type UpllnW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `UPLLR` reader - desc UPLLR"]
pub type UpllrR = crate::FieldReader;
#[doc = "Field `UPLLR` writer - desc UPLLR"]
pub type UpllrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UPLLQ` reader - desc UPLLQ"]
pub type UpllqR = crate::FieldReader;
#[doc = "Field `UPLLQ` writer - desc UPLLQ"]
pub type UpllqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UPLLP` reader - desc UPLLP"]
pub type UpllpR = crate::FieldReader;
#[doc = "Field `UPLLP` writer - desc UPLLP"]
pub type UpllpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - desc UPLLM"]
    #[inline(always)]
    pub fn upllm(&self) -> UpllmR {
        UpllmR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:16 - desc UPLLN"]
    #[inline(always)]
    pub fn uplln(&self) -> UpllnR {
        UpllnR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:23 - desc UPLLR"]
    #[inline(always)]
    pub fn upllr(&self) -> UpllrR {
        UpllrR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - desc UPLLQ"]
    #[inline(always)]
    pub fn upllq(&self) -> UpllqR {
        UpllqR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - desc UPLLP"]
    #[inline(always)]
    pub fn upllp(&self) -> UpllpR {
        UpllpR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - desc UPLLM"]
    #[inline(always)]
    pub fn upllm(&mut self) -> UpllmW<'_, UpllcfgrSpec> {
        UpllmW::new(self, 0)
    }
    #[doc = "Bits 8:16 - desc UPLLN"]
    #[inline(always)]
    pub fn uplln(&mut self) -> UpllnW<'_, UpllcfgrSpec> {
        UpllnW::new(self, 8)
    }
    #[doc = "Bits 20:23 - desc UPLLR"]
    #[inline(always)]
    pub fn upllr(&mut self) -> UpllrW<'_, UpllcfgrSpec> {
        UpllrW::new(self, 20)
    }
    #[doc = "Bits 24:27 - desc UPLLQ"]
    #[inline(always)]
    pub fn upllq(&mut self) -> UpllqW<'_, UpllcfgrSpec> {
        UpllqW::new(self, 24)
    }
    #[doc = "Bits 28:31 - desc UPLLP"]
    #[inline(always)]
    pub fn upllp(&mut self) -> UpllpW<'_, UpllcfgrSpec> {
        UpllpW::new(self, 28)
    }
}
#[doc = "desc UPLLCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`upllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`upllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UpllcfgrSpec;
impl crate::RegisterSpec for UpllcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`upllcfgr::R`](R) reader structure"]
impl crate::Readable for UpllcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`upllcfgr::W`](W) writer structure"]
impl crate::Writable for UpllcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UPLLCFGR to value 0x1110_1300"]
impl crate::Resettable for UpllcfgrSpec {
    const RESET_VALUE: u32 = 0x1110_1300;
}
