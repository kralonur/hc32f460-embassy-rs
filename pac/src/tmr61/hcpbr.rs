#[doc = "Register `HCPBR` reader"]
pub type R = crate::R<HcpbrSpec>;
#[doc = "Register `HCPBR` writer"]
pub type W = crate::W<HcpbrSpec>;
#[doc = "Field `HCPB0` reader - desc HCPB0"]
pub type Hcpb0R = crate::BitReader;
#[doc = "Field `HCPB0` writer - desc HCPB0"]
pub type Hcpb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB1` reader - desc HCPB1"]
pub type Hcpb1R = crate::BitReader;
#[doc = "Field `HCPB1` writer - desc HCPB1"]
pub type Hcpb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB4` reader - desc HCPB4"]
pub type Hcpb4R = crate::BitReader;
#[doc = "Field `HCPB4` writer - desc HCPB4"]
pub type Hcpb4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB5` reader - desc HCPB5"]
pub type Hcpb5R = crate::BitReader;
#[doc = "Field `HCPB5` writer - desc HCPB5"]
pub type Hcpb5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB6` reader - desc HCPB6"]
pub type Hcpb6R = crate::BitReader;
#[doc = "Field `HCPB6` writer - desc HCPB6"]
pub type Hcpb6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB7` reader - desc HCPB7"]
pub type Hcpb7R = crate::BitReader;
#[doc = "Field `HCPB7` writer - desc HCPB7"]
pub type Hcpb7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB8` reader - desc HCPB8"]
pub type Hcpb8R = crate::BitReader;
#[doc = "Field `HCPB8` writer - desc HCPB8"]
pub type Hcpb8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB9` reader - desc HCPB9"]
pub type Hcpb9R = crate::BitReader;
#[doc = "Field `HCPB9` writer - desc HCPB9"]
pub type Hcpb9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB10` reader - desc HCPB10"]
pub type Hcpb10R = crate::BitReader;
#[doc = "Field `HCPB10` writer - desc HCPB10"]
pub type Hcpb10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB11` reader - desc HCPB11"]
pub type Hcpb11R = crate::BitReader;
#[doc = "Field `HCPB11` writer - desc HCPB11"]
pub type Hcpb11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HCPB0"]
    #[inline(always)]
    pub fn hcpb0(&self) -> Hcpb0R {
        Hcpb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HCPB1"]
    #[inline(always)]
    pub fn hcpb1(&self) -> Hcpb1R {
        Hcpb1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc HCPB4"]
    #[inline(always)]
    pub fn hcpb4(&self) -> Hcpb4R {
        Hcpb4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HCPB5"]
    #[inline(always)]
    pub fn hcpb5(&self) -> Hcpb5R {
        Hcpb5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HCPB6"]
    #[inline(always)]
    pub fn hcpb6(&self) -> Hcpb6R {
        Hcpb6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HCPB7"]
    #[inline(always)]
    pub fn hcpb7(&self) -> Hcpb7R {
        Hcpb7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HCPB8"]
    #[inline(always)]
    pub fn hcpb8(&self) -> Hcpb8R {
        Hcpb8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HCPB9"]
    #[inline(always)]
    pub fn hcpb9(&self) -> Hcpb9R {
        Hcpb9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HCPB10"]
    #[inline(always)]
    pub fn hcpb10(&self) -> Hcpb10R {
        Hcpb10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HCPB11"]
    #[inline(always)]
    pub fn hcpb11(&self) -> Hcpb11R {
        Hcpb11R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HCPB0"]
    #[inline(always)]
    pub fn hcpb0(&mut self) -> Hcpb0W<'_, HcpbrSpec> {
        Hcpb0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc HCPB1"]
    #[inline(always)]
    pub fn hcpb1(&mut self) -> Hcpb1W<'_, HcpbrSpec> {
        Hcpb1W::new(self, 1)
    }
    #[doc = "Bit 4 - desc HCPB4"]
    #[inline(always)]
    pub fn hcpb4(&mut self) -> Hcpb4W<'_, HcpbrSpec> {
        Hcpb4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc HCPB5"]
    #[inline(always)]
    pub fn hcpb5(&mut self) -> Hcpb5W<'_, HcpbrSpec> {
        Hcpb5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc HCPB6"]
    #[inline(always)]
    pub fn hcpb6(&mut self) -> Hcpb6W<'_, HcpbrSpec> {
        Hcpb6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc HCPB7"]
    #[inline(always)]
    pub fn hcpb7(&mut self) -> Hcpb7W<'_, HcpbrSpec> {
        Hcpb7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc HCPB8"]
    #[inline(always)]
    pub fn hcpb8(&mut self) -> Hcpb8W<'_, HcpbrSpec> {
        Hcpb8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc HCPB9"]
    #[inline(always)]
    pub fn hcpb9(&mut self) -> Hcpb9W<'_, HcpbrSpec> {
        Hcpb9W::new(self, 9)
    }
    #[doc = "Bit 10 - desc HCPB10"]
    #[inline(always)]
    pub fn hcpb10(&mut self) -> Hcpb10W<'_, HcpbrSpec> {
        Hcpb10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc HCPB11"]
    #[inline(always)]
    pub fn hcpb11(&mut self) -> Hcpb11W<'_, HcpbrSpec> {
        Hcpb11W::new(self, 11)
    }
}
#[doc = "desc HCPBR\n\nYou can [`read`](crate::Reg::read) this register and get [`hcpbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcpbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcpbrSpec;
impl crate::RegisterSpec for HcpbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcpbr::R`](R) reader structure"]
impl crate::Readable for HcpbrSpec {}
#[doc = "`write(|w| ..)` method takes [`hcpbr::W`](W) writer structure"]
impl crate::Writable for HcpbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCPBR to value 0"]
impl crate::Resettable for HcpbrSpec {}
