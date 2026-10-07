#[doc = "Register `EIFR` reader"]
pub type R = crate::R<EifrSpec>;
#[doc = "Register `EIFR` writer"]
pub type W = crate::W<EifrSpec>;
#[doc = "Field `EIFR0` reader - desc EIFR0"]
pub type Eifr0R = crate::BitReader;
#[doc = "Field `EIFR0` writer - desc EIFR0"]
pub type Eifr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFR1` reader - desc EIFR1"]
pub type Eifr1R = crate::BitReader;
#[doc = "Field `EIFR1` writer - desc EIFR1"]
pub type Eifr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFR2` reader - desc EIFR2"]
pub type Eifr2R = crate::BitReader;
#[doc = "Field `EIFR2` writer - desc EIFR2"]
pub type Eifr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFR3` reader - desc EIFR3"]
pub type Eifr3R = crate::BitReader;
#[doc = "Field `EIFR3` writer - desc EIFR3"]
pub type Eifr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFR4` reader - desc EIFR4"]
pub type Eifr4R = crate::BitReader;
#[doc = "Field `EIFR4` writer - desc EIFR4"]
pub type Eifr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFR5` reader - desc EIFR5"]
pub type Eifr5R = crate::BitReader;
#[doc = "Field `EIFR5` writer - desc EIFR5"]
pub type Eifr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFR6` reader - desc EIFR6"]
pub type Eifr6R = crate::BitReader;
#[doc = "Field `EIFR6` writer - desc EIFR6"]
pub type Eifr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFR7` reader - desc EIFR7"]
pub type Eifr7R = crate::BitReader;
#[doc = "Field `EIFR7` writer - desc EIFR7"]
pub type Eifr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFR8` reader - desc EIFR8"]
pub type Eifr8R = crate::BitReader;
#[doc = "Field `EIFR8` writer - desc EIFR8"]
pub type Eifr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFR9` reader - desc EIFR9"]
pub type Eifr9R = crate::BitReader;
#[doc = "Field `EIFR9` writer - desc EIFR9"]
pub type Eifr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFR10` reader - desc EIFR10"]
pub type Eifr10R = crate::BitReader;
#[doc = "Field `EIFR10` writer - desc EIFR10"]
pub type Eifr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFR11` reader - desc EIFR11"]
pub type Eifr11R = crate::BitReader;
#[doc = "Field `EIFR11` writer - desc EIFR11"]
pub type Eifr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFR12` reader - desc EIFR12"]
pub type Eifr12R = crate::BitReader;
#[doc = "Field `EIFR12` writer - desc EIFR12"]
pub type Eifr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFR13` reader - desc EIFR13"]
pub type Eifr13R = crate::BitReader;
#[doc = "Field `EIFR13` writer - desc EIFR13"]
pub type Eifr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFR14` reader - desc EIFR14"]
pub type Eifr14R = crate::BitReader;
#[doc = "Field `EIFR14` writer - desc EIFR14"]
pub type Eifr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFR15` reader - desc EIFR15"]
pub type Eifr15R = crate::BitReader;
#[doc = "Field `EIFR15` writer - desc EIFR15"]
pub type Eifr15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EIFR0"]
    #[inline(always)]
    pub fn eifr0(&self) -> Eifr0R {
        Eifr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EIFR1"]
    #[inline(always)]
    pub fn eifr1(&self) -> Eifr1R {
        Eifr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc EIFR2"]
    #[inline(always)]
    pub fn eifr2(&self) -> Eifr2R {
        Eifr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc EIFR3"]
    #[inline(always)]
    pub fn eifr3(&self) -> Eifr3R {
        Eifr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc EIFR4"]
    #[inline(always)]
    pub fn eifr4(&self) -> Eifr4R {
        Eifr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc EIFR5"]
    #[inline(always)]
    pub fn eifr5(&self) -> Eifr5R {
        Eifr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc EIFR6"]
    #[inline(always)]
    pub fn eifr6(&self) -> Eifr6R {
        Eifr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc EIFR7"]
    #[inline(always)]
    pub fn eifr7(&self) -> Eifr7R {
        Eifr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc EIFR8"]
    #[inline(always)]
    pub fn eifr8(&self) -> Eifr8R {
        Eifr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc EIFR9"]
    #[inline(always)]
    pub fn eifr9(&self) -> Eifr9R {
        Eifr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc EIFR10"]
    #[inline(always)]
    pub fn eifr10(&self) -> Eifr10R {
        Eifr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc EIFR11"]
    #[inline(always)]
    pub fn eifr11(&self) -> Eifr11R {
        Eifr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc EIFR12"]
    #[inline(always)]
    pub fn eifr12(&self) -> Eifr12R {
        Eifr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc EIFR13"]
    #[inline(always)]
    pub fn eifr13(&self) -> Eifr13R {
        Eifr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc EIFR14"]
    #[inline(always)]
    pub fn eifr14(&self) -> Eifr14R {
        Eifr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EIFR15"]
    #[inline(always)]
    pub fn eifr15(&self) -> Eifr15R {
        Eifr15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EIFR0"]
    #[inline(always)]
    pub fn eifr0(&mut self) -> Eifr0W<'_, EifrSpec> {
        Eifr0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc EIFR1"]
    #[inline(always)]
    pub fn eifr1(&mut self) -> Eifr1W<'_, EifrSpec> {
        Eifr1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc EIFR2"]
    #[inline(always)]
    pub fn eifr2(&mut self) -> Eifr2W<'_, EifrSpec> {
        Eifr2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc EIFR3"]
    #[inline(always)]
    pub fn eifr3(&mut self) -> Eifr3W<'_, EifrSpec> {
        Eifr3W::new(self, 3)
    }
    #[doc = "Bit 4 - desc EIFR4"]
    #[inline(always)]
    pub fn eifr4(&mut self) -> Eifr4W<'_, EifrSpec> {
        Eifr4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc EIFR5"]
    #[inline(always)]
    pub fn eifr5(&mut self) -> Eifr5W<'_, EifrSpec> {
        Eifr5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc EIFR6"]
    #[inline(always)]
    pub fn eifr6(&mut self) -> Eifr6W<'_, EifrSpec> {
        Eifr6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc EIFR7"]
    #[inline(always)]
    pub fn eifr7(&mut self) -> Eifr7W<'_, EifrSpec> {
        Eifr7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc EIFR8"]
    #[inline(always)]
    pub fn eifr8(&mut self) -> Eifr8W<'_, EifrSpec> {
        Eifr8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc EIFR9"]
    #[inline(always)]
    pub fn eifr9(&mut self) -> Eifr9W<'_, EifrSpec> {
        Eifr9W::new(self, 9)
    }
    #[doc = "Bit 10 - desc EIFR10"]
    #[inline(always)]
    pub fn eifr10(&mut self) -> Eifr10W<'_, EifrSpec> {
        Eifr10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc EIFR11"]
    #[inline(always)]
    pub fn eifr11(&mut self) -> Eifr11W<'_, EifrSpec> {
        Eifr11W::new(self, 11)
    }
    #[doc = "Bit 12 - desc EIFR12"]
    #[inline(always)]
    pub fn eifr12(&mut self) -> Eifr12W<'_, EifrSpec> {
        Eifr12W::new(self, 12)
    }
    #[doc = "Bit 13 - desc EIFR13"]
    #[inline(always)]
    pub fn eifr13(&mut self) -> Eifr13W<'_, EifrSpec> {
        Eifr13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc EIFR14"]
    #[inline(always)]
    pub fn eifr14(&mut self) -> Eifr14W<'_, EifrSpec> {
        Eifr14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc EIFR15"]
    #[inline(always)]
    pub fn eifr15(&mut self) -> Eifr15W<'_, EifrSpec> {
        Eifr15W::new(self, 15)
    }
}
#[doc = "desc EIFR\n\nYou can [`read`](crate::Reg::read) this register and get [`eifr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EifrSpec;
impl crate::RegisterSpec for EifrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eifr::R`](R) reader structure"]
impl crate::Readable for EifrSpec {}
#[doc = "`write(|w| ..)` method takes [`eifr::W`](W) writer structure"]
impl crate::Writable for EifrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EIFR to value 0"]
impl crate::Resettable for EifrSpec {}
