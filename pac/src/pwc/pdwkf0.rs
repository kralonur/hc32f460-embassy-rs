#[doc = "Register `PDWKF0` reader"]
pub type R = crate::R<Pdwkf0Spec>;
#[doc = "Register `PDWKF0` writer"]
pub type W = crate::W<Pdwkf0Spec>;
#[doc = "Field `PTWK0F` reader - desc PTWK0F"]
pub type Ptwk0fR = crate::BitReader;
#[doc = "Field `PTWK0F` writer - desc PTWK0F"]
pub type Ptwk0fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTWK1F` reader - desc PTWK1F"]
pub type Ptwk1fR = crate::BitReader;
#[doc = "Field `PTWK1F` writer - desc PTWK1F"]
pub type Ptwk1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTWK2F` reader - desc PTWK2F"]
pub type Ptwk2fR = crate::BitReader;
#[doc = "Field `PTWK2F` writer - desc PTWK2F"]
pub type Ptwk2fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTWK3F` reader - desc PTWK3F"]
pub type Ptwk3fR = crate::BitReader;
#[doc = "Field `PTWK3F` writer - desc PTWK3F"]
pub type Ptwk3fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VD1WKF` reader - desc VD1WKF"]
pub type Vd1wkfR = crate::BitReader;
#[doc = "Field `VD1WKF` writer - desc VD1WKF"]
pub type Vd1wkfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VD2WKF` reader - desc VD2WKF"]
pub type Vd2wkfR = crate::BitReader;
#[doc = "Field `VD2WKF` writer - desc VD2WKF"]
pub type Vd2wkfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIWKF` reader - desc NMIWKF"]
pub type NmiwkfR = crate::BitReader;
#[doc = "Field `NMIWKF` writer - desc NMIWKF"]
pub type NmiwkfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PTWK0F"]
    #[inline(always)]
    pub fn ptwk0f(&self) -> Ptwk0fR {
        Ptwk0fR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PTWK1F"]
    #[inline(always)]
    pub fn ptwk1f(&self) -> Ptwk1fR {
        Ptwk1fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PTWK2F"]
    #[inline(always)]
    pub fn ptwk2f(&self) -> Ptwk2fR {
        Ptwk2fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PTWK3F"]
    #[inline(always)]
    pub fn ptwk3f(&self) -> Ptwk3fR {
        Ptwk3fR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc VD1WKF"]
    #[inline(always)]
    pub fn vd1wkf(&self) -> Vd1wkfR {
        Vd1wkfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc VD2WKF"]
    #[inline(always)]
    pub fn vd2wkf(&self) -> Vd2wkfR {
        Vd2wkfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc NMIWKF"]
    #[inline(always)]
    pub fn nmiwkf(&self) -> NmiwkfR {
        NmiwkfR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PTWK0F"]
    #[inline(always)]
    pub fn ptwk0f(&mut self) -> Ptwk0fW<'_, Pdwkf0Spec> {
        Ptwk0fW::new(self, 0)
    }
    #[doc = "Bit 1 - desc PTWK1F"]
    #[inline(always)]
    pub fn ptwk1f(&mut self) -> Ptwk1fW<'_, Pdwkf0Spec> {
        Ptwk1fW::new(self, 1)
    }
    #[doc = "Bit 2 - desc PTWK2F"]
    #[inline(always)]
    pub fn ptwk2f(&mut self) -> Ptwk2fW<'_, Pdwkf0Spec> {
        Ptwk2fW::new(self, 2)
    }
    #[doc = "Bit 3 - desc PTWK3F"]
    #[inline(always)]
    pub fn ptwk3f(&mut self) -> Ptwk3fW<'_, Pdwkf0Spec> {
        Ptwk3fW::new(self, 3)
    }
    #[doc = "Bit 4 - desc VD1WKF"]
    #[inline(always)]
    pub fn vd1wkf(&mut self) -> Vd1wkfW<'_, Pdwkf0Spec> {
        Vd1wkfW::new(self, 4)
    }
    #[doc = "Bit 5 - desc VD2WKF"]
    #[inline(always)]
    pub fn vd2wkf(&mut self) -> Vd2wkfW<'_, Pdwkf0Spec> {
        Vd2wkfW::new(self, 5)
    }
    #[doc = "Bit 6 - desc NMIWKF"]
    #[inline(always)]
    pub fn nmiwkf(&mut self) -> NmiwkfW<'_, Pdwkf0Spec> {
        NmiwkfW::new(self, 6)
    }
}
#[doc = "desc PDWKF0\n\nYou can [`read`](crate::Reg::read) this register and get [`pdwkf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdwkf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdwkf0Spec;
impl crate::RegisterSpec for Pdwkf0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pdwkf0::R`](R) reader structure"]
impl crate::Readable for Pdwkf0Spec {}
#[doc = "`write(|w| ..)` method takes [`pdwkf0::W`](W) writer structure"]
impl crate::Writable for Pdwkf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDWKF0 to value 0"]
impl crate::Resettable for Pdwkf0Spec {}
