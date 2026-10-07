#[doc = "Register `PDWKES` reader"]
pub type R = crate::R<PdwkesSpec>;
#[doc = "Register `PDWKES` writer"]
pub type W = crate::W<PdwkesSpec>;
#[doc = "Field `WK0EGS` reader - desc WK0EGS"]
pub type Wk0egsR = crate::BitReader;
#[doc = "Field `WK0EGS` writer - desc WK0EGS"]
pub type Wk0egsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WK1EGS` reader - desc WK1EGS"]
pub type Wk1egsR = crate::BitReader;
#[doc = "Field `WK1EGS` writer - desc WK1EGS"]
pub type Wk1egsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WK2EGS` reader - desc WK2EGS"]
pub type Wk2egsR = crate::BitReader;
#[doc = "Field `WK2EGS` writer - desc WK2EGS"]
pub type Wk2egsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WK3EGS` reader - desc WK3EGS"]
pub type Wk3egsR = crate::BitReader;
#[doc = "Field `WK3EGS` writer - desc WK3EGS"]
pub type Wk3egsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VD1EGS` reader - desc VD1EGS"]
pub type Vd1egsR = crate::BitReader;
#[doc = "Field `VD1EGS` writer - desc VD1EGS"]
pub type Vd1egsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VD2EGS` reader - desc VD2EGS"]
pub type Vd2egsR = crate::BitReader;
#[doc = "Field `VD2EGS` writer - desc VD2EGS"]
pub type Vd2egsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIEGS` reader - desc NMIEGS"]
pub type NmiegsR = crate::BitReader;
#[doc = "Field `NMIEGS` writer - desc NMIEGS"]
pub type NmiegsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc WK0EGS"]
    #[inline(always)]
    pub fn wk0egs(&self) -> Wk0egsR {
        Wk0egsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc WK1EGS"]
    #[inline(always)]
    pub fn wk1egs(&self) -> Wk1egsR {
        Wk1egsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc WK2EGS"]
    #[inline(always)]
    pub fn wk2egs(&self) -> Wk2egsR {
        Wk2egsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc WK3EGS"]
    #[inline(always)]
    pub fn wk3egs(&self) -> Wk3egsR {
        Wk3egsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc VD1EGS"]
    #[inline(always)]
    pub fn vd1egs(&self) -> Vd1egsR {
        Vd1egsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc VD2EGS"]
    #[inline(always)]
    pub fn vd2egs(&self) -> Vd2egsR {
        Vd2egsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc NMIEGS"]
    #[inline(always)]
    pub fn nmiegs(&self) -> NmiegsR {
        NmiegsR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc WK0EGS"]
    #[inline(always)]
    pub fn wk0egs(&mut self) -> Wk0egsW<'_, PdwkesSpec> {
        Wk0egsW::new(self, 0)
    }
    #[doc = "Bit 1 - desc WK1EGS"]
    #[inline(always)]
    pub fn wk1egs(&mut self) -> Wk1egsW<'_, PdwkesSpec> {
        Wk1egsW::new(self, 1)
    }
    #[doc = "Bit 2 - desc WK2EGS"]
    #[inline(always)]
    pub fn wk2egs(&mut self) -> Wk2egsW<'_, PdwkesSpec> {
        Wk2egsW::new(self, 2)
    }
    #[doc = "Bit 3 - desc WK3EGS"]
    #[inline(always)]
    pub fn wk3egs(&mut self) -> Wk3egsW<'_, PdwkesSpec> {
        Wk3egsW::new(self, 3)
    }
    #[doc = "Bit 4 - desc VD1EGS"]
    #[inline(always)]
    pub fn vd1egs(&mut self) -> Vd1egsW<'_, PdwkesSpec> {
        Vd1egsW::new(self, 4)
    }
    #[doc = "Bit 5 - desc VD2EGS"]
    #[inline(always)]
    pub fn vd2egs(&mut self) -> Vd2egsW<'_, PdwkesSpec> {
        Vd2egsW::new(self, 5)
    }
    #[doc = "Bit 6 - desc NMIEGS"]
    #[inline(always)]
    pub fn nmiegs(&mut self) -> NmiegsW<'_, PdwkesSpec> {
        NmiegsW::new(self, 6)
    }
}
#[doc = "desc PDWKES\n\nYou can [`read`](crate::Reg::read) this register and get [`pdwkes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdwkes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdwkesSpec;
impl crate::RegisterSpec for PdwkesSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pdwkes::R`](R) reader structure"]
impl crate::Readable for PdwkesSpec {}
#[doc = "`write(|w| ..)` method takes [`pdwkes::W`](W) writer structure"]
impl crate::Writable for PdwkesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDWKES to value 0"]
impl crate::Resettable for PdwkesSpec {}
