#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `BUSY` reader - desc BUSY"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - desc BUSY"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIPF` reader - desc XIPF"]
pub type XipfR = crate::BitReader;
#[doc = "Field `XIPF` writer - desc XIPF"]
pub type XipfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAER` reader - desc RAER"]
pub type RaerR = crate::BitReader;
#[doc = "Field `RAER` writer - desc RAER"]
pub type RaerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFNUM` reader - desc PFNUM"]
pub type PfnumR = crate::FieldReader;
#[doc = "Field `PFNUM` writer - desc PFNUM"]
pub type PfnumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PFFUL` reader - desc PFFUL"]
pub type PffulR = crate::BitReader;
#[doc = "Field `PFFUL` writer - desc PFFUL"]
pub type PffulW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFAN` reader - desc PFAN"]
pub type PfanR = crate::BitReader;
#[doc = "Field `PFAN` writer - desc PFAN"]
pub type PfanW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - desc XIPF"]
    #[inline(always)]
    pub fn xipf(&self) -> XipfR {
        XipfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RAER"]
    #[inline(always)]
    pub fn raer(&self) -> RaerR {
        RaerR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - desc PFNUM"]
    #[inline(always)]
    pub fn pfnum(&self) -> PfnumR {
        PfnumR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - desc PFFUL"]
    #[inline(always)]
    pub fn pfful(&self) -> PffulR {
        PffulR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PFAN"]
    #[inline(always)]
    pub fn pfan(&self) -> PfanR {
        PfanR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<'_, SrSpec> {
        BusyW::new(self, 0)
    }
    #[doc = "Bit 6 - desc XIPF"]
    #[inline(always)]
    pub fn xipf(&mut self) -> XipfW<'_, SrSpec> {
        XipfW::new(self, 6)
    }
    #[doc = "Bit 7 - desc RAER"]
    #[inline(always)]
    pub fn raer(&mut self) -> RaerW<'_, SrSpec> {
        RaerW::new(self, 7)
    }
    #[doc = "Bits 8:12 - desc PFNUM"]
    #[inline(always)]
    pub fn pfnum(&mut self) -> PfnumW<'_, SrSpec> {
        PfnumW::new(self, 8)
    }
    #[doc = "Bit 14 - desc PFFUL"]
    #[inline(always)]
    pub fn pfful(&mut self) -> PffulW<'_, SrSpec> {
        PffulW::new(self, 14)
    }
    #[doc = "Bit 15 - desc PFAN"]
    #[inline(always)]
    pub fn pfan(&mut self) -> PfanW<'_, SrSpec> {
        PfanW::new(self, 15)
    }
}
#[doc = "desc SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0x8000"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x8000;
}
