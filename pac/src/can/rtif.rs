#[doc = "Register `RTIF` reader"]
pub type R = crate::R<RtifSpec>;
#[doc = "Register `RTIF` writer"]
pub type W = crate::W<RtifSpec>;
#[doc = "Field `AIF` reader - desc AIF"]
pub type AifR = crate::BitReader;
#[doc = "Field `AIF` writer - desc AIF"]
pub type AifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIF` reader - desc EIF"]
pub type EifR = crate::BitReader;
#[doc = "Field `EIF` writer - desc EIF"]
pub type EifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIF` reader - desc TSIF"]
pub type TsifR = crate::BitReader;
#[doc = "Field `TSIF` writer - desc TSIF"]
pub type TsifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPIF` reader - desc TPIF"]
pub type TpifR = crate::BitReader;
#[doc = "Field `TPIF` writer - desc TPIF"]
pub type TpifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAFIF` reader - desc RAFIF"]
pub type RafifR = crate::BitReader;
#[doc = "Field `RAFIF` writer - desc RAFIF"]
pub type RafifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIF` reader - desc RFIF"]
pub type RfifR = crate::BitReader;
#[doc = "Field `RFIF` writer - desc RFIF"]
pub type RfifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROIF` reader - desc ROIF"]
pub type RoifR = crate::BitReader;
#[doc = "Field `ROIF` writer - desc ROIF"]
pub type RoifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIF` reader - desc RIF"]
pub type RifR = crate::BitReader;
#[doc = "Field `RIF` writer - desc RIF"]
pub type RifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc AIF"]
    #[inline(always)]
    pub fn aif(&self) -> AifR {
        AifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EIF"]
    #[inline(always)]
    pub fn eif(&self) -> EifR {
        EifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TSIF"]
    #[inline(always)]
    pub fn tsif(&self) -> TsifR {
        TsifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TPIF"]
    #[inline(always)]
    pub fn tpif(&self) -> TpifR {
        TpifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RAFIF"]
    #[inline(always)]
    pub fn rafif(&self) -> RafifR {
        RafifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RFIF"]
    #[inline(always)]
    pub fn rfif(&self) -> RfifR {
        RfifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ROIF"]
    #[inline(always)]
    pub fn roif(&self) -> RoifR {
        RoifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RIF"]
    #[inline(always)]
    pub fn rif(&self) -> RifR {
        RifR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc AIF"]
    #[inline(always)]
    pub fn aif(&mut self) -> AifW<'_, RtifSpec> {
        AifW::new(self, 0)
    }
    #[doc = "Bit 1 - desc EIF"]
    #[inline(always)]
    pub fn eif(&mut self) -> EifW<'_, RtifSpec> {
        EifW::new(self, 1)
    }
    #[doc = "Bit 2 - desc TSIF"]
    #[inline(always)]
    pub fn tsif(&mut self) -> TsifW<'_, RtifSpec> {
        TsifW::new(self, 2)
    }
    #[doc = "Bit 3 - desc TPIF"]
    #[inline(always)]
    pub fn tpif(&mut self) -> TpifW<'_, RtifSpec> {
        TpifW::new(self, 3)
    }
    #[doc = "Bit 4 - desc RAFIF"]
    #[inline(always)]
    pub fn rafif(&mut self) -> RafifW<'_, RtifSpec> {
        RafifW::new(self, 4)
    }
    #[doc = "Bit 5 - desc RFIF"]
    #[inline(always)]
    pub fn rfif(&mut self) -> RfifW<'_, RtifSpec> {
        RfifW::new(self, 5)
    }
    #[doc = "Bit 6 - desc ROIF"]
    #[inline(always)]
    pub fn roif(&mut self) -> RoifW<'_, RtifSpec> {
        RoifW::new(self, 6)
    }
    #[doc = "Bit 7 - desc RIF"]
    #[inline(always)]
    pub fn rif(&mut self) -> RifW<'_, RtifSpec> {
        RifW::new(self, 7)
    }
}
#[doc = "desc RTIF\n\nYou can [`read`](crate::Reg::read) this register and get [`rtif::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtif::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtifSpec;
impl crate::RegisterSpec for RtifSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtif::R`](R) reader structure"]
impl crate::Readable for RtifSpec {}
#[doc = "`write(|w| ..)` method takes [`rtif::W`](W) writer structure"]
impl crate::Writable for RtifSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTIF to value 0"]
impl crate::Resettable for RtifSpec {}
