#[doc = "Register `TPIUCKCFGR` reader"]
pub type R = crate::R<TpiuckcfgrSpec>;
#[doc = "Register `TPIUCKCFGR` writer"]
pub type W = crate::W<TpiuckcfgrSpec>;
#[doc = "Field `TPIUCKS` reader - desc TPIUCKS"]
pub type TpiucksR = crate::FieldReader;
#[doc = "Field `TPIUCKS` writer - desc TPIUCKS"]
pub type TpiucksW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TPIUCKOE` reader - desc TPIUCKOE"]
pub type TpiuckoeR = crate::BitReader;
#[doc = "Field `TPIUCKOE` writer - desc TPIUCKOE"]
pub type TpiuckoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc TPIUCKS"]
    #[inline(always)]
    pub fn tpiucks(&self) -> TpiucksR {
        TpiucksR::new(self.bits & 3)
    }
    #[doc = "Bit 7 - desc TPIUCKOE"]
    #[inline(always)]
    pub fn tpiuckoe(&self) -> TpiuckoeR {
        TpiuckoeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc TPIUCKS"]
    #[inline(always)]
    pub fn tpiucks(&mut self) -> TpiucksW<'_, TpiuckcfgrSpec> {
        TpiucksW::new(self, 0)
    }
    #[doc = "Bit 7 - desc TPIUCKOE"]
    #[inline(always)]
    pub fn tpiuckoe(&mut self) -> TpiuckoeW<'_, TpiuckcfgrSpec> {
        TpiuckoeW::new(self, 7)
    }
}
#[doc = "desc TPIUCKCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`tpiuckcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpiuckcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TpiuckcfgrSpec;
impl crate::RegisterSpec for TpiuckcfgrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tpiuckcfgr::R`](R) reader structure"]
impl crate::Readable for TpiuckcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`tpiuckcfgr::W`](W) writer structure"]
impl crate::Writable for TpiuckcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPIUCKCFGR to value 0"]
impl crate::Resettable for TpiuckcfgrSpec {}
