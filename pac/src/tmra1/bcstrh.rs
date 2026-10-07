#[doc = "Register `BCSTRH` reader"]
pub type R = crate::R<BcstrhSpec>;
#[doc = "Register `BCSTRH` writer"]
pub type W = crate::W<BcstrhSpec>;
#[doc = "Field `OVSTP` reader - desc OVSTP"]
pub type OvstpR = crate::BitReader;
#[doc = "Field `OVSTP` writer - desc OVSTP"]
pub type OvstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITENOVF` reader - desc ITENOVF"]
pub type ItenovfR = crate::BitReader;
#[doc = "Field `ITENOVF` writer - desc ITENOVF"]
pub type ItenovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITENUDF` reader - desc ITENUDF"]
pub type ItenudfR = crate::BitReader;
#[doc = "Field `ITENUDF` writer - desc ITENUDF"]
pub type ItenudfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFF` reader - desc OVFF"]
pub type OvffR = crate::BitReader;
#[doc = "Field `OVFF` writer - desc OVFF"]
pub type OvffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDFF` reader - desc UDFF"]
pub type UdffR = crate::BitReader;
#[doc = "Field `UDFF` writer - desc UDFF"]
pub type UdffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc OVSTP"]
    #[inline(always)]
    pub fn ovstp(&self) -> OvstpR {
        OvstpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - desc ITENOVF"]
    #[inline(always)]
    pub fn itenovf(&self) -> ItenovfR {
        ItenovfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ITENUDF"]
    #[inline(always)]
    pub fn itenudf(&self) -> ItenudfR {
        ItenudfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc OVFF"]
    #[inline(always)]
    pub fn ovff(&self) -> OvffR {
        OvffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc UDFF"]
    #[inline(always)]
    pub fn udff(&self) -> UdffR {
        UdffR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc OVSTP"]
    #[inline(always)]
    pub fn ovstp(&mut self) -> OvstpW<'_, BcstrhSpec> {
        OvstpW::new(self, 0)
    }
    #[doc = "Bit 4 - desc ITENOVF"]
    #[inline(always)]
    pub fn itenovf(&mut self) -> ItenovfW<'_, BcstrhSpec> {
        ItenovfW::new(self, 4)
    }
    #[doc = "Bit 5 - desc ITENUDF"]
    #[inline(always)]
    pub fn itenudf(&mut self) -> ItenudfW<'_, BcstrhSpec> {
        ItenudfW::new(self, 5)
    }
    #[doc = "Bit 6 - desc OVFF"]
    #[inline(always)]
    pub fn ovff(&mut self) -> OvffW<'_, BcstrhSpec> {
        OvffW::new(self, 6)
    }
    #[doc = "Bit 7 - desc UDFF"]
    #[inline(always)]
    pub fn udff(&mut self) -> UdffW<'_, BcstrhSpec> {
        UdffW::new(self, 7)
    }
}
#[doc = "desc BCSTRH\n\nYou can [`read`](crate::Reg::read) this register and get [`bcstrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcstrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcstrhSpec;
impl crate::RegisterSpec for BcstrhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcstrh::R`](R) reader structure"]
impl crate::Readable for BcstrhSpec {}
#[doc = "`write(|w| ..)` method takes [`bcstrh::W`](W) writer structure"]
impl crate::Writable for BcstrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCSTRH to value 0"]
impl crate::Resettable for BcstrhSpec {}
