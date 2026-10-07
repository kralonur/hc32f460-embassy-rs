#[doc = "Register `PVDFCR` reader"]
pub type R = crate::R<PvdfcrSpec>;
#[doc = "Register `PVDFCR` writer"]
pub type W = crate::W<PvdfcrSpec>;
#[doc = "Field `PVD1NFDIS` reader - desc PVD1NFDIS"]
pub type Pvd1nfdisR = crate::BitReader;
#[doc = "Field `PVD1NFDIS` writer - desc PVD1NFDIS"]
pub type Pvd1nfdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD1NFCKS` reader - desc PVD1NFCKS"]
pub type Pvd1nfcksR = crate::FieldReader;
#[doc = "Field `PVD1NFCKS` writer - desc PVD1NFCKS"]
pub type Pvd1nfcksW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PVD2NFDIS` reader - desc PVD2NFDIS"]
pub type Pvd2nfdisR = crate::BitReader;
#[doc = "Field `PVD2NFDIS` writer - desc PVD2NFDIS"]
pub type Pvd2nfdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2NFCKS` reader - desc PVD2NFCKS"]
pub type Pvd2nfcksR = crate::FieldReader;
#[doc = "Field `PVD2NFCKS` writer - desc PVD2NFCKS"]
pub type Pvd2nfcksW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc PVD1NFDIS"]
    #[inline(always)]
    pub fn pvd1nfdis(&self) -> Pvd1nfdisR {
        Pvd1nfdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc PVD1NFCKS"]
    #[inline(always)]
    pub fn pvd1nfcks(&self) -> Pvd1nfcksR {
        Pvd1nfcksR::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 4 - desc PVD2NFDIS"]
    #[inline(always)]
    pub fn pvd2nfdis(&self) -> Pvd2nfdisR {
        Pvd2nfdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - desc PVD2NFCKS"]
    #[inline(always)]
    pub fn pvd2nfcks(&self) -> Pvd2nfcksR {
        Pvd2nfcksR::new((self.bits >> 5) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - desc PVD1NFDIS"]
    #[inline(always)]
    pub fn pvd1nfdis(&mut self) -> Pvd1nfdisW<'_, PvdfcrSpec> {
        Pvd1nfdisW::new(self, 0)
    }
    #[doc = "Bits 1:2 - desc PVD1NFCKS"]
    #[inline(always)]
    pub fn pvd1nfcks(&mut self) -> Pvd1nfcksW<'_, PvdfcrSpec> {
        Pvd1nfcksW::new(self, 1)
    }
    #[doc = "Bit 4 - desc PVD2NFDIS"]
    #[inline(always)]
    pub fn pvd2nfdis(&mut self) -> Pvd2nfdisW<'_, PvdfcrSpec> {
        Pvd2nfdisW::new(self, 4)
    }
    #[doc = "Bits 5:6 - desc PVD2NFCKS"]
    #[inline(always)]
    pub fn pvd2nfcks(&mut self) -> Pvd2nfcksW<'_, PvdfcrSpec> {
        Pvd2nfcksW::new(self, 5)
    }
}
#[doc = "desc PVDFCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pvdfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvdfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PvdfcrSpec;
impl crate::RegisterSpec for PvdfcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pvdfcr::R`](R) reader structure"]
impl crate::Readable for PvdfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pvdfcr::W`](W) writer structure"]
impl crate::Writable for PvdfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVDFCR to value 0x11"]
impl crate::Resettable for PvdfcrSpec {
    const RESET_VALUE: u8 = 0x11;
}
