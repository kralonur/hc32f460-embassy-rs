#[doc = "Register `GCCTL` reader"]
pub type R = crate::R<GcctlSpec>;
#[doc = "Register `GCCTL` writer"]
pub type W = crate::W<GcctlSpec>;
#[doc = "Field `STPPCLK` reader - desc STPPCLK"]
pub type StppclkR = crate::BitReader;
#[doc = "Field `STPPCLK` writer - desc STPPCLK"]
pub type StppclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GATEHCLK` reader - desc GATEHCLK"]
pub type GatehclkR = crate::BitReader;
#[doc = "Field `GATEHCLK` writer - desc GATEHCLK"]
pub type GatehclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc STPPCLK"]
    #[inline(always)]
    pub fn stppclk(&self) -> StppclkR {
        StppclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc GATEHCLK"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GatehclkR {
        GatehclkR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc STPPCLK"]
    #[inline(always)]
    pub fn stppclk(&mut self) -> StppclkW<'_, GcctlSpec> {
        StppclkW::new(self, 0)
    }
    #[doc = "Bit 1 - desc GATEHCLK"]
    #[inline(always)]
    pub fn gatehclk(&mut self) -> GatehclkW<'_, GcctlSpec> {
        GatehclkW::new(self, 1)
    }
}
#[doc = "desc GCCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`gcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcctlSpec;
impl crate::RegisterSpec for GcctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcctl::R`](R) reader structure"]
impl crate::Readable for GcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`gcctl::W`](W) writer structure"]
impl crate::Writable for GcctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCCTL to value 0"]
impl crate::Resettable for GcctlSpec {}
