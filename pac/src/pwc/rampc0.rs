#[doc = "Register `RAMPC0` reader"]
pub type R = crate::R<Rampc0Spec>;
#[doc = "Register `RAMPC0` writer"]
pub type W = crate::W<Rampc0Spec>;
#[doc = "Field `RAMPDC0` reader - desc RAMPDC0"]
pub type Rampdc0R = crate::BitReader;
#[doc = "Field `RAMPDC0` writer - desc RAMPDC0"]
pub type Rampdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMPDC1` reader - desc RAMPDC1"]
pub type Rampdc1R = crate::BitReader;
#[doc = "Field `RAMPDC1` writer - desc RAMPDC1"]
pub type Rampdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMPDC2` reader - desc RAMPDC2"]
pub type Rampdc2R = crate::BitReader;
#[doc = "Field `RAMPDC2` writer - desc RAMPDC2"]
pub type Rampdc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMPDC3` reader - desc RAMPDC3"]
pub type Rampdc3R = crate::BitReader;
#[doc = "Field `RAMPDC3` writer - desc RAMPDC3"]
pub type Rampdc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMPDC4` reader - desc RAMPDC4"]
pub type Rampdc4R = crate::BitReader;
#[doc = "Field `RAMPDC4` writer - desc RAMPDC4"]
pub type Rampdc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMPDC5` reader - desc RAMPDC5"]
pub type Rampdc5R = crate::BitReader;
#[doc = "Field `RAMPDC5` writer - desc RAMPDC5"]
pub type Rampdc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMPDC6` reader - desc RAMPDC6"]
pub type Rampdc6R = crate::BitReader;
#[doc = "Field `RAMPDC6` writer - desc RAMPDC6"]
pub type Rampdc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMPDC7` reader - desc RAMPDC7"]
pub type Rampdc7R = crate::BitReader;
#[doc = "Field `RAMPDC7` writer - desc RAMPDC7"]
pub type Rampdc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMPDC8` reader - desc RAMPDC8"]
pub type Rampdc8R = crate::BitReader;
#[doc = "Field `RAMPDC8` writer - desc RAMPDC8"]
pub type Rampdc8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc RAMPDC0"]
    #[inline(always)]
    pub fn rampdc0(&self) -> Rampdc0R {
        Rampdc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RAMPDC1"]
    #[inline(always)]
    pub fn rampdc1(&self) -> Rampdc1R {
        Rampdc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc RAMPDC2"]
    #[inline(always)]
    pub fn rampdc2(&self) -> Rampdc2R {
        Rampdc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc RAMPDC3"]
    #[inline(always)]
    pub fn rampdc3(&self) -> Rampdc3R {
        Rampdc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RAMPDC4"]
    #[inline(always)]
    pub fn rampdc4(&self) -> Rampdc4R {
        Rampdc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RAMPDC5"]
    #[inline(always)]
    pub fn rampdc5(&self) -> Rampdc5R {
        Rampdc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RAMPDC6"]
    #[inline(always)]
    pub fn rampdc6(&self) -> Rampdc6R {
        Rampdc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RAMPDC7"]
    #[inline(always)]
    pub fn rampdc7(&self) -> Rampdc7R {
        Rampdc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RAMPDC8"]
    #[inline(always)]
    pub fn rampdc8(&self) -> Rampdc8R {
        Rampdc8R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc RAMPDC0"]
    #[inline(always)]
    pub fn rampdc0(&mut self) -> Rampdc0W<'_, Rampc0Spec> {
        Rampdc0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc RAMPDC1"]
    #[inline(always)]
    pub fn rampdc1(&mut self) -> Rampdc1W<'_, Rampc0Spec> {
        Rampdc1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc RAMPDC2"]
    #[inline(always)]
    pub fn rampdc2(&mut self) -> Rampdc2W<'_, Rampc0Spec> {
        Rampdc2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc RAMPDC3"]
    #[inline(always)]
    pub fn rampdc3(&mut self) -> Rampdc3W<'_, Rampc0Spec> {
        Rampdc3W::new(self, 3)
    }
    #[doc = "Bit 4 - desc RAMPDC4"]
    #[inline(always)]
    pub fn rampdc4(&mut self) -> Rampdc4W<'_, Rampc0Spec> {
        Rampdc4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc RAMPDC5"]
    #[inline(always)]
    pub fn rampdc5(&mut self) -> Rampdc5W<'_, Rampc0Spec> {
        Rampdc5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc RAMPDC6"]
    #[inline(always)]
    pub fn rampdc6(&mut self) -> Rampdc6W<'_, Rampc0Spec> {
        Rampdc6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc RAMPDC7"]
    #[inline(always)]
    pub fn rampdc7(&mut self) -> Rampdc7W<'_, Rampc0Spec> {
        Rampdc7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc RAMPDC8"]
    #[inline(always)]
    pub fn rampdc8(&mut self) -> Rampdc8W<'_, Rampc0Spec> {
        Rampdc8W::new(self, 8)
    }
}
#[doc = "desc RAMPC0\n\nYou can [`read`](crate::Reg::read) this register and get [`rampc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rampc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rampc0Spec;
impl crate::RegisterSpec for Rampc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rampc0::R`](R) reader structure"]
impl crate::Readable for Rampc0Spec {}
#[doc = "`write(|w| ..)` method takes [`rampc0::W`](W) writer structure"]
impl crate::Writable for Rampc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAMPC0 to value 0"]
impl crate::Resettable for Rampc0Spec {}
