#[doc = "Register `PVDICR` reader"]
pub type R = crate::R<PvdicrSpec>;
#[doc = "Register `PVDICR` writer"]
pub type W = crate::W<PvdicrSpec>;
#[doc = "Field `PVD1NMIS` reader - desc PVD1NMIS"]
pub type Pvd1nmisR = crate::BitReader;
#[doc = "Field `PVD1NMIS` writer - desc PVD1NMIS"]
pub type Pvd1nmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2NMIS` reader - desc PVD2NMIS"]
pub type Pvd2nmisR = crate::BitReader;
#[doc = "Field `PVD2NMIS` writer - desc PVD2NMIS"]
pub type Pvd2nmisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PVD1NMIS"]
    #[inline(always)]
    pub fn pvd1nmis(&self) -> Pvd1nmisR {
        Pvd1nmisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - desc PVD2NMIS"]
    #[inline(always)]
    pub fn pvd2nmis(&self) -> Pvd2nmisR {
        Pvd2nmisR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PVD1NMIS"]
    #[inline(always)]
    pub fn pvd1nmis(&mut self) -> Pvd1nmisW<'_, PvdicrSpec> {
        Pvd1nmisW::new(self, 0)
    }
    #[doc = "Bit 4 - desc PVD2NMIS"]
    #[inline(always)]
    pub fn pvd2nmis(&mut self) -> Pvd2nmisW<'_, PvdicrSpec> {
        Pvd2nmisW::new(self, 4)
    }
}
#[doc = "desc PVDICR\n\nYou can [`read`](crate::Reg::read) this register and get [`pvdicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvdicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PvdicrSpec;
impl crate::RegisterSpec for PvdicrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pvdicr::R`](R) reader structure"]
impl crate::Readable for PvdicrSpec {}
#[doc = "`write(|w| ..)` method takes [`pvdicr::W`](W) writer structure"]
impl crate::Writable for PvdicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVDICR to value 0"]
impl crate::Resettable for PvdicrSpec {}
