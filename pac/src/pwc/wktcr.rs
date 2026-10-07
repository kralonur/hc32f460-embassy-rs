#[doc = "Register `WKTCR` reader"]
pub type R = crate::R<WktcrSpec>;
#[doc = "Register `WKTCR` writer"]
pub type W = crate::W<WktcrSpec>;
#[doc = "Field `WKTMCMP` reader - desc WKTMCMP"]
pub type WktmcmpR = crate::FieldReader<u16>;
#[doc = "Field `WKTMCMP` writer - desc WKTMCMP"]
pub type WktmcmpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WKOVF` reader - desc WKOVF"]
pub type WkovfR = crate::BitReader;
#[doc = "Field `WKOVF` writer - desc WKOVF"]
pub type WkovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKCKS` reader - desc WKCKS"]
pub type WkcksR = crate::FieldReader;
#[doc = "Field `WKCKS` writer - desc WKCKS"]
pub type WkcksW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WKTCE` reader - desc WKTCE"]
pub type WktceR = crate::BitReader;
#[doc = "Field `WKTCE` writer - desc WKTCE"]
pub type WktceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - desc WKTMCMP"]
    #[inline(always)]
    pub fn wktmcmp(&self) -> WktmcmpR {
        WktmcmpR::new(self.bits & 0x0fff)
    }
    #[doc = "Bit 12 - desc WKOVF"]
    #[inline(always)]
    pub fn wkovf(&self) -> WkovfR {
        WkovfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - desc WKCKS"]
    #[inline(always)]
    pub fn wkcks(&self) -> WkcksR {
        WkcksR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - desc WKTCE"]
    #[inline(always)]
    pub fn wktce(&self) -> WktceR {
        WktceR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc WKTMCMP"]
    #[inline(always)]
    pub fn wktmcmp(&mut self) -> WktmcmpW<'_, WktcrSpec> {
        WktmcmpW::new(self, 0)
    }
    #[doc = "Bit 12 - desc WKOVF"]
    #[inline(always)]
    pub fn wkovf(&mut self) -> WkovfW<'_, WktcrSpec> {
        WkovfW::new(self, 12)
    }
    #[doc = "Bits 13:14 - desc WKCKS"]
    #[inline(always)]
    pub fn wkcks(&mut self) -> WkcksW<'_, WktcrSpec> {
        WkcksW::new(self, 13)
    }
    #[doc = "Bit 15 - desc WKTCE"]
    #[inline(always)]
    pub fn wktce(&mut self) -> WktceW<'_, WktcrSpec> {
        WktceW::new(self, 15)
    }
}
#[doc = "desc WKTCR\n\nYou can [`read`](crate::Reg::read) this register and get [`wktcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wktcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WktcrSpec;
impl crate::RegisterSpec for WktcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wktcr::R`](R) reader structure"]
impl crate::Readable for WktcrSpec {}
#[doc = "`write(|w| ..)` method takes [`wktcr::W`](W) writer structure"]
impl crate::Writable for WktcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WKTCR to value 0"]
impl crate::Resettable for WktcrSpec {}
