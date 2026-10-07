#[doc = "Register `CCONR7` reader"]
pub type R = crate::R<Cconr7Spec>;
#[doc = "Register `CCONR7` writer"]
pub type W = crate::W<Cconr7Spec>;
#[doc = "Field `CAPMD` reader - desc CAPMD"]
pub type CapmdR = crate::BitReader;
#[doc = "Field `CAPMD` writer - desc CAPMD"]
pub type CapmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICP0` reader - desc HICP0"]
pub type Hicp0R = crate::BitReader;
#[doc = "Field `HICP0` writer - desc HICP0"]
pub type Hicp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICP1` reader - desc HICP1"]
pub type Hicp1R = crate::BitReader;
#[doc = "Field `HICP1` writer - desc HICP1"]
pub type Hicp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICP2` reader - desc HICP2"]
pub type Hicp2R = crate::BitReader;
#[doc = "Field `HICP2` writer - desc HICP2"]
pub type Hicp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICP3` reader - desc HICP3"]
pub type Hicp3R = crate::BitReader;
#[doc = "Field `HICP3` writer - desc HICP3"]
pub type Hicp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICP4` reader - desc HICP4"]
pub type Hicp4R = crate::BitReader;
#[doc = "Field `HICP4` writer - desc HICP4"]
pub type Hicp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOFIENCP` reader - desc NOFIENCP"]
pub type NofiencpR = crate::BitReader;
#[doc = "Field `NOFIENCP` writer - desc NOFIENCP"]
pub type NofiencpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOFICKCP` reader - desc NOFICKCP"]
pub type NofickcpR = crate::FieldReader;
#[doc = "Field `NOFICKCP` writer - desc NOFICKCP"]
pub type NofickcpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc CAPMD"]
    #[inline(always)]
    pub fn capmd(&self) -> CapmdR {
        CapmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - desc HICP0"]
    #[inline(always)]
    pub fn hicp0(&self) -> Hicp0R {
        Hicp0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HICP1"]
    #[inline(always)]
    pub fn hicp1(&self) -> Hicp1R {
        Hicp1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HICP2"]
    #[inline(always)]
    pub fn hicp2(&self) -> Hicp2R {
        Hicp2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HICP3"]
    #[inline(always)]
    pub fn hicp3(&self) -> Hicp3R {
        Hicp3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HICP4"]
    #[inline(always)]
    pub fn hicp4(&self) -> Hicp4R {
        Hicp4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - desc NOFIENCP"]
    #[inline(always)]
    pub fn nofiencp(&self) -> NofiencpR {
        NofiencpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - desc NOFICKCP"]
    #[inline(always)]
    pub fn nofickcp(&self) -> NofickcpR {
        NofickcpR::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc CAPMD"]
    #[inline(always)]
    pub fn capmd(&mut self) -> CapmdW<'_, Cconr7Spec> {
        CapmdW::new(self, 0)
    }
    #[doc = "Bit 4 - desc HICP0"]
    #[inline(always)]
    pub fn hicp0(&mut self) -> Hicp0W<'_, Cconr7Spec> {
        Hicp0W::new(self, 4)
    }
    #[doc = "Bit 5 - desc HICP1"]
    #[inline(always)]
    pub fn hicp1(&mut self) -> Hicp1W<'_, Cconr7Spec> {
        Hicp1W::new(self, 5)
    }
    #[doc = "Bit 6 - desc HICP2"]
    #[inline(always)]
    pub fn hicp2(&mut self) -> Hicp2W<'_, Cconr7Spec> {
        Hicp2W::new(self, 6)
    }
    #[doc = "Bit 8 - desc HICP3"]
    #[inline(always)]
    pub fn hicp3(&mut self) -> Hicp3W<'_, Cconr7Spec> {
        Hicp3W::new(self, 8)
    }
    #[doc = "Bit 9 - desc HICP4"]
    #[inline(always)]
    pub fn hicp4(&mut self) -> Hicp4W<'_, Cconr7Spec> {
        Hicp4W::new(self, 9)
    }
    #[doc = "Bit 12 - desc NOFIENCP"]
    #[inline(always)]
    pub fn nofiencp(&mut self) -> NofiencpW<'_, Cconr7Spec> {
        NofiencpW::new(self, 12)
    }
    #[doc = "Bits 13:14 - desc NOFICKCP"]
    #[inline(always)]
    pub fn nofickcp(&mut self) -> NofickcpW<'_, Cconr7Spec> {
        NofickcpW::new(self, 13)
    }
}
#[doc = "desc CCONR7\n\nYou can [`read`](crate::Reg::read) this register and get [`cconr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cconr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cconr7Spec;
impl crate::RegisterSpec for Cconr7Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cconr7::R`](R) reader structure"]
impl crate::Readable for Cconr7Spec {}
#[doc = "`write(|w| ..)` method takes [`cconr7::W`](W) writer structure"]
impl crate::Writable for Cconr7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCONR7 to value 0"]
impl crate::Resettable for Cconr7Spec {}
