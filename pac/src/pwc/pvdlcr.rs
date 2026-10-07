#[doc = "Register `PVDLCR` reader"]
pub type R = crate::R<PvdlcrSpec>;
#[doc = "Register `PVDLCR` writer"]
pub type W = crate::W<PvdlcrSpec>;
#[doc = "Field `PVD1LVL` reader - desc PVD1LVL"]
pub type Pvd1lvlR = crate::FieldReader;
#[doc = "Field `PVD1LVL` writer - desc PVD1LVL"]
pub type Pvd1lvlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PVD2LVL` reader - desc PVD2LVL"]
pub type Pvd2lvlR = crate::FieldReader;
#[doc = "Field `PVD2LVL` writer - desc PVD2LVL"]
pub type Pvd2lvlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc PVD1LVL"]
    #[inline(always)]
    pub fn pvd1lvl(&self) -> Pvd1lvlR {
        Pvd1lvlR::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - desc PVD2LVL"]
    #[inline(always)]
    pub fn pvd2lvl(&self) -> Pvd2lvlR {
        Pvd2lvlR::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc PVD1LVL"]
    #[inline(always)]
    pub fn pvd1lvl(&mut self) -> Pvd1lvlW<'_, PvdlcrSpec> {
        Pvd1lvlW::new(self, 0)
    }
    #[doc = "Bits 4:6 - desc PVD2LVL"]
    #[inline(always)]
    pub fn pvd2lvl(&mut self) -> Pvd2lvlW<'_, PvdlcrSpec> {
        Pvd2lvlW::new(self, 4)
    }
}
#[doc = "desc PVDLCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pvdlcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvdlcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PvdlcrSpec;
impl crate::RegisterSpec for PvdlcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pvdlcr::R`](R) reader structure"]
impl crate::Readable for PvdlcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pvdlcr::W`](W) writer structure"]
impl crate::Writable for PvdlcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVDLCR to value 0"]
impl crate::Resettable for PvdlcrSpec {}
