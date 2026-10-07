#[doc = "Register `MCUSTAT` reader"]
pub type R = crate::R<McustatSpec>;
#[doc = "Register `MCUSTAT` writer"]
pub type W = crate::W<McustatSpec>;
#[doc = "Field `AUTHFG` reader - desc AUTHFG"]
pub type AuthfgR = crate::BitReader;
#[doc = "Field `AUTHFG` writer - desc AUTHFG"]
pub type AuthfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTLV1` reader - desc PRTLV1"]
pub type Prtlv1R = crate::BitReader;
#[doc = "Field `PRTLV1` writer - desc PRTLV1"]
pub type Prtlv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTLV2` reader - desc PRTLV2"]
pub type Prtlv2R = crate::BitReader;
#[doc = "Field `PRTLV2` writer - desc PRTLV2"]
pub type Prtlv2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc AUTHFG"]
    #[inline(always)]
    pub fn authfg(&self) -> AuthfgR {
        AuthfgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - desc PRTLV1"]
    #[inline(always)]
    pub fn prtlv1(&self) -> Prtlv1R {
        Prtlv1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PRTLV2"]
    #[inline(always)]
    pub fn prtlv2(&self) -> Prtlv2R {
        Prtlv2R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc AUTHFG"]
    #[inline(always)]
    pub fn authfg(&mut self) -> AuthfgW<'_, McustatSpec> {
        AuthfgW::new(self, 0)
    }
    #[doc = "Bit 2 - desc PRTLV1"]
    #[inline(always)]
    pub fn prtlv1(&mut self) -> Prtlv1W<'_, McustatSpec> {
        Prtlv1W::new(self, 2)
    }
    #[doc = "Bit 3 - desc PRTLV2"]
    #[inline(always)]
    pub fn prtlv2(&mut self) -> Prtlv2W<'_, McustatSpec> {
        Prtlv2W::new(self, 3)
    }
}
#[doc = "desc MCUSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`mcustat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcustat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McustatSpec;
impl crate::RegisterSpec for McustatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcustat::R`](R) reader structure"]
impl crate::Readable for McustatSpec {}
#[doc = "`write(|w| ..)` method takes [`mcustat::W`](W) writer structure"]
impl crate::Writable for McustatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCUSTAT to value 0"]
impl crate::Resettable for McustatSpec {}
