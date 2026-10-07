#[doc = "Register `PLLCFGR` reader"]
pub type R = crate::R<PllcfgrSpec>;
#[doc = "Register `PLLCFGR` writer"]
pub type W = crate::W<PllcfgrSpec>;
#[doc = "Field `MPLLM` reader - desc MPLLM"]
pub type MpllmR = crate::FieldReader;
#[doc = "Field `MPLLM` writer - desc MPLLM"]
pub type MpllmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PLLSRC` reader - desc PLLSRC"]
pub type PllsrcR = crate::BitReader;
#[doc = "Field `PLLSRC` writer - desc PLLSRC"]
pub type PllsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPLLN` reader - desc MPLLN"]
pub type MpllnR = crate::FieldReader<u16>;
#[doc = "Field `MPLLN` writer - desc MPLLN"]
pub type MpllnW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `MPLLR` reader - desc MPLLR"]
pub type MpllrR = crate::FieldReader;
#[doc = "Field `MPLLR` writer - desc MPLLR"]
pub type MpllrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MPLLQ` reader - desc MPLLQ"]
pub type MpllqR = crate::FieldReader;
#[doc = "Field `MPLLQ` writer - desc MPLLQ"]
pub type MpllqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MPLLP` reader - desc MPLLP"]
pub type MpllpR = crate::FieldReader;
#[doc = "Field `MPLLP` writer - desc MPLLP"]
pub type MpllpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - desc MPLLM"]
    #[inline(always)]
    pub fn mpllm(&self) -> MpllmR {
        MpllmR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - desc PLLSRC"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PllsrcR {
        PllsrcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:16 - desc MPLLN"]
    #[inline(always)]
    pub fn mplln(&self) -> MpllnR {
        MpllnR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:23 - desc MPLLR"]
    #[inline(always)]
    pub fn mpllr(&self) -> MpllrR {
        MpllrR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - desc MPLLQ"]
    #[inline(always)]
    pub fn mpllq(&self) -> MpllqR {
        MpllqR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - desc MPLLP"]
    #[inline(always)]
    pub fn mpllp(&self) -> MpllpR {
        MpllpR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - desc MPLLM"]
    #[inline(always)]
    pub fn mpllm(&mut self) -> MpllmW<'_, PllcfgrSpec> {
        MpllmW::new(self, 0)
    }
    #[doc = "Bit 7 - desc PLLSRC"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PllsrcW<'_, PllcfgrSpec> {
        PllsrcW::new(self, 7)
    }
    #[doc = "Bits 8:16 - desc MPLLN"]
    #[inline(always)]
    pub fn mplln(&mut self) -> MpllnW<'_, PllcfgrSpec> {
        MpllnW::new(self, 8)
    }
    #[doc = "Bits 20:23 - desc MPLLR"]
    #[inline(always)]
    pub fn mpllr(&mut self) -> MpllrW<'_, PllcfgrSpec> {
        MpllrW::new(self, 20)
    }
    #[doc = "Bits 24:27 - desc MPLLQ"]
    #[inline(always)]
    pub fn mpllq(&mut self) -> MpllqW<'_, PllcfgrSpec> {
        MpllqW::new(self, 24)
    }
    #[doc = "Bits 28:31 - desc MPLLP"]
    #[inline(always)]
    pub fn mpllp(&mut self) -> MpllpW<'_, PllcfgrSpec> {
        MpllpW::new(self, 28)
    }
}
#[doc = "desc PLLCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllcfgrSpec;
impl crate::RegisterSpec for PllcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfgr::R`](R) reader structure"]
impl crate::Readable for PllcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure"]
impl crate::Writable for PllcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x1110_1300"]
impl crate::Resettable for PllcfgrSpec {
    const RESET_VALUE: u32 = 0x1110_1300;
}
