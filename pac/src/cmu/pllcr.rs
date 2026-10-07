#[doc = "Register `PLLCR` reader"]
pub type R = crate::R<PllcrSpec>;
#[doc = "Register `PLLCR` writer"]
pub type W = crate::W<PllcrSpec>;
#[doc = "Field `MPLLOFF` reader - desc MPLLOFF"]
pub type MplloffR = crate::BitReader;
#[doc = "Field `MPLLOFF` writer - desc MPLLOFF"]
pub type MplloffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc MPLLOFF"]
    #[inline(always)]
    pub fn mplloff(&self) -> MplloffR {
        MplloffR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc MPLLOFF"]
    #[inline(always)]
    pub fn mplloff(&mut self) -> MplloffW<'_, PllcrSpec> {
        MplloffW::new(self, 0)
    }
}
#[doc = "desc PLLCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllcrSpec;
impl crate::RegisterSpec for PllcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pllcr::R`](R) reader structure"]
impl crate::Readable for PllcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pllcr::W`](W) writer structure"]
impl crate::Writable for PllcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLCR to value 0x01"]
impl crate::Resettable for PllcrSpec {
    const RESET_VALUE: u8 = 0x01;
}
