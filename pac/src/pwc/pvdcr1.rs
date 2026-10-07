#[doc = "Register `PVDCR1` reader"]
pub type R = crate::R<Pvdcr1Spec>;
#[doc = "Register `PVDCR1` writer"]
pub type W = crate::W<Pvdcr1Spec>;
#[doc = "Field `PVD1IRE` reader - desc PVD1IRE"]
pub type Pvd1ireR = crate::BitReader;
#[doc = "Field `PVD1IRE` writer - desc PVD1IRE"]
pub type Pvd1ireW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD1IRS` reader - desc PVD1IRS"]
pub type Pvd1irsR = crate::BitReader;
#[doc = "Field `PVD1IRS` writer - desc PVD1IRS"]
pub type Pvd1irsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD1CMPOE` reader - desc PVD1CMPOE"]
pub type Pvd1cmpoeR = crate::BitReader;
#[doc = "Field `PVD1CMPOE` writer - desc PVD1CMPOE"]
pub type Pvd1cmpoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2IRE` reader - desc PVD2IRE"]
pub type Pvd2ireR = crate::BitReader;
#[doc = "Field `PVD2IRE` writer - desc PVD2IRE"]
pub type Pvd2ireW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2IRS` reader - desc PVD2IRS"]
pub type Pvd2irsR = crate::BitReader;
#[doc = "Field `PVD2IRS` writer - desc PVD2IRS"]
pub type Pvd2irsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2CMPOE` reader - desc PVD2CMPOE"]
pub type Pvd2cmpoeR = crate::BitReader;
#[doc = "Field `PVD2CMPOE` writer - desc PVD2CMPOE"]
pub type Pvd2cmpoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PVD1IRE"]
    #[inline(always)]
    pub fn pvd1ire(&self) -> Pvd1ireR {
        Pvd1ireR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PVD1IRS"]
    #[inline(always)]
    pub fn pvd1irs(&self) -> Pvd1irsR {
        Pvd1irsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PVD1CMPOE"]
    #[inline(always)]
    pub fn pvd1cmpoe(&self) -> Pvd1cmpoeR {
        Pvd1cmpoeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PVD2IRE"]
    #[inline(always)]
    pub fn pvd2ire(&self) -> Pvd2ireR {
        Pvd2ireR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PVD2IRS"]
    #[inline(always)]
    pub fn pvd2irs(&self) -> Pvd2irsR {
        Pvd2irsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PVD2CMPOE"]
    #[inline(always)]
    pub fn pvd2cmpoe(&self) -> Pvd2cmpoeR {
        Pvd2cmpoeR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PVD1IRE"]
    #[inline(always)]
    pub fn pvd1ire(&mut self) -> Pvd1ireW<'_, Pvdcr1Spec> {
        Pvd1ireW::new(self, 0)
    }
    #[doc = "Bit 1 - desc PVD1IRS"]
    #[inline(always)]
    pub fn pvd1irs(&mut self) -> Pvd1irsW<'_, Pvdcr1Spec> {
        Pvd1irsW::new(self, 1)
    }
    #[doc = "Bit 2 - desc PVD1CMPOE"]
    #[inline(always)]
    pub fn pvd1cmpoe(&mut self) -> Pvd1cmpoeW<'_, Pvdcr1Spec> {
        Pvd1cmpoeW::new(self, 2)
    }
    #[doc = "Bit 4 - desc PVD2IRE"]
    #[inline(always)]
    pub fn pvd2ire(&mut self) -> Pvd2ireW<'_, Pvdcr1Spec> {
        Pvd2ireW::new(self, 4)
    }
    #[doc = "Bit 5 - desc PVD2IRS"]
    #[inline(always)]
    pub fn pvd2irs(&mut self) -> Pvd2irsW<'_, Pvdcr1Spec> {
        Pvd2irsW::new(self, 5)
    }
    #[doc = "Bit 6 - desc PVD2CMPOE"]
    #[inline(always)]
    pub fn pvd2cmpoe(&mut self) -> Pvd2cmpoeW<'_, Pvdcr1Spec> {
        Pvd2cmpoeW::new(self, 6)
    }
}
#[doc = "desc PVDCR1\n\nYou can [`read`](crate::Reg::read) this register and get [`pvdcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvdcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pvdcr1Spec;
impl crate::RegisterSpec for Pvdcr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pvdcr1::R`](R) reader structure"]
impl crate::Readable for Pvdcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pvdcr1::W`](W) writer structure"]
impl crate::Writable for Pvdcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVDCR1 to value 0"]
impl crate::Resettable for Pvdcr1Spec {}
