#[doc = "Register `HCPAR` reader"]
pub type R = crate::R<HcparSpec>;
#[doc = "Register `HCPAR` writer"]
pub type W = crate::W<HcparSpec>;
#[doc = "Field `HCPA0` reader - desc HCPA0"]
pub type Hcpa0R = crate::BitReader;
#[doc = "Field `HCPA0` writer - desc HCPA0"]
pub type Hcpa0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA1` reader - desc HCPA1"]
pub type Hcpa1R = crate::BitReader;
#[doc = "Field `HCPA1` writer - desc HCPA1"]
pub type Hcpa1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA4` reader - desc HCPA4"]
pub type Hcpa4R = crate::BitReader;
#[doc = "Field `HCPA4` writer - desc HCPA4"]
pub type Hcpa4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA5` reader - desc HCPA5"]
pub type Hcpa5R = crate::BitReader;
#[doc = "Field `HCPA5` writer - desc HCPA5"]
pub type Hcpa5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA6` reader - desc HCPA6"]
pub type Hcpa6R = crate::BitReader;
#[doc = "Field `HCPA6` writer - desc HCPA6"]
pub type Hcpa6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA7` reader - desc HCPA7"]
pub type Hcpa7R = crate::BitReader;
#[doc = "Field `HCPA7` writer - desc HCPA7"]
pub type Hcpa7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA8` reader - desc HCPA8"]
pub type Hcpa8R = crate::BitReader;
#[doc = "Field `HCPA8` writer - desc HCPA8"]
pub type Hcpa8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA9` reader - desc HCPA9"]
pub type Hcpa9R = crate::BitReader;
#[doc = "Field `HCPA9` writer - desc HCPA9"]
pub type Hcpa9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA10` reader - desc HCPA10"]
pub type Hcpa10R = crate::BitReader;
#[doc = "Field `HCPA10` writer - desc HCPA10"]
pub type Hcpa10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA11` reader - desc HCPA11"]
pub type Hcpa11R = crate::BitReader;
#[doc = "Field `HCPA11` writer - desc HCPA11"]
pub type Hcpa11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HCPA0"]
    #[inline(always)]
    pub fn hcpa0(&self) -> Hcpa0R {
        Hcpa0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HCPA1"]
    #[inline(always)]
    pub fn hcpa1(&self) -> Hcpa1R {
        Hcpa1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc HCPA4"]
    #[inline(always)]
    pub fn hcpa4(&self) -> Hcpa4R {
        Hcpa4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HCPA5"]
    #[inline(always)]
    pub fn hcpa5(&self) -> Hcpa5R {
        Hcpa5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HCPA6"]
    #[inline(always)]
    pub fn hcpa6(&self) -> Hcpa6R {
        Hcpa6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HCPA7"]
    #[inline(always)]
    pub fn hcpa7(&self) -> Hcpa7R {
        Hcpa7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HCPA8"]
    #[inline(always)]
    pub fn hcpa8(&self) -> Hcpa8R {
        Hcpa8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HCPA9"]
    #[inline(always)]
    pub fn hcpa9(&self) -> Hcpa9R {
        Hcpa9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HCPA10"]
    #[inline(always)]
    pub fn hcpa10(&self) -> Hcpa10R {
        Hcpa10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HCPA11"]
    #[inline(always)]
    pub fn hcpa11(&self) -> Hcpa11R {
        Hcpa11R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HCPA0"]
    #[inline(always)]
    pub fn hcpa0(&mut self) -> Hcpa0W<'_, HcparSpec> {
        Hcpa0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc HCPA1"]
    #[inline(always)]
    pub fn hcpa1(&mut self) -> Hcpa1W<'_, HcparSpec> {
        Hcpa1W::new(self, 1)
    }
    #[doc = "Bit 4 - desc HCPA4"]
    #[inline(always)]
    pub fn hcpa4(&mut self) -> Hcpa4W<'_, HcparSpec> {
        Hcpa4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc HCPA5"]
    #[inline(always)]
    pub fn hcpa5(&mut self) -> Hcpa5W<'_, HcparSpec> {
        Hcpa5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc HCPA6"]
    #[inline(always)]
    pub fn hcpa6(&mut self) -> Hcpa6W<'_, HcparSpec> {
        Hcpa6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc HCPA7"]
    #[inline(always)]
    pub fn hcpa7(&mut self) -> Hcpa7W<'_, HcparSpec> {
        Hcpa7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc HCPA8"]
    #[inline(always)]
    pub fn hcpa8(&mut self) -> Hcpa8W<'_, HcparSpec> {
        Hcpa8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc HCPA9"]
    #[inline(always)]
    pub fn hcpa9(&mut self) -> Hcpa9W<'_, HcparSpec> {
        Hcpa9W::new(self, 9)
    }
    #[doc = "Bit 10 - desc HCPA10"]
    #[inline(always)]
    pub fn hcpa10(&mut self) -> Hcpa10W<'_, HcparSpec> {
        Hcpa10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc HCPA11"]
    #[inline(always)]
    pub fn hcpa11(&mut self) -> Hcpa11W<'_, HcparSpec> {
        Hcpa11W::new(self, 11)
    }
}
#[doc = "desc HCPAR\n\nYou can [`read`](crate::Reg::read) this register and get [`hcpar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcpar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcparSpec;
impl crate::RegisterSpec for HcparSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcpar::R`](R) reader structure"]
impl crate::Readable for HcparSpec {}
#[doc = "`write(|w| ..)` method takes [`hcpar::W`](W) writer structure"]
impl crate::Writable for HcparSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCPAR to value 0"]
impl crate::Resettable for HcparSpec {}
