#[doc = "Register `ICONR` reader"]
pub type R = crate::R<IconrSpec>;
#[doc = "Register `ICONR` writer"]
pub type W = crate::W<IconrSpec>;
#[doc = "Field `ITEN1` reader - desc ITEN1"]
pub type Iten1R = crate::BitReader;
#[doc = "Field `ITEN1` writer - desc ITEN1"]
pub type Iten1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITEN2` reader - desc ITEN2"]
pub type Iten2R = crate::BitReader;
#[doc = "Field `ITEN2` writer - desc ITEN2"]
pub type Iten2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITEN3` reader - desc ITEN3"]
pub type Iten3R = crate::BitReader;
#[doc = "Field `ITEN3` writer - desc ITEN3"]
pub type Iten3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITEN4` reader - desc ITEN4"]
pub type Iten4R = crate::BitReader;
#[doc = "Field `ITEN4` writer - desc ITEN4"]
pub type Iten4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITEN5` reader - desc ITEN5"]
pub type Iten5R = crate::BitReader;
#[doc = "Field `ITEN5` writer - desc ITEN5"]
pub type Iten5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITEN6` reader - desc ITEN6"]
pub type Iten6R = crate::BitReader;
#[doc = "Field `ITEN6` writer - desc ITEN6"]
pub type Iten6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITEN7` reader - desc ITEN7"]
pub type Iten7R = crate::BitReader;
#[doc = "Field `ITEN7` writer - desc ITEN7"]
pub type Iten7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITEN8` reader - desc ITEN8"]
pub type Iten8R = crate::BitReader;
#[doc = "Field `ITEN8` writer - desc ITEN8"]
pub type Iten8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc ITEN1"]
    #[inline(always)]
    pub fn iten1(&self) -> Iten1R {
        Iten1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ITEN2"]
    #[inline(always)]
    pub fn iten2(&self) -> Iten2R {
        Iten2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ITEN3"]
    #[inline(always)]
    pub fn iten3(&self) -> Iten3R {
        Iten3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc ITEN4"]
    #[inline(always)]
    pub fn iten4(&self) -> Iten4R {
        Iten4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc ITEN5"]
    #[inline(always)]
    pub fn iten5(&self) -> Iten5R {
        Iten5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ITEN6"]
    #[inline(always)]
    pub fn iten6(&self) -> Iten6R {
        Iten6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ITEN7"]
    #[inline(always)]
    pub fn iten7(&self) -> Iten7R {
        Iten7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc ITEN8"]
    #[inline(always)]
    pub fn iten8(&self) -> Iten8R {
        Iten8R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ITEN1"]
    #[inline(always)]
    pub fn iten1(&mut self) -> Iten1W<'_, IconrSpec> {
        Iten1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc ITEN2"]
    #[inline(always)]
    pub fn iten2(&mut self) -> Iten2W<'_, IconrSpec> {
        Iten2W::new(self, 1)
    }
    #[doc = "Bit 2 - desc ITEN3"]
    #[inline(always)]
    pub fn iten3(&mut self) -> Iten3W<'_, IconrSpec> {
        Iten3W::new(self, 2)
    }
    #[doc = "Bit 3 - desc ITEN4"]
    #[inline(always)]
    pub fn iten4(&mut self) -> Iten4W<'_, IconrSpec> {
        Iten4W::new(self, 3)
    }
    #[doc = "Bit 4 - desc ITEN5"]
    #[inline(always)]
    pub fn iten5(&mut self) -> Iten5W<'_, IconrSpec> {
        Iten5W::new(self, 4)
    }
    #[doc = "Bit 5 - desc ITEN6"]
    #[inline(always)]
    pub fn iten6(&mut self) -> Iten6W<'_, IconrSpec> {
        Iten6W::new(self, 5)
    }
    #[doc = "Bit 6 - desc ITEN7"]
    #[inline(always)]
    pub fn iten7(&mut self) -> Iten7W<'_, IconrSpec> {
        Iten7W::new(self, 6)
    }
    #[doc = "Bit 7 - desc ITEN8"]
    #[inline(always)]
    pub fn iten8(&mut self) -> Iten8W<'_, IconrSpec> {
        Iten8W::new(self, 7)
    }
}
#[doc = "desc ICONR\n\nYou can [`read`](crate::Reg::read) this register and get [`iconr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iconr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IconrSpec;
impl crate::RegisterSpec for IconrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`iconr::R`](R) reader structure"]
impl crate::Readable for IconrSpec {}
#[doc = "`write(|w| ..)` method takes [`iconr::W`](W) writer structure"]
impl crate::Writable for IconrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICONR to value 0"]
impl crate::Resettable for IconrSpec {}
