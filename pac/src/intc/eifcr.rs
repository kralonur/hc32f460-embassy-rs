#[doc = "Register `EIFCR` reader"]
pub type R = crate::R<EifcrSpec>;
#[doc = "Register `EIFCR` writer"]
pub type W = crate::W<EifcrSpec>;
#[doc = "Field `EIFCR0` reader - desc EIFCR0"]
pub type Eifcr0R = crate::BitReader;
#[doc = "Field `EIFCR0` writer - desc EIFCR0"]
pub type Eifcr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCR1` reader - desc EIFCR1"]
pub type Eifcr1R = crate::BitReader;
#[doc = "Field `EIFCR1` writer - desc EIFCR1"]
pub type Eifcr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCR2` reader - desc EIFCR2"]
pub type Eifcr2R = crate::BitReader;
#[doc = "Field `EIFCR2` writer - desc EIFCR2"]
pub type Eifcr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCR3` reader - desc EIFCR3"]
pub type Eifcr3R = crate::BitReader;
#[doc = "Field `EIFCR3` writer - desc EIFCR3"]
pub type Eifcr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCR4` reader - desc EIFCR4"]
pub type Eifcr4R = crate::BitReader;
#[doc = "Field `EIFCR4` writer - desc EIFCR4"]
pub type Eifcr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCR5` reader - desc EIFCR5"]
pub type Eifcr5R = crate::BitReader;
#[doc = "Field `EIFCR5` writer - desc EIFCR5"]
pub type Eifcr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCR6` reader - desc EIFCR6"]
pub type Eifcr6R = crate::BitReader;
#[doc = "Field `EIFCR6` writer - desc EIFCR6"]
pub type Eifcr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCR7` reader - desc EIFCR7"]
pub type Eifcr7R = crate::BitReader;
#[doc = "Field `EIFCR7` writer - desc EIFCR7"]
pub type Eifcr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCR8` reader - desc EIFCR8"]
pub type Eifcr8R = crate::BitReader;
#[doc = "Field `EIFCR8` writer - desc EIFCR8"]
pub type Eifcr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCR9` reader - desc EIFCR9"]
pub type Eifcr9R = crate::BitReader;
#[doc = "Field `EIFCR9` writer - desc EIFCR9"]
pub type Eifcr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCR10` reader - desc EIFCR10"]
pub type Eifcr10R = crate::BitReader;
#[doc = "Field `EIFCR10` writer - desc EIFCR10"]
pub type Eifcr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCR11` reader - desc EIFCR11"]
pub type Eifcr11R = crate::BitReader;
#[doc = "Field `EIFCR11` writer - desc EIFCR11"]
pub type Eifcr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCR12` reader - desc EIFCR12"]
pub type Eifcr12R = crate::BitReader;
#[doc = "Field `EIFCR12` writer - desc EIFCR12"]
pub type Eifcr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCR13` reader - desc EIFCR13"]
pub type Eifcr13R = crate::BitReader;
#[doc = "Field `EIFCR13` writer - desc EIFCR13"]
pub type Eifcr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCR14` reader - desc EIFCR14"]
pub type Eifcr14R = crate::BitReader;
#[doc = "Field `EIFCR14` writer - desc EIFCR14"]
pub type Eifcr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCR15` reader - desc EIFCR15"]
pub type Eifcr15R = crate::BitReader;
#[doc = "Field `EIFCR15` writer - desc EIFCR15"]
pub type Eifcr15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EIFCR0"]
    #[inline(always)]
    pub fn eifcr0(&self) -> Eifcr0R {
        Eifcr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EIFCR1"]
    #[inline(always)]
    pub fn eifcr1(&self) -> Eifcr1R {
        Eifcr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc EIFCR2"]
    #[inline(always)]
    pub fn eifcr2(&self) -> Eifcr2R {
        Eifcr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc EIFCR3"]
    #[inline(always)]
    pub fn eifcr3(&self) -> Eifcr3R {
        Eifcr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc EIFCR4"]
    #[inline(always)]
    pub fn eifcr4(&self) -> Eifcr4R {
        Eifcr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc EIFCR5"]
    #[inline(always)]
    pub fn eifcr5(&self) -> Eifcr5R {
        Eifcr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc EIFCR6"]
    #[inline(always)]
    pub fn eifcr6(&self) -> Eifcr6R {
        Eifcr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc EIFCR7"]
    #[inline(always)]
    pub fn eifcr7(&self) -> Eifcr7R {
        Eifcr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc EIFCR8"]
    #[inline(always)]
    pub fn eifcr8(&self) -> Eifcr8R {
        Eifcr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc EIFCR9"]
    #[inline(always)]
    pub fn eifcr9(&self) -> Eifcr9R {
        Eifcr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc EIFCR10"]
    #[inline(always)]
    pub fn eifcr10(&self) -> Eifcr10R {
        Eifcr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc EIFCR11"]
    #[inline(always)]
    pub fn eifcr11(&self) -> Eifcr11R {
        Eifcr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc EIFCR12"]
    #[inline(always)]
    pub fn eifcr12(&self) -> Eifcr12R {
        Eifcr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc EIFCR13"]
    #[inline(always)]
    pub fn eifcr13(&self) -> Eifcr13R {
        Eifcr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc EIFCR14"]
    #[inline(always)]
    pub fn eifcr14(&self) -> Eifcr14R {
        Eifcr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EIFCR15"]
    #[inline(always)]
    pub fn eifcr15(&self) -> Eifcr15R {
        Eifcr15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EIFCR0"]
    #[inline(always)]
    pub fn eifcr0(&mut self) -> Eifcr0W<'_, EifcrSpec> {
        Eifcr0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc EIFCR1"]
    #[inline(always)]
    pub fn eifcr1(&mut self) -> Eifcr1W<'_, EifcrSpec> {
        Eifcr1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc EIFCR2"]
    #[inline(always)]
    pub fn eifcr2(&mut self) -> Eifcr2W<'_, EifcrSpec> {
        Eifcr2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc EIFCR3"]
    #[inline(always)]
    pub fn eifcr3(&mut self) -> Eifcr3W<'_, EifcrSpec> {
        Eifcr3W::new(self, 3)
    }
    #[doc = "Bit 4 - desc EIFCR4"]
    #[inline(always)]
    pub fn eifcr4(&mut self) -> Eifcr4W<'_, EifcrSpec> {
        Eifcr4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc EIFCR5"]
    #[inline(always)]
    pub fn eifcr5(&mut self) -> Eifcr5W<'_, EifcrSpec> {
        Eifcr5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc EIFCR6"]
    #[inline(always)]
    pub fn eifcr6(&mut self) -> Eifcr6W<'_, EifcrSpec> {
        Eifcr6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc EIFCR7"]
    #[inline(always)]
    pub fn eifcr7(&mut self) -> Eifcr7W<'_, EifcrSpec> {
        Eifcr7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc EIFCR8"]
    #[inline(always)]
    pub fn eifcr8(&mut self) -> Eifcr8W<'_, EifcrSpec> {
        Eifcr8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc EIFCR9"]
    #[inline(always)]
    pub fn eifcr9(&mut self) -> Eifcr9W<'_, EifcrSpec> {
        Eifcr9W::new(self, 9)
    }
    #[doc = "Bit 10 - desc EIFCR10"]
    #[inline(always)]
    pub fn eifcr10(&mut self) -> Eifcr10W<'_, EifcrSpec> {
        Eifcr10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc EIFCR11"]
    #[inline(always)]
    pub fn eifcr11(&mut self) -> Eifcr11W<'_, EifcrSpec> {
        Eifcr11W::new(self, 11)
    }
    #[doc = "Bit 12 - desc EIFCR12"]
    #[inline(always)]
    pub fn eifcr12(&mut self) -> Eifcr12W<'_, EifcrSpec> {
        Eifcr12W::new(self, 12)
    }
    #[doc = "Bit 13 - desc EIFCR13"]
    #[inline(always)]
    pub fn eifcr13(&mut self) -> Eifcr13W<'_, EifcrSpec> {
        Eifcr13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc EIFCR14"]
    #[inline(always)]
    pub fn eifcr14(&mut self) -> Eifcr14W<'_, EifcrSpec> {
        Eifcr14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc EIFCR15"]
    #[inline(always)]
    pub fn eifcr15(&mut self) -> Eifcr15W<'_, EifcrSpec> {
        Eifcr15W::new(self, 15)
    }
}
#[doc = "desc EIFCR\n\nYou can [`read`](crate::Reg::read) this register and get [`eifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EifcrSpec;
impl crate::RegisterSpec for EifcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eifcr::R`](R) reader structure"]
impl crate::Readable for EifcrSpec {}
#[doc = "`write(|w| ..)` method takes [`eifcr::W`](W) writer structure"]
impl crate::Writable for EifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EIFCR to value 0"]
impl crate::Resettable for EifcrSpec {}
