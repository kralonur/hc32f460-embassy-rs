#[doc = "Register `HSTPR` reader"]
pub type R = crate::R<HstprSpec>;
#[doc = "Register `HSTPR` writer"]
pub type W = crate::W<HstprSpec>;
#[doc = "Field `HSTP0` reader - desc HSTP0"]
pub type Hstp0R = crate::BitReader;
#[doc = "Field `HSTP0` writer - desc HSTP0"]
pub type Hstp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP1` reader - desc HSTP1"]
pub type Hstp1R = crate::BitReader;
#[doc = "Field `HSTP1` writer - desc HSTP1"]
pub type Hstp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP4` reader - desc HSTP4"]
pub type Hstp4R = crate::BitReader;
#[doc = "Field `HSTP4` writer - desc HSTP4"]
pub type Hstp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP5` reader - desc HSTP5"]
pub type Hstp5R = crate::BitReader;
#[doc = "Field `HSTP5` writer - desc HSTP5"]
pub type Hstp5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP6` reader - desc HSTP6"]
pub type Hstp6R = crate::BitReader;
#[doc = "Field `HSTP6` writer - desc HSTP6"]
pub type Hstp6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP7` reader - desc HSTP7"]
pub type Hstp7R = crate::BitReader;
#[doc = "Field `HSTP7` writer - desc HSTP7"]
pub type Hstp7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP8` reader - desc HSTP8"]
pub type Hstp8R = crate::BitReader;
#[doc = "Field `HSTP8` writer - desc HSTP8"]
pub type Hstp8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP9` reader - desc HSTP9"]
pub type Hstp9R = crate::BitReader;
#[doc = "Field `HSTP9` writer - desc HSTP9"]
pub type Hstp9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP10` reader - desc HSTP10"]
pub type Hstp10R = crate::BitReader;
#[doc = "Field `HSTP10` writer - desc HSTP10"]
pub type Hstp10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP11` reader - desc HSTP11"]
pub type Hstp11R = crate::BitReader;
#[doc = "Field `HSTP11` writer - desc HSTP11"]
pub type Hstp11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPS` reader - desc STPS"]
pub type StpsR = crate::BitReader;
#[doc = "Field `STPS` writer - desc STPS"]
pub type StpsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HSTP0"]
    #[inline(always)]
    pub fn hstp0(&self) -> Hstp0R {
        Hstp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HSTP1"]
    #[inline(always)]
    pub fn hstp1(&self) -> Hstp1R {
        Hstp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc HSTP4"]
    #[inline(always)]
    pub fn hstp4(&self) -> Hstp4R {
        Hstp4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HSTP5"]
    #[inline(always)]
    pub fn hstp5(&self) -> Hstp5R {
        Hstp5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HSTP6"]
    #[inline(always)]
    pub fn hstp6(&self) -> Hstp6R {
        Hstp6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HSTP7"]
    #[inline(always)]
    pub fn hstp7(&self) -> Hstp7R {
        Hstp7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HSTP8"]
    #[inline(always)]
    pub fn hstp8(&self) -> Hstp8R {
        Hstp8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HSTP9"]
    #[inline(always)]
    pub fn hstp9(&self) -> Hstp9R {
        Hstp9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HSTP10"]
    #[inline(always)]
    pub fn hstp10(&self) -> Hstp10R {
        Hstp10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HSTP11"]
    #[inline(always)]
    pub fn hstp11(&self) -> Hstp11R {
        Hstp11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 31 - desc STPS"]
    #[inline(always)]
    pub fn stps(&self) -> StpsR {
        StpsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HSTP0"]
    #[inline(always)]
    pub fn hstp0(&mut self) -> Hstp0W<'_, HstprSpec> {
        Hstp0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc HSTP1"]
    #[inline(always)]
    pub fn hstp1(&mut self) -> Hstp1W<'_, HstprSpec> {
        Hstp1W::new(self, 1)
    }
    #[doc = "Bit 4 - desc HSTP4"]
    #[inline(always)]
    pub fn hstp4(&mut self) -> Hstp4W<'_, HstprSpec> {
        Hstp4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc HSTP5"]
    #[inline(always)]
    pub fn hstp5(&mut self) -> Hstp5W<'_, HstprSpec> {
        Hstp5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc HSTP6"]
    #[inline(always)]
    pub fn hstp6(&mut self) -> Hstp6W<'_, HstprSpec> {
        Hstp6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc HSTP7"]
    #[inline(always)]
    pub fn hstp7(&mut self) -> Hstp7W<'_, HstprSpec> {
        Hstp7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc HSTP8"]
    #[inline(always)]
    pub fn hstp8(&mut self) -> Hstp8W<'_, HstprSpec> {
        Hstp8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc HSTP9"]
    #[inline(always)]
    pub fn hstp9(&mut self) -> Hstp9W<'_, HstprSpec> {
        Hstp9W::new(self, 9)
    }
    #[doc = "Bit 10 - desc HSTP10"]
    #[inline(always)]
    pub fn hstp10(&mut self) -> Hstp10W<'_, HstprSpec> {
        Hstp10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc HSTP11"]
    #[inline(always)]
    pub fn hstp11(&mut self) -> Hstp11W<'_, HstprSpec> {
        Hstp11W::new(self, 11)
    }
    #[doc = "Bit 31 - desc STPS"]
    #[inline(always)]
    pub fn stps(&mut self) -> StpsW<'_, HstprSpec> {
        StpsW::new(self, 31)
    }
}
#[doc = "desc HSTPR\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstprSpec;
impl crate::RegisterSpec for HstprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpr::R`](R) reader structure"]
impl crate::Readable for HstprSpec {}
#[doc = "`write(|w| ..)` method takes [`hstpr::W`](W) writer structure"]
impl crate::Writable for HstprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSTPR to value 0"]
impl crate::Resettable for HstprSpec {}
