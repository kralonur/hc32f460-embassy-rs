#[doc = "Register `RSTF0` reader"]
pub type R = crate::R<Rstf0Spec>;
#[doc = "Register `RSTF0` writer"]
pub type W = crate::W<Rstf0Spec>;
#[doc = "Field `PORF` reader - desc PORF"]
pub type PorfR = crate::BitReader;
#[doc = "Field `PORF` writer - desc PORF"]
pub type PorfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINRF` reader - desc PINRF"]
pub type PinrfR = crate::BitReader;
#[doc = "Field `PINRF` writer - desc PINRF"]
pub type PinrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BORF` reader - desc BORF"]
pub type BorfR = crate::BitReader;
#[doc = "Field `BORF` writer - desc BORF"]
pub type BorfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD1RF` reader - desc PVD1RF"]
pub type Pvd1rfR = crate::BitReader;
#[doc = "Field `PVD1RF` writer - desc PVD1RF"]
pub type Pvd1rfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2RF` reader - desc PVD2RF"]
pub type Pvd2rfR = crate::BitReader;
#[doc = "Field `PVD2RF` writer - desc PVD2RF"]
pub type Pvd2rfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDRF` reader - desc WDRF"]
pub type WdrfR = crate::BitReader;
#[doc = "Field `WDRF` writer - desc WDRF"]
pub type WdrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDRF` reader - desc SWDRF"]
pub type SwdrfR = crate::BitReader;
#[doc = "Field `SWDRF` writer - desc SWDRF"]
pub type SwdrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDRF` reader - desc PDRF"]
pub type PdrfR = crate::BitReader;
#[doc = "Field `PDRF` writer - desc PDRF"]
pub type PdrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRF` reader - desc SWRF"]
pub type SwrfR = crate::BitReader;
#[doc = "Field `SWRF` writer - desc SWRF"]
pub type SwrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPUERF` reader - desc MPUERF"]
pub type MpuerfR = crate::BitReader;
#[doc = "Field `MPUERF` writer - desc MPUERF"]
pub type MpuerfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAPERF` reader - desc RAPERF"]
pub type RaperfR = crate::BitReader;
#[doc = "Field `RAPERF` writer - desc RAPERF"]
pub type RaperfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAECRF` reader - desc RAECRF"]
pub type RaecrfR = crate::BitReader;
#[doc = "Field `RAECRF` writer - desc RAECRF"]
pub type RaecrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKFERF` reader - desc CKFERF"]
pub type CkferfR = crate::BitReader;
#[doc = "Field `CKFERF` writer - desc CKFERF"]
pub type CkferfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALERF` reader - desc XTALERF"]
pub type XtalerfR = crate::BitReader;
#[doc = "Field `XTALERF` writer - desc XTALERF"]
pub type XtalerfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTIRF` reader - desc MULTIRF"]
pub type MultirfR = crate::BitReader;
#[doc = "Field `MULTIRF` writer - desc MULTIRF"]
pub type MultirfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRF` reader - desc CLRF"]
pub type ClrfR = crate::BitReader;
#[doc = "Field `CLRF` writer - desc CLRF"]
pub type ClrfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PORF"]
    #[inline(always)]
    pub fn porf(&self) -> PorfR {
        PorfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PINRF"]
    #[inline(always)]
    pub fn pinrf(&self) -> PinrfR {
        PinrfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc BORF"]
    #[inline(always)]
    pub fn borf(&self) -> BorfR {
        BorfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PVD1RF"]
    #[inline(always)]
    pub fn pvd1rf(&self) -> Pvd1rfR {
        Pvd1rfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PVD2RF"]
    #[inline(always)]
    pub fn pvd2rf(&self) -> Pvd2rfR {
        Pvd2rfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc WDRF"]
    #[inline(always)]
    pub fn wdrf(&self) -> WdrfR {
        WdrfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SWDRF"]
    #[inline(always)]
    pub fn swdrf(&self) -> SwdrfR {
        SwdrfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PDRF"]
    #[inline(always)]
    pub fn pdrf(&self) -> PdrfR {
        PdrfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc SWRF"]
    #[inline(always)]
    pub fn swrf(&self) -> SwrfR {
        SwrfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc MPUERF"]
    #[inline(always)]
    pub fn mpuerf(&self) -> MpuerfR {
        MpuerfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc RAPERF"]
    #[inline(always)]
    pub fn raperf(&self) -> RaperfR {
        RaperfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc RAECRF"]
    #[inline(always)]
    pub fn raecrf(&self) -> RaecrfR {
        RaecrfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc CKFERF"]
    #[inline(always)]
    pub fn ckferf(&self) -> CkferfR {
        CkferfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc XTALERF"]
    #[inline(always)]
    pub fn xtalerf(&self) -> XtalerfR {
        XtalerfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc MULTIRF"]
    #[inline(always)]
    pub fn multirf(&self) -> MultirfR {
        MultirfR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc CLRF"]
    #[inline(always)]
    pub fn clrf(&self) -> ClrfR {
        ClrfR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PORF"]
    #[inline(always)]
    pub fn porf(&mut self) -> PorfW<'_, Rstf0Spec> {
        PorfW::new(self, 0)
    }
    #[doc = "Bit 1 - desc PINRF"]
    #[inline(always)]
    pub fn pinrf(&mut self) -> PinrfW<'_, Rstf0Spec> {
        PinrfW::new(self, 1)
    }
    #[doc = "Bit 2 - desc BORF"]
    #[inline(always)]
    pub fn borf(&mut self) -> BorfW<'_, Rstf0Spec> {
        BorfW::new(self, 2)
    }
    #[doc = "Bit 3 - desc PVD1RF"]
    #[inline(always)]
    pub fn pvd1rf(&mut self) -> Pvd1rfW<'_, Rstf0Spec> {
        Pvd1rfW::new(self, 3)
    }
    #[doc = "Bit 4 - desc PVD2RF"]
    #[inline(always)]
    pub fn pvd2rf(&mut self) -> Pvd2rfW<'_, Rstf0Spec> {
        Pvd2rfW::new(self, 4)
    }
    #[doc = "Bit 5 - desc WDRF"]
    #[inline(always)]
    pub fn wdrf(&mut self) -> WdrfW<'_, Rstf0Spec> {
        WdrfW::new(self, 5)
    }
    #[doc = "Bit 6 - desc SWDRF"]
    #[inline(always)]
    pub fn swdrf(&mut self) -> SwdrfW<'_, Rstf0Spec> {
        SwdrfW::new(self, 6)
    }
    #[doc = "Bit 7 - desc PDRF"]
    #[inline(always)]
    pub fn pdrf(&mut self) -> PdrfW<'_, Rstf0Spec> {
        PdrfW::new(self, 7)
    }
    #[doc = "Bit 8 - desc SWRF"]
    #[inline(always)]
    pub fn swrf(&mut self) -> SwrfW<'_, Rstf0Spec> {
        SwrfW::new(self, 8)
    }
    #[doc = "Bit 9 - desc MPUERF"]
    #[inline(always)]
    pub fn mpuerf(&mut self) -> MpuerfW<'_, Rstf0Spec> {
        MpuerfW::new(self, 9)
    }
    #[doc = "Bit 10 - desc RAPERF"]
    #[inline(always)]
    pub fn raperf(&mut self) -> RaperfW<'_, Rstf0Spec> {
        RaperfW::new(self, 10)
    }
    #[doc = "Bit 11 - desc RAECRF"]
    #[inline(always)]
    pub fn raecrf(&mut self) -> RaecrfW<'_, Rstf0Spec> {
        RaecrfW::new(self, 11)
    }
    #[doc = "Bit 12 - desc CKFERF"]
    #[inline(always)]
    pub fn ckferf(&mut self) -> CkferfW<'_, Rstf0Spec> {
        CkferfW::new(self, 12)
    }
    #[doc = "Bit 13 - desc XTALERF"]
    #[inline(always)]
    pub fn xtalerf(&mut self) -> XtalerfW<'_, Rstf0Spec> {
        XtalerfW::new(self, 13)
    }
    #[doc = "Bit 14 - desc MULTIRF"]
    #[inline(always)]
    pub fn multirf(&mut self) -> MultirfW<'_, Rstf0Spec> {
        MultirfW::new(self, 14)
    }
    #[doc = "Bit 15 - desc CLRF"]
    #[inline(always)]
    pub fn clrf(&mut self) -> ClrfW<'_, Rstf0Spec> {
        ClrfW::new(self, 15)
    }
}
#[doc = "desc RSTF0\n\nYou can [`read`](crate::Reg::read) this register and get [`rstf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rstf0Spec;
impl crate::RegisterSpec for Rstf0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rstf0::R`](R) reader structure"]
impl crate::Readable for Rstf0Spec {}
#[doc = "`write(|w| ..)` method takes [`rstf0::W`](W) writer structure"]
impl crate::Writable for Rstf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSTF0 to value 0x02"]
impl crate::Resettable for Rstf0Spec {
    const RESET_VALUE: u16 = 0x02;
}
