#[doc = "Register `PVDDSR` reader"]
pub type R = crate::R<PvddsrSpec>;
#[doc = "Register `PVDDSR` writer"]
pub type W = crate::W<PvddsrSpec>;
#[doc = "Field `PVD1MON` reader - desc PVD1MON"]
pub type Pvd1monR = crate::BitReader;
#[doc = "Field `PVD1MON` writer - desc PVD1MON"]
pub type Pvd1monW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD1DETFLG` reader - desc PVD1DETFLG"]
pub type Pvd1detflgR = crate::BitReader;
#[doc = "Field `PVD1DETFLG` writer - desc PVD1DETFLG"]
pub type Pvd1detflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2MON` reader - desc PVD2MON"]
pub type Pvd2monR = crate::BitReader;
#[doc = "Field `PVD2MON` writer - desc PVD2MON"]
pub type Pvd2monW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2DETFLG` reader - desc PVD2DETFLG"]
pub type Pvd2detflgR = crate::BitReader;
#[doc = "Field `PVD2DETFLG` writer - desc PVD2DETFLG"]
pub type Pvd2detflgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PVD1MON"]
    #[inline(always)]
    pub fn pvd1mon(&self) -> Pvd1monR {
        Pvd1monR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PVD1DETFLG"]
    #[inline(always)]
    pub fn pvd1detflg(&self) -> Pvd1detflgR {
        Pvd1detflgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PVD2MON"]
    #[inline(always)]
    pub fn pvd2mon(&self) -> Pvd2monR {
        Pvd2monR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PVD2DETFLG"]
    #[inline(always)]
    pub fn pvd2detflg(&self) -> Pvd2detflgR {
        Pvd2detflgR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PVD1MON"]
    #[inline(always)]
    pub fn pvd1mon(&mut self) -> Pvd1monW<'_, PvddsrSpec> {
        Pvd1monW::new(self, 0)
    }
    #[doc = "Bit 1 - desc PVD1DETFLG"]
    #[inline(always)]
    pub fn pvd1detflg(&mut self) -> Pvd1detflgW<'_, PvddsrSpec> {
        Pvd1detflgW::new(self, 1)
    }
    #[doc = "Bit 4 - desc PVD2MON"]
    #[inline(always)]
    pub fn pvd2mon(&mut self) -> Pvd2monW<'_, PvddsrSpec> {
        Pvd2monW::new(self, 4)
    }
    #[doc = "Bit 5 - desc PVD2DETFLG"]
    #[inline(always)]
    pub fn pvd2detflg(&mut self) -> Pvd2detflgW<'_, PvddsrSpec> {
        Pvd2detflgW::new(self, 5)
    }
}
#[doc = "desc PVDDSR\n\nYou can [`read`](crate::Reg::read) this register and get [`pvddsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvddsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PvddsrSpec;
impl crate::RegisterSpec for PvddsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pvddsr::R`](R) reader structure"]
impl crate::Readable for PvddsrSpec {}
#[doc = "`write(|w| ..)` method takes [`pvddsr::W`](W) writer structure"]
impl crate::Writable for PvddsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVDDSR to value 0x11"]
impl crate::Resettable for PvddsrSpec {
    const RESET_VALUE: u8 = 0x11;
}
