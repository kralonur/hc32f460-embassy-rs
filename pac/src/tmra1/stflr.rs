#[doc = "Register `STFLR` reader"]
pub type R = crate::R<StflrSpec>;
#[doc = "Register `STFLR` writer"]
pub type W = crate::W<StflrSpec>;
#[doc = "Field `CMPF1` reader - desc CMPF1"]
pub type Cmpf1R = crate::BitReader;
#[doc = "Field `CMPF1` writer - desc CMPF1"]
pub type Cmpf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPF2` reader - desc CMPF2"]
pub type Cmpf2R = crate::BitReader;
#[doc = "Field `CMPF2` writer - desc CMPF2"]
pub type Cmpf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPF3` reader - desc CMPF3"]
pub type Cmpf3R = crate::BitReader;
#[doc = "Field `CMPF3` writer - desc CMPF3"]
pub type Cmpf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPF4` reader - desc CMPF4"]
pub type Cmpf4R = crate::BitReader;
#[doc = "Field `CMPF4` writer - desc CMPF4"]
pub type Cmpf4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPF5` reader - desc CMPF5"]
pub type Cmpf5R = crate::BitReader;
#[doc = "Field `CMPF5` writer - desc CMPF5"]
pub type Cmpf5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPF6` reader - desc CMPF6"]
pub type Cmpf6R = crate::BitReader;
#[doc = "Field `CMPF6` writer - desc CMPF6"]
pub type Cmpf6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPF7` reader - desc CMPF7"]
pub type Cmpf7R = crate::BitReader;
#[doc = "Field `CMPF7` writer - desc CMPF7"]
pub type Cmpf7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPF8` reader - desc CMPF8"]
pub type Cmpf8R = crate::BitReader;
#[doc = "Field `CMPF8` writer - desc CMPF8"]
pub type Cmpf8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CMPF1"]
    #[inline(always)]
    pub fn cmpf1(&self) -> Cmpf1R {
        Cmpf1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CMPF2"]
    #[inline(always)]
    pub fn cmpf2(&self) -> Cmpf2R {
        Cmpf2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CMPF3"]
    #[inline(always)]
    pub fn cmpf3(&self) -> Cmpf3R {
        Cmpf3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CMPF4"]
    #[inline(always)]
    pub fn cmpf4(&self) -> Cmpf4R {
        Cmpf4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CMPF5"]
    #[inline(always)]
    pub fn cmpf5(&self) -> Cmpf5R {
        Cmpf5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CMPF6"]
    #[inline(always)]
    pub fn cmpf6(&self) -> Cmpf6R {
        Cmpf6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CMPF7"]
    #[inline(always)]
    pub fn cmpf7(&self) -> Cmpf7R {
        Cmpf7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CMPF8"]
    #[inline(always)]
    pub fn cmpf8(&self) -> Cmpf8R {
        Cmpf8R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CMPF1"]
    #[inline(always)]
    pub fn cmpf1(&mut self) -> Cmpf1W<'_, StflrSpec> {
        Cmpf1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc CMPF2"]
    #[inline(always)]
    pub fn cmpf2(&mut self) -> Cmpf2W<'_, StflrSpec> {
        Cmpf2W::new(self, 1)
    }
    #[doc = "Bit 2 - desc CMPF3"]
    #[inline(always)]
    pub fn cmpf3(&mut self) -> Cmpf3W<'_, StflrSpec> {
        Cmpf3W::new(self, 2)
    }
    #[doc = "Bit 3 - desc CMPF4"]
    #[inline(always)]
    pub fn cmpf4(&mut self) -> Cmpf4W<'_, StflrSpec> {
        Cmpf4W::new(self, 3)
    }
    #[doc = "Bit 4 - desc CMPF5"]
    #[inline(always)]
    pub fn cmpf5(&mut self) -> Cmpf5W<'_, StflrSpec> {
        Cmpf5W::new(self, 4)
    }
    #[doc = "Bit 5 - desc CMPF6"]
    #[inline(always)]
    pub fn cmpf6(&mut self) -> Cmpf6W<'_, StflrSpec> {
        Cmpf6W::new(self, 5)
    }
    #[doc = "Bit 6 - desc CMPF7"]
    #[inline(always)]
    pub fn cmpf7(&mut self) -> Cmpf7W<'_, StflrSpec> {
        Cmpf7W::new(self, 6)
    }
    #[doc = "Bit 7 - desc CMPF8"]
    #[inline(always)]
    pub fn cmpf8(&mut self) -> Cmpf8W<'_, StflrSpec> {
        Cmpf8W::new(self, 7)
    }
}
#[doc = "desc STFLR\n\nYou can [`read`](crate::Reg::read) this register and get [`stflr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stflr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StflrSpec;
impl crate::RegisterSpec for StflrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`stflr::R`](R) reader structure"]
impl crate::Readable for StflrSpec {}
#[doc = "`write(|w| ..)` method takes [`stflr::W`](W) writer structure"]
impl crate::Writable for StflrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STFLR to value 0"]
impl crate::Resettable for StflrSpec {}
