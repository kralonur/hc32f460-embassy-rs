#[doc = "Register `SDIOC_SYCTLREG` reader"]
pub type R = crate::R<SdiocSyctlregSpec>;
#[doc = "Register `SDIOC_SYCTLREG` writer"]
pub type W = crate::W<SdiocSyctlregSpec>;
#[doc = "Field `SELMMC1` reader - desc SELMMC1"]
pub type Selmmc1R = crate::BitReader;
#[doc = "Field `SELMMC1` writer - desc SELMMC1"]
pub type Selmmc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELMMC2` reader - desc SELMMC2"]
pub type Selmmc2R = crate::BitReader;
#[doc = "Field `SELMMC2` writer - desc SELMMC2"]
pub type Selmmc2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - desc SELMMC1"]
    #[inline(always)]
    pub fn selmmc1(&self) -> Selmmc1R {
        Selmmc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SELMMC2"]
    #[inline(always)]
    pub fn selmmc2(&self) -> Selmmc2R {
        Selmmc2R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc SELMMC1"]
    #[inline(always)]
    pub fn selmmc1(&mut self) -> Selmmc1W<'_, SdiocSyctlregSpec> {
        Selmmc1W::new(self, 1)
    }
    #[doc = "Bit 3 - desc SELMMC2"]
    #[inline(always)]
    pub fn selmmc2(&mut self) -> Selmmc2W<'_, SdiocSyctlregSpec> {
        Selmmc2W::new(self, 3)
    }
}
#[doc = "desc SDIOC_SYCTLREG\n\nYou can [`read`](crate::Reg::read) this register and get [`sdioc_syctlreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdioc_syctlreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdiocSyctlregSpec;
impl crate::RegisterSpec for SdiocSyctlregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdioc_syctlreg::R`](R) reader structure"]
impl crate::Readable for SdiocSyctlregSpec {}
#[doc = "`write(|w| ..)` method takes [`sdioc_syctlreg::W`](W) writer structure"]
impl crate::Writable for SdiocSyctlregSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDIOC_SYCTLREG to value 0"]
impl crate::Resettable for SdiocSyctlregSpec {}
